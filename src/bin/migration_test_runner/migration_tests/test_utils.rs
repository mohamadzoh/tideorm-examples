//! Migration Test Utilities
//!
//! Helper functions for migration testing.

#![allow(dead_code)]

use serde_json::Value;
use tideorm::{Database, DbValue, Error};

// A failed or malformed read is an error, never `false`, `0` or an empty list: a
// check that something is gone must not pass because the catalog could not be read.

/// The `column` of the first row `sql` returns, with `params` bound to its `$n` markers.
async fn first_value<T>(
    sql: &str,
    params: Vec<DbValue>,
    column: &str,
    read: impl Fn(&Value) -> Option<T>,
) -> tideorm::Result<T> {
    let rows = Database::raw_json_with_params(sql, params).await?;
    rows.first()
        .and_then(|row| read(&row[column]))
        .ok_or_else(|| Error::query(format!("no `{column}` value in {rows:?} from {sql}")))
}

/// The `exists` column of the first row `sql` returns.
async fn exists(sql: &str, params: Vec<DbValue>) -> tideorm::Result<bool> {
    first_value(sql, params, "exists", Value::as_bool).await
}

/// The `cnt` column of the first row `sql` returns.
async fn count(sql: &str) -> tideorm::Result<i64> {
    first_value(sql, Vec::new(), "cnt", Value::as_i64).await
}

/// The `version` column of every row `sql` returns.
async fn versions(sql: &str) -> tideorm::Result<Vec<String>> {
    Database::raw_json(sql)
        .await?
        .iter()
        .map(|row| {
            row["version"]
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| Error::query(format!("no `version` in {row} from {sql}")))
        })
        .collect()
}

/// Check if a table exists in the database
pub async fn table_exists(table_name: &str) -> tideorm::Result<bool> {
    exists(
        r#"SELECT EXISTS (
            SELECT FROM information_schema.tables
            WHERE table_schema = 'public' AND table_name = $1
        ) AS "exists""#,
        vec![table_name.into()],
    )
    .await
}

/// Check if a column exists in a table
pub async fn column_exists(table_name: &str, column_name: &str) -> tideorm::Result<bool> {
    exists(
        r#"SELECT EXISTS (
            SELECT FROM information_schema.columns
            WHERE table_schema = 'public' AND table_name = $1 AND column_name = $2
        ) AS "exists""#,
        vec![table_name.into(), column_name.into()],
    )
    .await
}

/// Check if an index exists
pub async fn index_exists(index_name: &str) -> tideorm::Result<bool> {
    exists(
        r#"SELECT EXISTS (
            SELECT FROM pg_indexes
            WHERE schemaname = 'public' AND indexname = $1
        ) AS "exists""#,
        vec![index_name.into()],
    )
    .await
}

/// Get count of records in migrations table
pub async fn get_migration_count() -> tideorm::Result<i64> {
    count(r#"SELECT COUNT(*)::bigint AS cnt FROM "_migrations""#).await
}

/// Get count of test migrations (those with version starting with "20260106_")
pub async fn get_test_migration_count() -> tideorm::Result<i64> {
    count(
        r#"SELECT COUNT(*)::bigint AS cnt FROM "_migrations" WHERE starts_with("version", '20260106_')"#,
    )
    .await
}

/// Get all applied migration versions
pub async fn get_applied_versions() -> tideorm::Result<Vec<String>> {
    versions(r#"SELECT "version" FROM "_migrations" ORDER BY "version" ASC"#).await
}

/// Get applied test migration versions (those with version starting with "20260106_")
pub async fn get_test_applied_versions() -> tideorm::Result<Vec<String>> {
    versions(
        r#"SELECT "version" FROM "_migrations" WHERE starts_with("version", '20260106_') ORDER BY "version" ASC"#,
    )
    .await
}

/// Clean up all test tables, and the test migrations' ledger rows
pub async fn cleanup_test_tables() -> tideorm::Result<()> {
    let tables = [
        "test_inventory",
        "test_order_items",
        "test_orders",
        "test_products",
    ];

    for table in tables {
        Database::execute(&format!(r#"DROP TABLE IF EXISTS "{table}" CASCADE"#)).await?;
    }

    // A fresh database has no ledger yet.
    if table_exists("_migrations").await? {
        Database::execute(r#"DELETE FROM "_migrations" WHERE starts_with("version", '20260106_')"#)
            .await?;
    }

    Ok(())
}

/// Print test result with formatting
pub fn print_test_result(name: &str, passed: bool) {
    if passed {
        println!("   {}", name);
    } else {
        println!("  ❌ {}", name);
    }
}

/// Print section header
pub fn print_section(title: &str) {
    println!("\n{}", "=".repeat(60));
    println!("📋 {}", title);
    println!("{}", "=".repeat(60));
}
