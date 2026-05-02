//! TideORM dirty tracking demo.
//!
//! This example demonstrates:
//! - `changed_fields()` on loaded and modified models
//! - `original_value()` before and after `update()`
//! - the shared baseline behavior when multiple in-memory copies represent the same row
//!
//! Run with:
//! `cargo run --bin dirty_tracking_demo --features "sqlite runtime-tokio dirty-tracking" --no-default-features`

use serde_json::json;
use tideorm::{Database, TideConfig, prelude::*};

const DATABASE_URL: &str = "sqlite::memory:";

#[tideorm::model(table = "dirty_tracking_users")]
#[unique_index("email")]
pub struct TrackedUser {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub email: String,
    pub name: String,
    pub active: bool,
}

impl TrackedUser {
    fn new(email: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: 0,
            email: email.into(),
            name: name.into(),
            active: true,
        }
    }
}

#[tokio::main]
async fn main() -> tideorm::Result<()> {
    println!("TideORM Dirty Tracking Demo\n");

    Database::reset_global();
    TideConfig::reset();

    TideConfig::init()
        .database_type(DatabaseType::SQLite)
        .database(DATABASE_URL)
        .max_connections(1)
        .min_connections(1)
        .sync(true)
        .force_sync(true)
        .models::<(TrackedUser,)>()
        .connect()
        .await?;

    let seeded = TrackedUser::new("dirty@example.com", "Alice Original")
        .save()
        .await?;

    section("1. Fresh loads start with a clean baseline");
    let mut loaded = TrackedUser::find(seeded.id)
        .await?
        .expect("seeded model should exist");
    assert!(loaded.changed_fields()?.is_empty());
    assert_eq!(
        loaded.original_value("name")?,
        Some(json!("Alice Original"))
    );
    println!(
        "changed fields after find(): {:?}",
        loaded.changed_fields()?
    );
    println!(
        "original name baseline: {:?}",
        loaded.original_value("name")?
    );

    section("2. Unsaved edits expose changed fields and original values");
    loaded.name = "Alice Pending".to_string();
    assert_eq!(loaded.changed_fields()?, vec!["name"]);
    assert_eq!(
        loaded.original_value("name")?,
        Some(json!("Alice Original"))
    );
    println!("changed fields after edit: {:?}", loaded.changed_fields()?);
    println!(
        "original name before update(): {:?}",
        loaded.original_value("name")?
    );

    section("3. update() refreshes the dirty-tracking baseline");
    let updated = loaded.update().await?;
    assert!(updated.changed_fields()?.is_empty());
    assert_eq!(
        updated.original_value("name")?,
        Some(json!("Alice Pending"))
    );
    println!(
        "changed fields after update(): {:?}",
        updated.changed_fields()?
    );
    println!(
        "new baseline after update(): {:?}",
        updated.original_value("name")?
    );

    section("4. Stale copies compare against the latest persisted snapshot");
    let mut first = TrackedUser::find(seeded.id)
        .await?
        .expect("first copy should exist");
    let mut second = TrackedUser::find(seeded.id)
        .await?
        .expect("second copy should exist");

    first.name = "Bob Persisted".to_string();
    let first = first.update().await?;
    assert_eq!(first.original_value("name")?, Some(json!("Bob Persisted")));

    second.name = "Charlie Local".to_string();
    assert_eq!(second.changed_fields()?, vec!["name"]);
    assert_eq!(second.original_value("name")?, Some(json!("Bob Persisted")));
    println!("latest persisted name: {:?}", first.original_value("name")?);
    println!("stale copy changed fields: {:?}", second.changed_fields()?);
    println!(
        "stale copy original value now points to: {:?}",
        second.original_value("name")?
    );

    println!("\nDemo complete.");
    Ok(())
}

fn section(title: &str) {
    println!("============================================================");
    println!("{title}");
    println!("============================================================");
}
