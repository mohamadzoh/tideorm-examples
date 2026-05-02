//! TideORM auto model discovery demo.
//!
//! This example demonstrates:
//! - discovering compiled `#[tideorm::model]` types by source-path glob
//! - syncing schemas through `TideConfig::models_matching(...)`
//! - using the discovered models normally once the database is connected
//!
//! Run with:
//! `cargo run --bin auto_model_discovery_demo --features "sqlite runtime-tokio" --no-default-features`

use tideorm::{Database, TideConfig, prelude::*, sync::SyncRegistry};

#[path = "auto_model_discovery_demo/models/mod.rs"]
mod models;

use models::{CatalogProduct, WarehouseStock};

const DATABASE_URL: &str = "sqlite::memory:";
const MODEL_PATTERN: &str = "**/auto_model_discovery_demo/models/**/*.rs";

#[tokio::main]
async fn main() -> tideorm::Result<()> {
    println!("TideORM Auto Model Discovery Demo\n");

    Database::reset_global();
    TideConfig::reset();
    SyncRegistry::clear();

    section("1. Discover compiled models by source path");
    let matched_models = SyncRegistry::register_models_matching(MODEL_PATTERN);
    let discovered_tables = registered_table_names();
    assert_eq!(matched_models, 2);
    assert_eq!(
        discovered_tables,
        vec![
            "discovery_products".to_string(),
            "discovery_stock".to_string(),
        ]
    );
    println!("pattern: {MODEL_PATTERN}");
    println!("matched compiled models: {matched_models}");
    println!("registered tables: {:?}", discovered_tables);

    section("2. Sync with TideConfig::models_matching(...)");
    SyncRegistry::clear();
    TideConfig::reset();

    TideConfig::init()
        .database_type(DatabaseType::SQLite)
        .database(DATABASE_URL)
        .max_connections(1)
        .min_connections(1)
        .sync(true)
        .force_sync(true)
        .models_matching(MODEL_PATTERN)
        .connect()
        .await?;

    let synced_tables = registered_table_names();
    assert_eq!(
        synced_tables,
        vec![
            "discovery_products".to_string(),
            "discovery_stock".to_string(),
        ]
    );
    println!("synced tables: {:?}", synced_tables);

    section("3. Persist and query discovered models");
    let product = CatalogProduct::new("SKU-100", "Field-registered product")
        .save()
        .await?;
    WarehouseStock::new(product.id, "main", 18).save().await?;
    WarehouseStock::new(product.id, "overflow", 6)
        .save()
        .await?;

    let stock_rows = WarehouseStock::query()
        .where_eq("product_id", product.id)
        .order_by("warehouse", Order::Asc)
        .get()
        .await?;
    assert_eq!(stock_rows.len(), 2);

    println!("saved product {} ({})", product.id, product.sku);
    for row in stock_rows {
        println!("  - warehouse={} on_hand={}", row.warehouse, row.on_hand);
    }

    println!("\nDemo complete.");
    Ok(())
}

fn registered_table_names() -> Vec<String> {
    let mut table_names: Vec<_> = SyncRegistry::get_all_schemas()
        .into_iter()
        .map(|schema| schema.table_name)
        .collect();
    table_names.sort();
    table_names
}

fn section(title: &str) {
    println!("============================================================");
    println!("{title}");
    println!("============================================================");
}
