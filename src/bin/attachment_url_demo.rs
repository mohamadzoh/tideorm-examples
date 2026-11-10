//! File Attachment URL Generation Demo
//!
//! This example demonstrates the file attachment URL generation feature in TideORM.
//! It shows how to:
//! - Configure a global file base URL
//! - Use a custom URL generator function with field name and full file metadata
//! - Override URL generation per-model
//! - Access generated URLs in JSON output
//! - Generate different URLs based on field name and file properties
//!
//! Run with: cargo run --bin attachment_url_demo

use tideorm::prelude::*;

// ============================================================================
// HELPER MACROS FOR TESTING
// ============================================================================

macro_rules! test_section {
    ($name:expr) => {
        println!("\n{}", "=".repeat(70));
        println!("📋 {}", $name);
        println!("{}", "=".repeat(70));
    };
}

macro_rules! test_case {
    ($name:expr) => {
        println!("\n  🧪 {}", $name);
        println!("  {}", "-".repeat(60));
    };
}

macro_rules! verify {
    ($condition:expr, $msg:expr) => {
        if $condition {
            println!("    ✅ {}", $msg);
        } else {
            println!("    ❌ FAILED: {}", $msg);
            panic!("Verification failed: {}", $msg);
        }
    };
}

macro_rules! verify_eq {
    ($actual:expr, $expected:expr, $msg:expr) => {
        if $actual == $expected {
            println!("    ✅ {} (value: {:?})", $msg, $actual);
        } else {
            println!("    ❌ FAILED: {}", $msg);
            println!("       Expected: {:?}", $expected);
            println!("       Actual:   {:?}", $actual);
            panic!("Verification failed: {}", $msg);
        }
    };
}

// ============================================================================
// A MODEL WITH ATTACHMENTS, AND HELPERS
// ============================================================================

/// The model whose `to_json()` renders attachments with their URLs.
#[tideorm::model(
    table = "attachment_demo_products",
    has_one_files = "thumbnail,avatar,gallery,document"
)]
pub struct Product {
    #[tideorm(primary_key)]
    pub id: i64,
    pub files: Option<serde_json::Value>,
}

/// Install `generator` as TideORM's attachment URL generator. `apply()` installs
/// the configuration without opening a database connection.
fn use_generator(generator: fn(&str, &FileAttachment) -> String) {
    TideConfig::init().file_url_generator(generator).apply();
}

/// What `Product::to_json` renders for `file` attached as `field`, under the
/// configuration applied last.
fn rendered(field: &str, file: &FileAttachment) -> serde_json::Value {
    let mut files = FilesData::new();
    files.set_one(field, file.clone());
    let product = Product {
        id: 1,
        files: Some(files.to_json()),
    };
    product.to_json(None)[field].clone()
}

/// `key` as a URL path: every byte but unreserved characters and `/` is
/// percent-encoded, so a `?`, `#` or space in a key cannot end the path.
/// Keys from outside the application should also pass
/// `FileAttachment::is_safe_key` before they are stored.
fn url_path(key: &str) -> String {
    let mut path = String::new();
    for byte in key.trim_start_matches('/').bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                path.push(char::from(byte))
            }
            _ => path.push_str(&format!("%{byte:02X}")),
        }
    }
    path
}

// ============================================================================
// CUSTOM URL GENERATORS (Now with field_name AND full file metadata access)
// ============================================================================

/// Simple CDN URL generator (ignores field_name)
fn cdn_url_generator(_field_name: &str, file: &FileAttachment) -> String {
    format!(
        "https://cdn.example.com/{}",
        url_path(&file.key)
    )
}

/// S3 URL generator with bucket
#[allow(dead_code)]
fn s3_url_generator(_field_name: &str, file: &FileAttachment) -> String {
    format!(
        "https://my-bucket.s3.amazonaws.com/{}",
        url_path(&file.key)
    )
}

