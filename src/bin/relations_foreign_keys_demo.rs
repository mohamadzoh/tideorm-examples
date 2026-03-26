//! TideORM relations and foreign keys demo.
//!
//! This example provides a compact, runnable showcase for:
//! - `has_one`, `has_many`, and `belongs_to` relation fields
//! - relation loading with `load`, `exists`, `count`, and `load_with`
//! - relation query helpers like `has_any_related` and `has_related`
//! - foreign-key enforcement on SQLite
//!
//! Run with:
//! `cargo run --bin relations_foreign_keys_demo --features "sqlite runtime-tokio" --no-default-features`

use tideorm::prelude::*;
use tideorm::relations::{BelongsTo, HasMany, HasOne};

#[tideorm::model(table = "authors")]
#[index("email")]
#[unique_index("email")]
pub struct Author {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub email: String,
    pub name: String,

    #[tideorm(has_one = "AuthorProfile", foreign_key = "author_id")]
    pub profile: HasOne<AuthorProfile>,

    #[tideorm(has_many = "Article", foreign_key = "author_id")]
    pub articles: HasMany<Article>,
}

#[tideorm::model(table = "author_profiles")]
#[unique_index("author_id")]
pub struct AuthorProfile {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub author_id: i64,
    pub bio: String,
    pub website: Option<String>,

    #[tideorm(belongs_to = "Author", foreign_key = "author_id")]
    pub author: BelongsTo<Author>,
}

#[tideorm::model(table = "articles")]
#[index("author_id")]
#[index("published")]
pub struct Article {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub published: bool,

    #[tideorm(belongs_to = "Author", foreign_key = "author_id")]
    pub author: BelongsTo<Author>,

    #[tideorm(has_many = "Comment", foreign_key = "article_id")]
    pub comments: HasMany<Comment>,
}

#[tideorm::model(table = "comments")]
#[index("article_id")]
#[index("author_id")]
pub struct Comment {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub article_id: i64,
    pub author_id: i64,
    pub body: String,

    #[tideorm(belongs_to = "Article", foreign_key = "article_id")]
    pub article: BelongsTo<Article>,

    #[tideorm(belongs_to = "Author", foreign_key = "author_id")]
    pub author: BelongsTo<Author>,
}

impl Author {
    fn new(email: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: 0,
            email: email.into(),
            name: name.into(),
            ..Default::default()
        }
    }
}

impl AuthorProfile {
    fn new(author_id: i64, bio: impl Into<String>) -> Self {
        Self {
            id: 0,
            author_id,
            bio: bio.into(),
            website: None,
            ..Default::default()
        }
    }
}

impl Article {
    fn new(author_id: i64, title: impl Into<String>, published: bool) -> Self {
        Self {
            id: 0,
            author_id,
            title: title.into(),
            published,
            ..Default::default()
        }
    }
}

impl Comment {
    fn new(article_id: i64, author_id: i64, body: impl Into<String>) -> Self {
        Self {
            id: 0,
            article_id,
            author_id,
            body: body.into(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> tideorm::Result<()> {
    println!("TideORM Relations and Foreign Keys Demo\n");

    TideConfig::init()
        .database_type(DatabaseType::SQLite)
        .database("sqlite://./relations_foreign_keys_demo.db?mode=rwc")
        .max_connections(1)
        .min_connections(1)
        .sync(true)
        .force_sync(true)
        .models::<(Author, AuthorProfile, Article, Comment)>()
        .connect()
        .await?;

    Database::execute("PRAGMA foreign_keys = ON").await?;

    section("1. Create related records");
    let alice = Author::new("alice@example.com", "Alice Author").save().await?;
    let bob = Author::new("bob@example.com", "Bob Reader").save().await?;
    println!("authors: alice={}, bob={}", alice.id, bob.id);

    let profile = AuthorProfile::new(alice.id, "Writes Rust and database tutorials").save().await?;
    println!("profile created for author_id={}", profile.author_id);

    let article1 = Article::new(alice.id, "Rust ORM relations", true).save().await?;
    let article2 = Article::new(alice.id, "Draft foreign-key checklist", false).save().await?;
    println!("articles: {}, {}", article1.id, article2.id);

    let comment = Comment::new(article1.id, bob.id, "Clear example, thanks.").save().await?;
    println!("comment created on article_id={} by author_id={}", comment.article_id, comment.author_id);

    section("2. Load relations");
    let mut author = Author::find_or_fail(alice.id).await?;
    author.profile = HasOne::new("author_id", "id").with_parent_pk(serde_json::json!(author.id));
    author.articles = HasMany::new("author_id", "id").with_parent_pk(serde_json::json!(author.id));

    let loaded_profile = author.profile.load().await?;
    println!("has_one profile exists: {}", author.profile.exists().await?);
    println!("loaded profile bio: {:?}", loaded_profile.map(|item| item.bio));

    let all_articles = author.articles.load().await?;
    let published_articles = author
        .articles
        .load_with(|query| query.where_eq("published", true).order_by("id", Order::Asc))
        .await?;
    println!("has_many count: {}", author.articles.count().await?);
    println!("all articles: {}", all_articles.len());
    println!("published articles: {}", published_articles.len());

    let mut loaded_article = Article::find_or_fail(article1.id).await?;
    loaded_article.author =
        BelongsTo::new("author_id", "id").with_fk_value(serde_json::json!(loaded_article.author_id));
    loaded_article.comments =
        HasMany::new("article_id", "id").with_parent_pk(serde_json::json!(loaded_article.id));

    let article_author = loaded_article.author.load().await?;
    let article_comments = loaded_article.comments.load().await?;
    println!("belongs_to author: {:?}", article_author.map(|item| item.name));
    println!("article comments: {}", article_comments.len());

    let mut loaded_comment = Comment::find_or_fail(comment.id).await?;
    loaded_comment.article =
        BelongsTo::new("article_id", "id").with_fk_value(serde_json::json!(loaded_comment.article_id));
    loaded_comment.author =
        BelongsTo::new("author_id", "id").with_fk_value(serde_json::json!(loaded_comment.author_id));
    println!(
        "comment article exists: {}, comment author exists: {}",
        loaded_comment.article.exists().await?,
        loaded_comment.author.exists().await?
    );

    section("3. Query by related records");
    let authors_with_articles = Author::query()
        .has_any_related("articles", "author_id", "id")
        .get()
        .await?;
    println!("authors with any articles: {}", authors_with_articles.len());

    let authors_with_published_articles = Author::query()
        .has_related("articles", "author_id", "id", "published", true)
        .get()
        .await?;
    println!("authors with published articles: {}", authors_with_published_articles.len());

    let authors_without_articles = Author::query()
        .has_no_related_at_all("articles", "author_id", "id")
        .get()
        .await?;
    println!("authors without articles: {}", authors_without_articles.len());

    section("4. Foreign-key enforcement");
    let invalid_article = Article::new(999_999, "Should fail", true);
    match invalid_article.save().await {
        Ok(record) => println!("success: article {} saved", record.id),
        Err(error) => println!("insert blocked by FK: {}", error),
    }

    let invalid_comment = Comment::new(article1.id, 999_999, "Invalid commenter");
    match invalid_comment.save().await {
        Ok(record) => println!("success: comment {} saved", record.id),
        Err(error) => println!("comment blocked by FK: {}", error),
    }

    println!("\nDemo complete.");
    Ok(())
}

fn section(title: &str) {
    println!("\n============================================================");
    println!("{}", title);
    println!("============================================================");
}