//! TideORM entity manager demo.
//!
//! This example demonstrates:
//! - aggregate loading and persistence with `EntityManager::find`, `load`, and `save`
//! - compatibility helpers such as `find_in_entity_manager`, `load_in_entity_manager`, and
//!   `save_with_entity_manager`
//! - managed lifecycle operations: `persist`, `find_managed`, `merge`, `remove`, `detach`, and
//!   `flush`
//!
//! Run with:
//! `cargo run --bin entity_manager_demo --features "sqlite runtime-tokio entity-manager" --no-default-features`

use std::sync::Arc;

use tideorm::prelude::*;

#[tideorm::model(table = "entity_manager_users")]
#[unique_index("email")]
pub struct DemoUser {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub email: String,
    pub name: String,

    #[tideorm(has_one = "DemoProfile", foreign_key = "user_id")]
    pub profile: HasOne<DemoProfile>,

    #[tideorm(has_many = "DemoTask", foreign_key = "user_id")]
    pub tasks: HasMany<DemoTask>,
}

#[tideorm::model(table = "entity_manager_profiles")]
#[unique_index("user_id")]
pub struct DemoProfile {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub user_id: i64,
    pub bio: String,

    #[tideorm(belongs_to = "DemoUser", foreign_key = "user_id")]
    pub user: BelongsTo<DemoUser>,
}

#[tideorm::model(table = "entity_manager_tasks")]
#[index("user_id")]
pub struct DemoTask {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub done: bool,

    #[tideorm(belongs_to = "DemoUser", foreign_key = "user_id")]
    pub user: BelongsTo<DemoUser>,
}

impl DemoUser {
    fn new(email: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: 0,
            email: email.into(),
            name: name.into(),
            ..Default::default()
        }
    }
}

impl DemoProfile {
    fn new(user_id: i64, bio: impl Into<String>) -> Self {
        Self {
            id: 0,
            user_id,
            bio: bio.into(),
            ..Default::default()
        }
    }
}