/// Field-aware URL generator - routes based on field name
fn field_aware_url_generator(field_name: &str, file: &FileAttachment) -> String {
    let key = url_path(&file.key);

    match field_name {
        "thumbnail" | "avatar" | "profile_image" => {
            // Images go to image-specific CDN with transformations
            format!("https://images.example.com/w_auto,f_auto/{}", key)
        }
        "video" | "preview_video" => {
            // Videos go to streaming CDN
            format!("https://stream.example.com/{}", key)
        }
        "document" | "attachment" | "resume" => {
            // Documents go to secure document storage
            format!("https://docs.example.com/secure/{}", key)
        }
        "cover_image" | "banner" => {
            // Large images optimized for display
            format!("https://images.example.com/w_1200,q_80/{}", key)
        }
        _ => {
            // Default: general CDN
            format!("https://cdn.example.com/{}", key)
        }
    }
}

/// Combined field + metadata URL generator
fn smart_url_generator(field_name: &str, file: &FileAttachment) -> String {
    let key = url_path(&file.key);

    // First, check field-specific routing
    match field_name {
        "thumbnail" => {
            // Thumbnails: Apply quality based on size
            let quality = if file.size.unwrap_or(0) > 500_000 {
                "60"
            } else {
                "auto"
            };
            return format!("https://thumbs.example.com/q_{}/{}", quality, key);
        }
        "avatar" => {
            // Avatars: Always compressed, square format
            return format!("https://avatars.example.com/w_200,h_200,c_fill/{}", key);
        }
        _ => {}
    }

    // Then, check mime_type for type-specific routing
    match file.mime_type.as_deref() {
        Some(m) if m.starts_with("video/") => {
            format!("https://stream.example.com/{}", key)
        }
        Some(m) if m.starts_with("image/") => {
            let quality = if file.size.unwrap_or(0) > 1_000_000 {
                "80"
            } else {
                "auto"
            };
            format!("https://images.example.com/q_{}/{}", quality, key)
        }
        Some("application/pdf") => {
            format!("https://docs.example.com/{}", key)
        }
        _ => {
            format!("https://cdn.example.com/{}", key)
        }
    }
}

/// Product model URL generator
fn product_url_generator(field_name: &str, file: &FileAttachment) -> String {
    let key = url_path(&file.key);

    match field_name {
        "thumbnail" => format!("https://products-cdn.example.com/thumb/{}", key),
        "gallery" => format!("https://products-cdn.example.com/gallery/{}", key),
        "manual" => format!("https://products-cdn.example.com/docs/{}", key),
        _ => format!("https://products-cdn.example.com/assets/{}", key),
    }
}

/// User model URL generator
fn user_url_generator(field_name: &str, file: &FileAttachment) -> String {
    let key = url_path(&file.key);

    match field_name {
        "avatar" => format!("https://avatars.example.com/{}", key),
        "cover_photo" => format!("https://covers.example.com/{}", key),
        "resume" => format!("https://secure-docs.example.com/resumes/{}", key),
        _ => format!("https://user-files.example.com/{}", key),
    }
}

