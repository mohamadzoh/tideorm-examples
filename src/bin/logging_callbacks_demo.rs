//! TideORM logging and callbacks demo.
//!
//! This example fills the remaining observability and lifecycle-hook coverage in
//! this repository by demonstrating:
//! - programmatic query logging configuration
//! - query builder debug output without executing SQL
//! - real model lifecycle callbacks during save/update/delete
//! - query history, slow-query filtering, and error context inspection
//!
//! Run with:
//! `cargo run --bin logging_callbacks_demo --features "sqlite runtime-tokio" --no-default-features`

use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use tideorm::callbacks::Callbacks;
use tideorm::prelude::*;

#[tideorm::model(table = "accounts")]
#[index("email")]
#[unique_index("email")]
pub struct Account {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub email: String,
    pub name: String,
    pub active: bool,
}

impl Account {
    fn new(email: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: 0,
            email: email.into(),
            name: name.into(),
            active: true,
        }
    }
}

impl Callbacks for Account {
    fn before_validation(&mut self) -> tideorm::Result<()> {
        push_event("before_validation");
        self.name = self.name.trim().to_string();
        Ok(())
    }

    fn after_validation(&self) -> tideorm::Result<()> {
        push_event("after_validation");
        Ok(())
    }

    fn before_save(&mut self) -> tideorm::Result<()> {
        push_event("before_save");
        self.email = self.email.trim().to_lowercase();
        Ok(())
    }

    fn after_save(&self) -> tideorm::Result<()> {
        push_event("after_save");
        Ok(())
    }

    fn before_create(&mut self) -> tideorm::Result<()> {
        push_event("before_create");
        Ok(())
    }

    fn after_create(&self) -> tideorm::Result<()> {
        push_event("after_create");
        Ok(())
    }

    fn before_update(&mut self) -> tideorm::Result<()> {
        push_event("before_update");
        Ok(())
    }

    fn after_update(&self) -> tideorm::Result<()> {
        push_event("after_update");
        Ok(())
    }

    fn before_delete(&self) -> tideorm::Result<()> {
        push_event("before_delete");
        if self.email == "admin@example.com" {
            return Err(tideorm::Error::validation(
                "email",
                "Cannot delete the protected admin account",
            ));
        }
        Ok(())
    }

    fn after_delete(&self) -> tideorm::Result<()> {
        push_event("after_delete");
        Ok(())
    }
}

static CALLBACK_EVENTS: OnceLock<Mutex<Vec<&'static str>>> = OnceLock::new();

fn callback_events() -> &'static Mutex<Vec<&'static str>> {
    CALLBACK_EVENTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn push_event(event: &'static str) {
    callback_events().lock().unwrap().push(event);
}

fn take_events() -> Vec<&'static str> {
    let mut guard = callback_events().lock().unwrap();
    let events = guard.clone();
    guard.clear();
    events
}

#[tokio::main]
async fn main() -> tideorm::Result<()> {
    println!("TideORM Logging and Callbacks Demo\n");

    QueryLogger::clear_history();
    QueryLogger::reset_stats();
    QueryLogger::global()
        .set_level(LogLevel::Trace)
        .enable_timing(true)
        .set_slow_query_threshold_ms(5)
        .set_history_limit(50)
        .enable();

    let debug_info = Account::query()
        .where_eq(Account::columns.active, true)
        .where_like(Account::columns.email, "%@example.com")
        .order_by("id", Order::Desc)
        .limit(5)
        .debug();

    section("1. Query Debug Output");
    println!("{}", debug_info);

    TideConfig::init()
        .database_type(DatabaseType::SQLite)
        .database("sqlite://./logging_callbacks_demo.db?mode=rwc")
        .max_connections(1)
        .min_connections(1)
        .sync(true)
        .force_sync(true)
        .models::<(Account,)>()
        .connect()
        .await?;

    section("2. Create With Callbacks");
    let admin = Account::new(" ADMIN@EXAMPLE.COM ", "  Admin User  ").save().await?;
    println!("created: id={}, email={}, name={}", admin.id, admin.email, admin.name);
    println!("callback order: {:?}", take_events());

    section("3. Update With Callbacks");
    let mut admin = Account::find_or_fail(admin.id).await?;
    admin.name = " Admin User Reviewed ".to_string();
    let admin = admin.update().await?;
    println!("updated: id={}, email={}, name={}", admin.id, admin.email, admin.name);
    println!("callback order: {:?}", take_events());

    section("4. Delete Guard Callback");
    match admin.delete().await {
        Ok(rows) => println!("unexpected delete success: {}", rows),
        Err(error) => {
            println!("delete blocked: {}", error);
            println!("callback order: {:?}", take_events());
        }
    }

    section("5. Successful Delete");
    let user = Account::new("person@example.com", "Person User").save().await?;
    let _ = take_events();
    let deleted = user.delete().await?;
    println!("deleted rows: {}", deleted);
    println!("callback order: {:?}", take_events());

    section("6. Logger History And Stats");
    let active_accounts = Account::query()
        .where_eq(Account::columns.active, true)
        .order_by("email", Order::Asc)
        .get()
        .await?;
    println!("active accounts loaded: {}", active_accounts.len());

    let timed_entry = QueryTimer::start("SELECT * FROM accounts WHERE active = ?")
        .with_table("accounts");
    tokio::time::sleep(Duration::from_millis(12)).await;
    QueryLogger::log(timed_entry.finish_with_rows(active_accounts.len() as u64));

    let failed_entry = QueryTimer::start("DELETE FROM accounts WHERE email = ?")
        .with_table("accounts");
    tokio::time::sleep(Duration::from_millis(2)).await;
    QueryLogger::log(failed_entry.finish_with_error("blocked by before_delete callback"));

    let history = QueryLogger::history();
    println!("history entries: {}", history.len());
    for entry in history.iter().rev().take(5) {
        println!(
            "  - op={}, table={:?}, rows={:?}, duration_ms={:?}, success={}",
            entry.operation,
            entry.table,
            entry.rows,
            entry.duration.map(|duration| duration.as_millis()),
            entry.success
        );
    }

    let slow_queries = QueryLogger::slow_queries();
    println!("slow queries: {}", slow_queries.len());
    println!("\n{}", QueryLogger::stats());

    section("7. Error Context");
    if let Err(error) = Account::find_or_fail(999_999).await {
        println!("error: {}", error);
        if let Some(context) = error.context() {
            println!("table: {:?}", context.table);
            println!("column: {:?}", context.column);
            println!("query: {:?}", context.query);
        }
    }

    QueryLogger::disable();
    Ok(())
}

fn section(title: &str) {
    println!("\n============================================================");
    println!("{}", title);
    println!("============================================================");
}