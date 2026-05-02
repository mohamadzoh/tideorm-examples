//! TideORM encrypted fields demo.
//!
//! This example demonstrates:
//! - model-level encrypted columns via `encrypted = "..."`
//! - startup encryption key configuration with `TideConfig::init().encryption_key(...)`
//! - ciphertext at rest via `Database::raw_json(...)`
//! - automatic decryption on `find()` and `Database::raw::<Model>()`
//! - encrypted batch updates with `update_all().set(...)`
//!
//! Run with:
//! `cargo run --bin encrypted_fields_demo --features "sqlite runtime-tokio encrypted-fields" --no-default-features`

use tideorm::{Database, TideConfig, prelude::*, tokenization::TokenConfig};

const DATABASE_URL: &str = "sqlite::memory:";
const ENCRYPTION_KEY: &str = "encrypted-fields-demo-key-32chars";

#[tideorm::model(
    table = "encrypted_contacts",
    encrypted = "customer_phone_number,backup_phone,internal_note"
)]
#[unique_index("email")]
pub struct SecureContact {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub name: String,
    pub email: String,
    #[tideorm(column = "customer_phone_number")]
    pub phone_number: String,
    #[tideorm(nullable)]
    pub backup_phone: Option<String>,
    #[tideorm(nullable)]
    pub internal_note: Option<String>,
}

impl SecureContact {
    fn demo_record() -> Self {
        Self {
            id: 0,
            name: "Alice Example".to_string(),
            email: "alice@example.com".to_string(),
            phone_number: "+1-555-0101".to_string(),
            backup_phone: Some("+1-555-0199".to_string()),
            internal_note: Some("Send invoices to finance@example.com".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> tideorm::Result<()> {
    println!("TideORM Encrypted Fields Demo\n");

    Database::reset_global();
    TideConfig::reset();
    TokenConfig::reset();

    TideConfig::init()
        .database_type(DatabaseType::SQLite)
        .database(DATABASE_URL)
        .max_connections(1)
        .min_connections(1)
        .sync(true)
        .force_sync(true)
        .models::<(SecureContact,)>()
        .encryption_key(ENCRYPTION_KEY)
        .connect()
        .await?;

    section("1. Save a model with plaintext Rust fields");
    let created = SecureContact::demo_record().save().await?;
    println!("created id: {}", created.id);
    println!("phone_number in Rust: {}", created.phone_number);
    println!("backup_phone in Rust: {:?}", created.backup_phone);
    println!("internal_note in Rust: {:?}", created.internal_note);

    section("2. Inspect the raw database row");
    let stored_row = Database::raw_json(
        "SELECT name, email, customer_phone_number, backup_phone, internal_note FROM encrypted_contacts",
    )
    .await?
    .into_iter()
    .next()
    .expect("encrypted contact row should exist");
    let stored_phone = stored_row["customer_phone_number"]
        .as_str()
        .expect("stored encrypted phone should be a string");
    let stored_backup = stored_row["backup_phone"]
        .as_str()
        .expect("stored encrypted backup phone should be a string");
    let stored_note = stored_row["internal_note"]
        .as_str()
        .expect("stored encrypted note should be a string");

    assert_eq!(stored_row["name"].as_str(), Some(created.name.as_str()));
    assert_eq!(stored_row["email"].as_str(), Some(created.email.as_str()));
    assert!(stored_phone.starts_with("enc::"));
    assert!(stored_backup.starts_with("enc::"));
    assert!(stored_note.starts_with("enc::"));
    assert_ne!(stored_phone, created.phone_number);
    assert_ne!(stored_backup, created.backup_phone.as_deref().unwrap());
    assert_ne!(stored_note, created.internal_note.as_deref().unwrap());

    println!("name stays plaintext in storage: {:?}", stored_row["name"]);
    println!(
        "customer_phone_number stored as ciphertext: {}",
        preview(stored_phone)
    );
    println!(
        "backup_phone stored as ciphertext: {}",
        preview(stored_backup)
    );
    println!(
        "internal_note stored as ciphertext: {}",
        preview(stored_note)
    );

    section("3. TideORM decrypts on normal model loads");
    let loaded = SecureContact::find(created.id)
        .await?
        .expect("saved contact should load");
    assert_eq!(loaded.phone_number, created.phone_number);
    assert_eq!(loaded.backup_phone, created.backup_phone);
    assert_eq!(loaded.internal_note, created.internal_note);
    println!(
        "find() returned plaintext phone_number: {}",
        loaded.phone_number
    );

    let hydrated = Database::raw::<SecureContact>("SELECT * FROM encrypted_contacts")
        .await?
        .into_iter()
        .next()
        .expect("raw hydrated contact should exist");
    assert_eq!(hydrated.phone_number, created.phone_number);
    println!("Database::raw::<SecureContact>() also returned plaintext fields.");

    section("4. Batch updates re-encrypt encrypted fields");
    let rows_affected = SecureContact::update_all()
        .where_eq("id", created.id)
        .set("phone_number", "+1-555-0111")
        .execute()
        .await?;
    assert_eq!(rows_affected, 1);

    let updated_row = Database::raw_json("SELECT customer_phone_number FROM encrypted_contacts")
        .await?
        .into_iter()
        .next()
        .expect("updated contact row should exist");
    let updated_phone = updated_row["customer_phone_number"]
        .as_str()
        .expect("updated encrypted phone should be a string");
    assert!(updated_phone.starts_with("enc::"));
    assert_ne!(updated_phone, "+1-555-0111");

    let updated = SecureContact::find(created.id)
        .await?
        .expect("updated contact should load");
    assert_eq!(updated.phone_number, "+1-555-0111");
    println!("update_all().set(...) wrote fresh ciphertext and reloaded as plaintext.");
    Ok(())
}

fn section(title: &str) {
    println!("============================================================");
    println!("{title}");
    println!("============================================================");
}

fn preview(value: &str) -> String {
    let prefix_len = value.len().min(32);
    format!("{}...", &value[..prefix_len])
}