/// Access-tagged URL generator: a mock, NOT a signed URL.
///
/// Its token is computed from public values with no secret, so anyone can forge
/// it. Real signed URLs come from the storage service (an S3 presigned URL) or an
/// HMAC over the key and expiry with a server-side secret, which a generator
/// installed with `TideConfig::file_url_generator` can capture.
fn signed_url_generator(field_name: &str, file: &FileAttachment) -> String {
    let key = url_path(&file.key);

    // Generate token based on field and file
    let access_level = match field_name {
        "private_document" | "secure_file" => "restricted",
        "internal_only" => "internal",
        _ => "public",
    };

    let size_hash = file.size.unwrap_or(0) % 1000;
    let token = format!("{}_{:03}", access_level, size_hash);

    format!(
        "https://secure-cdn.example.com/{}?token={}&expires=3600",
        key, token
    )
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  TideORM File Attachment URL Generation Demo (Field Name + Metadata) ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    let mut passed_tests = 0;
    let mut total_tests = 0;

    // ========================================================================
    // TEST 1: Default URL Generation (No base URL)
    // ========================================================================
    test_section!("TEST 1: Default URL Generation (No Base URL Configured)");

    test_case!("1.1: URL without base URL returns key as-is");
    {
        TideConfig::init().apply();
        let file = FileAttachment::new("uploads/2024/image.jpg");
        let url = file.url("thumbnail");
        verify_eq!(
            url,
            "uploads/2024/image.jpg",
            "URL equals key when no base URL"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("1.2: File with metadata - URL still works");
    {
        TideConfig::init().apply();
        let file = FileAttachment::with_metadata(
            "uploads/doc.pdf",
            Some("Report.pdf"),
            Some(1024),
            Some("application/pdf"),
        );
        let url = file.url("document");
        verify_eq!(url, "uploads/doc.pdf", "URL from file with metadata");
        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 2: Base URL Configuration
    // ========================================================================
    test_section!("TEST 2: Base URL Configuration");

    test_case!("2.1: Simple base URL concatenation");
    {
        TideConfig::init().file_base_url("https://cdn.example.com/uploads").apply();
        let file = FileAttachment::new("2024/image.jpg");
        let url = file.url("thumbnail");
        verify_eq!(
            url,
            "https://cdn.example.com/uploads/2024/image.jpg",
            "Base URL + key"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("2.2: Trailing slash in base URL handled");
    {
        TideConfig::init().file_base_url("https://cdn.example.com/uploads/").apply();
        let file = FileAttachment::new("2024/image.jpg");
        let url = file.url("avatar");
        verify_eq!(
            url,
            "https://cdn.example.com/uploads/2024/image.jpg",
            "Trailing slash trimmed"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("2.3: Leading slash in key handled");
    {
        TideConfig::init().file_base_url("https://cdn.example.com/uploads").apply();
        let file = FileAttachment::new("/2024/image.jpg");
        let url = file.url("document");
        verify_eq!(
            url,
            "https://cdn.example.com/uploads/2024/image.jpg",
            "Leading slash trimmed"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 3: Field-Aware URL Generation
    // ========================================================================
    test_section!("TEST 3: Field-Aware URL Generation");

    test_case!("3.1: Different URLs for different field names");
    {
        use_generator(field_aware_url_generator);
        let file = FileAttachment::new("products/123/image.jpg");

        let thumb_url = file.url("thumbnail");
        let video_url = file.url("video");
        let doc_url = file.url("document");
        let other_url = file.url("other");

        verify!(
            thumb_url.contains("images.example.com"),
            "Thumbnail goes to images CDN"
        );
        verify!(
            video_url.contains("stream.example.com"),
            "Video goes to streaming CDN"
        );
        verify!(
            doc_url.contains("docs.example.com/secure"),
            "Document goes to secure docs"
        );
        verify!(
            other_url.contains("cdn.example.com"),
            "Other goes to general CDN"
        );

        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("3.2: Avatar field gets special formatting");
    {
        use_generator(field_aware_url_generator);
        let file = FileAttachment::new("users/avatar.jpg");
        let url = file.url("avatar");
        verify!(
            url.contains("images.example.com/w_auto,f_auto"),
            "Avatar has transformations"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("3.3: Cover image gets optimized resolution");
    {
        use_generator(field_aware_url_generator);
        let file = FileAttachment::new("banners/hero.jpg");
        let url = file.url("cover_image");
        verify!(
            url.contains("w_1200,q_80"),
            "Cover image optimized for large display"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 4: Combined Field + Metadata URL Generation
    // ========================================================================
    test_section!("TEST 4: Combined Field + Metadata URL Generation");

    test_case!("4.1: Thumbnail field takes priority over mime_type");
    {
        use_generator(smart_url_generator);
        let file =
            FileAttachment::with_metadata("img.jpg", None, Some(100_000), Some("image/jpeg"));
        let url = file.url("thumbnail");
        verify!(
            url.contains("thumbs.example.com"),
            "Thumbnail field routes to thumbs CDN"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("4.2: Avatar field applies specific transformations");
    {
        use_generator(smart_url_generator);
        let file =
            FileAttachment::with_metadata("avatar.png", None, Some(50_000), Some("image/png"));
        let url = file.url("avatar");
        verify!(
            url.contains("w_200,h_200,c_fill"),
            "Avatar has square crop applied"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("4.3: Non-special fields fall back to mime_type routing");
    {
        use_generator(smart_url_generator);
        let video = FileAttachment::with_metadata("media/clip.mp4", None, None, Some("video/mp4"));
        let image = FileAttachment::with_metadata(
            "media/photo.jpg",
            None,
            Some(2_000_000),
            Some("image/jpeg"),
        );
        let pdf =
            FileAttachment::with_metadata("media/doc.pdf", None, None, Some("application/pdf"));

        let video_url = video.url("gallery");
        let image_url = image.url("gallery");
        let pdf_url = pdf.url("gallery");

        verify!(
            video_url.contains("stream.example.com"),
            "Video to streaming"
        );
        verify!(
            image_url.contains("images.example.com/q_80"),
            "Large image gets reduced quality"
        );
        verify!(pdf_url.contains("docs.example.com"), "PDF to docs");

        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 5: Model-Specific URL Generators
    // ========================================================================
    test_section!("TEST 5: Model-Specific URL Generators");

    test_case!("5.1: Product model routes by field");
    {
        use_generator(product_url_generator);
        let file = FileAttachment::new("products/123/image.jpg");

        let thumb = file.url("thumbnail");
        let gallery = file.url("gallery");
        let manual = file.url("manual");
        let other = file.url("specs");

        verify!(thumb.contains("/thumb/"), "Thumbnail to thumb path");
        verify!(gallery.contains("/gallery/"), "Gallery to gallery path");
        verify!(manual.contains("/docs/"), "Manual to docs path");
        verify!(other.contains("/assets/"), "Other to assets path");

        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("5.2: User model routes by field");
    {
        use_generator(user_url_generator);
        let file = FileAttachment::new("users/456/file.jpg");

        let avatar = file.url("avatar");
        let cover = file.url("cover_photo");
        let resume = file.url("resume");

        verify!(
            avatar.contains("avatars.example.com"),
            "Avatar to avatars CDN"
        );
        verify!(cover.contains("covers.example.com"), "Cover to covers CDN");
        verify!(
            resume.contains("secure-docs.example.com/resumes"),
            "Resume to secure docs"
        );

        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 6: Access-Tagged URLs (a mock, not signing)
    // ========================================================================
    test_section!("TEST 6: Access-Tagged URLs (mock tokens, not cryptographic signatures)");

    test_case!("6.1: Restricted access for private documents");
    {
        use_generator(signed_url_generator);
        let file = FileAttachment::with_metadata("docs/secret.pdf", None, Some(5000), None);
        let url = file.url("private_document");
        verify!(
            url.contains("token=restricted_"),
            "Private document gets restricted token"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("6.2: Internal access for internal-only files");
    {
        use_generator(signed_url_generator);
        let file = FileAttachment::with_metadata("internal/report.xlsx", None, Some(3000), None);
        let url = file.url("internal_only");
        verify!(
            url.contains("token=internal_"),
            "Internal file gets internal token"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("6.3: Public access for regular files");
    {
        use_generator(signed_url_generator);
        let file = FileAttachment::with_metadata("public/image.jpg", None, Some(1234), None);
        let url = file.url("thumbnail");
        verify!(
            url.contains("token=public_234"),
            "Public file gets public token with size hash"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 7: JSON Output with Field-Aware URLs
    // ========================================================================
    test_section!("TEST 7: JSON Output with Field-Aware URLs");

    test_case!("7.1: Single file attachment with field-specific URL");
    {
        use_generator(smart_url_generator);
        let attachment = FileAttachment::with_metadata(
            "uploads/thumbnail.jpg",
            Some("Photo.jpg"),
            Some(50_000),
            Some("image/jpeg"),
        );

        let processed = rendered("thumbnail", &attachment);

        verify!(processed.get("url").is_some(), "URL field added");
        let url = processed["url"].as_str().unwrap();
        verify!(
            url.contains("thumbs.example.com"),
            "Thumbnail field routed correctly"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("7.2: Different fields get different URLs for same file");
    {
        use_generator(smart_url_generator);
        let attachment = FileAttachment::with_metadata(
            "uploads/image.jpg",
            None,
            Some(100_000),
            Some("image/jpeg"),
        );

        let as_thumbnail = rendered("thumbnail", &attachment);
        let as_avatar = rendered("avatar", &attachment);
        let as_gallery = rendered("gallery", &attachment);

        let thumb_url = as_thumbnail["url"].as_str().unwrap();
        let avatar_url = as_avatar["url"].as_str().unwrap();
        let gallery_url = as_gallery["url"].as_str().unwrap();

        verify!(
            thumb_url.contains("thumbs.example.com"),
            "Thumbnail to thumbs"
        );
        verify!(
            avatar_url.contains("avatars.example.com"),
            "Avatar to avatars"
        );
        verify!(
            gallery_url.contains("images.example.com"),
            "Gallery to images"
        );

        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("7.3: Hidden attributes excluded, URL still added");
    {
        TideConfig::init()
            .hidden_attributes(["size", "mime_type"])
            .file_url_generator(cdn_url_generator)
            .apply();
        let attachment = FileAttachment::with_metadata(
            "docs/secret.pdf",
            Some("Secret Document.pdf"),
            Some(1024 * 1024),
            Some("application/pdf"),
        );

        let processed = rendered("document", &attachment);

        verify!(processed.get("url").is_some(), "URL present");
        verify!(processed.get("key").is_some(), "Key present");
        verify!(processed.get("size").is_none(), "Size hidden");
        verify!(processed.get("mime_type").is_none(), "Mime type hidden");

        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 8: Edge Cases
    // ========================================================================
    test_section!("TEST 8: Edge Cases");

    test_case!("8.1: Empty field name");
    {
        use_generator(field_aware_url_generator);
        let file = FileAttachment::new("file.jpg");
        let url = file.url("");
        verify!(
            url.contains("cdn.example.com"),
            "Empty field name falls back to default"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("8.2: Field name with special characters");
    {
        use_generator(field_aware_url_generator);
        let file = FileAttachment::new("file.jpg");
        let url = file.url("user_avatar_2");
        // Should fall through to default
        verify!(
            url.contains("cdn.example.com"),
            "Special field name handled"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("8.3: Unicode field name, and key characters encoded in the URL");
    {
        use_generator(cdn_url_generator);
        let file = FileAttachment::new("файл.jpg");
        let url = file.url("изображение");
        verify_eq!(
            url,
            "https://cdn.example.com/%D1%84%D0%B0%D0%B9%D0%BB.jpg",
            "Unicode key percent-encoded"
        );

        let file = FileAttachment::new("reports/q1 summary?.pdf#v2");
        let url = file.url("document");
        verify_eq!(
            url,
            "https://cdn.example.com/reports/q1%20summary%3F.pdf%23v2",
            "`?`, `#` and spaces cannot end the path"
        );
        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // TEST 9: Timing (informational)
    // ========================================================================
    test_section!("TEST 9: Timing (informational, not a benchmark)");

    test_case!("9.1: URL generation with field routing");
    {
        use_generator(field_aware_url_generator);
        let file = FileAttachment::new("test.jpg");
        let fields = ["thumbnail", "avatar", "document", "video", "gallery"];

        let start = std::time::Instant::now();
        let mut generated = 0;
        for _ in 0..10000 {
            for field in &fields {
                generated += std::hint::black_box(file.url(field)).len();
            }
        }
        let duration = start.elapsed();

        println!("    ⏱️  50,000 field-aware URL generations: {:?}", duration);
        verify!(generated > 0, "Every call generated a URL");

        total_tests += 1;
        passed_tests += 1;
    }

    test_case!("9.2: Generator with field + metadata access");
    {
        use_generator(smart_url_generator);
        let files: Vec<_> = (0..1000)
            .map(|i| {
                FileAttachment::with_metadata(
                    &format!("files/{}.jpg", i),
                    Some("original.jpg"),
                    Some(i as u64 * 1000),
                    Some("image/jpeg"),
                )
            })
            .collect();

        let fields = ["thumbnail", "avatar", "gallery"];

        let start = std::time::Instant::now();
        let mut generated = 0;
        for file in &files {
            for field in &fields {
                generated += std::hint::black_box(file.url(field)).len();
            }
        }
        let duration = start.elapsed();

        println!("    ⏱️  3,000 smart URL generations: {:?}", duration);
        verify!(generated > 0, "Every call generated a URL");

        total_tests += 1;
        passed_tests += 1;
    }

    // ========================================================================
    // SUMMARY
    // ========================================================================
    println!("\n{}", "═".repeat(70));
    println!("📊 TEST SUMMARY");
    println!("{}", "═".repeat(70));
    println!("  Total Tests: {}", total_tests);
    println!("  Passed:      {}", passed_tests);
    println!("  Failed:      {}", total_tests - passed_tests);
    println!("{}", "═".repeat(70));

    if passed_tests == total_tests {
        println!("\n✅ ALL FILE ATTACHMENT URL TESTS PASSED!\n");
    } else {
        println!("\n❌ SOME TESTS FAILED!\n");
        std::process::exit(1);
    }

    // ========================================================================
    // USAGE EXAMPLES OUTPUT
    // ========================================================================
    println!("\n{}", "═".repeat(70));
    println!("📚 USAGE EXAMPLES (with field_name + FileAttachment access)");
    println!("{}", "═".repeat(70));

    println!("\n1️⃣  Configure global base URL:");
    println!(
        r#"
    TideConfig::init()
        .database("postgres://localhost/mydb")
        .file_base_url("https://cdn.example.com/uploads")
        .connect()
        .await?;
    "#
    );

    println!("\n2️⃣  Use field-aware URL generator:");
    println!(
        r#"
    fn smart_url_generator(field_name: &str, file: &FileAttachment) -> String {{
        // Route based on field name first
        match field_name {{
            "thumbnail" => format!("https://thumbs.example.com/{{}}", file.key),
            "avatar" => format!("https://avatars.example.com/{{}}", file.key),
            _ => {{
                // Fall back to mime_type routing
                match file.mime_type.as_deref() {{
                    Some(m) if m.starts_with("video/") => {{
                        format!("https://stream.example.com/{{}}", file.key)
                    }}
                    _ => format!("https://cdn.example.com/{{}}", file.key),
                }}
            }}
        }}
    }}
    
    TideConfig::init()
        .database("postgres://localhost/mydb")
        .file_url_generator(smart_url_generator)
        .connect()
        .await?;
    "#
    );

    println!("\n3️⃣  Serve one field from its own base URL:");
    println!(
        r#"
    TideConfig::init()
        .database("postgres://localhost/mydb")
        .file_base_url("https://cdn.example.com/uploads")
        .file_base_url_for("thumbnail", "https://products-cdn.example.com/thumb")
        .connect()
        .await?;
    "#
    );

    println!("\n4️⃣  Access URLs in JSON output:");
    println!(
        r#"
    let product = Product::find(1).await?;
    let json = product.to_json(None);
    
    // JSON output includes field-specific URLs:
    // {{
    //   "thumbnail": {{
    //     "key": "products/1/thumb.jpg",
    //     "filename": "thumb.jpg",
    //     "url": "https://products-cdn.example.com/thumb/products/1/thumb.jpg"
    //   }},
    //   "gallery": [
    //     {{
    //       "key": "products/1/img1.jpg",
    //       "url": "https://products-cdn.example.com/gallery/products/1/img1.jpg"
    //     }}
    //   ]
    // }}
    "#
    );
}