impl DemoTask {
    fn new(user_id: i64, title: impl Into<String>, done: bool) -> Self {
        Self {
            id: 0,
            user_id,
            title: title.into(),
            done,
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> tideorm::Result<()> {
    println!("TideORM Entity Manager Demo\n");

    let db = Arc::new(
        TideConfig::init()
            .database_type(DatabaseType::SQLite)
            .database("sqlite://./entity_manager_demo.db?mode=rwc")
            .max_connections(1)
            .min_connections(1)
            .sync(true)
            .force_sync(true)
            .models::<(DemoUser, DemoProfile, DemoTask)>()
            .connect()
            .await?
            .clone(),
    );

    Database::execute("PRAGMA foreign_keys = ON").await?;

    section("1. Seed baseline data");
    let seeded_user = DemoUser::new("owner@example.com", "Aggregate Owner").save().await?;
    DemoTask::new(seeded_user.id, "Audit the aggregate", false)
        .save()
        .await?;
    println!("seeded user {} with one task", seeded_user.id);

    section("2. Aggregate workflow through EntityManager");
    let aggregate_manager = EntityManager::new(db.clone());
    let mut user = aggregate_manager
        .find::<DemoUser>(seeded_user.id)
        .await?
        .expect("seeded user should exist");

    aggregate_manager.load(&mut user.profile).await?;
    aggregate_manager.load(&mut user.tasks).await?;

    println!(
        "loaded profile? {} | loaded task count: {}",
        user.profile.get_cached().is_some(),
        user.tasks.get_cached().map_or(0, |tasks| tasks.len())
    );

    user.profile
        .set_cached(Some(DemoProfile::new(0, "Owns the entity-manager example")));

    let tasks = user.tasks.as_mut().expect("tasks should be loaded");
    tasks[0].done = true;
    tasks.push(DemoTask::new(0, "Document aggregate save behavior", false));

    let mut user = aggregate_manager.save(&user).await?;
    let saved_profile = user.profile.get_cached().expect("profile should be inserted");
    assert!(saved_profile.id > 0);
    assert_eq!(saved_profile.user_id, user.id);

    let stored_tasks = DemoTask::query_with(db.as_ref())
        .where_eq("user_id", user.id)
        .order_by("id", Order::Asc)
        .get()
        .await?;
    assert_eq!(stored_tasks.len(), 2);

    println!(
        "saved profile {} and tasks: {:?}",
        saved_profile.id,
        stored_tasks
            .iter()
            .map(|task| format!("{} [{}]", task.title, if task.done { "done" } else { "open" }))
            .collect::<Vec<_>>()
    );

    user.tasks
        .as_mut()
        .expect("tasks should remain loaded")
        .remove(0);
    let user = aggregate_manager.save(&user).await?;
    let remaining_tasks = DemoTask::query_with(db.as_ref())
        .where_eq("user_id", user.id)
        .count()
        .await?;
    assert_eq!(remaining_tasks, 1);
    println!("after removing a loaded child, database task count is {}", remaining_tasks);

    section("3. Compatibility helpers");
    let mut compatibility_user = DemoUser::find_in_entity_manager(user.id, &aggregate_manager)
        .await?
        .expect("user should still be available through compatibility helper");
    compatibility_user
        .tasks
        .load_in_entity_manager(&aggregate_manager)
        .await?;
    compatibility_user
        .tasks
        .as_mut()
        .expect("tasks should be loaded in compatibility helper")
        .push(DemoTask::new(0, "Exercise compatibility helpers", false));

    let compatibility_user = save_with_entity_manager(&compatibility_user, &aggregate_manager).await?;
    let compatibility_tasks = compatibility_user
        .tasks
        .get_cached()
        .expect("tasks should stay loaded after compatibility save");
    assert_eq!(compatibility_tasks.len(), 2);
    println!(
        "compatibility save kept {} loaded tasks in the aggregate cache",
        compatibility_tasks.len()
    );

    section("4. Managed lifecycle operations");
    let lifecycle_manager = EntityManager::new(db.clone());

    let inserted = lifecycle_manager.persist(DemoUser::new(
        "managed.insert@example.com",
        "Managed Insert",
    ));
    assert_eq!(inserted.state(), EntityState::New);
    lifecycle_manager.flush().await?;
    let inserted_id = inserted.get().id;
    assert!(inserted_id > 0);
    assert_eq!(inserted.state(), EntityState::Managed);
    println!("persist + flush inserted managed user {}", inserted_id);

    let managed_existing = lifecycle_manager
        .find_managed::<DemoUser>(user.id)
        .await?
        .expect("aggregate user should load as managed");
    managed_existing.edit(|item| item.name = "Managed Update".to_string());
    lifecycle_manager.flush().await?;
    assert_eq!(managed_existing.get().name, "Managed Update");
    println!("find_managed + flush updated user {}", managed_existing.get().id);

    let merged = lifecycle_manager.merge(DemoUser {
        id: inserted_id,
        email: "managed.insert@example.com".to_string(),
        name: "Merged Name".to_string(),
        ..Default::default()
    })?;
    lifecycle_manager.flush().await?;
    assert_eq!(merged.get().name, "Merged Name");
    println!("merge + flush renamed user {}", inserted_id);

    let removable = lifecycle_manager.persist(DemoUser::new("remove.me@example.com", "Remove Me"));
    let detachable = lifecycle_manager.persist(DemoUser::new("detach.me@example.com", "Detach Me"));
    lifecycle_manager.flush().await?;

    let removable_id = removable.get().id;
    let detachable_id = detachable.get().id;
    let removable = lifecycle_manager
        .find_managed::<DemoUser>(removable_id)
        .await?
        .expect("removable user should load as managed");
    let detachable = lifecycle_manager
        .find_managed::<DemoUser>(detachable_id)
        .await?
        .expect("detachable user should load as managed");

    lifecycle_manager.remove(&removable);
    lifecycle_manager.detach(&detachable);
    detachable.edit(|item| item.name = "Detached Update".to_string());
    lifecycle_manager.flush().await?;

    assert!(DemoUser::find_with(removable_id, db.as_ref()).await?.is_none());
    let detached_row = DemoUser::find_with(detachable_id, db.as_ref())
        .await?
        .expect("detached row should still exist in the database");
    assert_eq!(detached_row.name, "Detach Me");

    println!(
        "remove deleted user {}, while detached user {} stayed unchanged in the database",
        removable_id, detachable_id
    );

    println!("\nDemo complete.");
    Ok(())
}

fn section(title: &str) {
    println!("\n============================================================");
    println!("{}", title);
    println!("============================================================");
}