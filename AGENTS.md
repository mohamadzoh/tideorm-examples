# Writing applications with TideORM

Instructions for AI coding agents working in a Rust project that uses TideORM. TideORM is an
async, ActiveRecord-style ORM: a model is a plain struct, its relations are fields of that struct,
and the model's own type carries the methods that read and write it. Every example in this
repository (`src/bin/*.rs`) builds against the published crate.

Full reference: the API cheat sheet (<https://github.com/mohamadzoh/tideorm/blob/master/API.md>),
the guide (<https://tideorm.com>, all of it as one file at <https://tideorm.com/llms-full.txt>)
and <https://docs.rs/tideorm>.

## Habits from other Rust ORMs that do not apply

| Do not write | Write |
| --- | --- |
| A separate entity, model and active-model type per table | One struct with `#[tideorm::model(table = "..")]` |
| A `Relation` enum, `Related` impls or join tables declared in a schema DSL | A field: `pub posts: HasMany<Post>` with its `foreign_key` |
| `Entity::find().filter(Column::Email.eq(x)).one(&db)` | `User::query().where_eq(User::EMAIL, x).first().await?` |
| Filling an active model with `Set(..)` and calling `.insert(&db)` | `User::create(user).await?`, or `user.save().await?` |
| A schema macro or a generated `schema.rs` kept beside the structs | The struct is the mapping; tables come from migrations (`Schema`) |
| `&db`, `&txn` or `&mut conn` passed to every call | Nothing: calls use the global database, join the enclosing `transaction`, and `db.scope(..)` picks another database |
| Blocking calls on a connection | Everything is `async` (Tokio or async-std) |
| `format!` building SQL | Typed filters, or raw SQL with bound parameters |

## Dependency

```toml
[dependencies]
tideorm = { version = "0.18", default-features = false, features = ["postgres", "runtime-tokio"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Backends: `postgres`, `mysql` (MySQL and MariaDB), `sqlite`. Runtimes: `runtime-tokio`,
`runtime-async-std`. Module features, each only adding API: `dirty-tracking`, `entity-manager`,
`attachments`, `translations`, `fulltext`, `encrypted-fields`. A method that does not exist at
compile time usually needs its feature. MSRV is Rust 1.94; edition 2024.

## Models

```rust
use tideorm::prelude::*;

#[tideorm::model(table = "users")]
pub struct User {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub email: String,
    pub active: bool,

    #[tideorm(has_many = "Post", foreign_key = "user_id")]
    pub posts: HasMany<Post>,
}

#[tideorm::model(table = "posts", soft_delete)]
pub struct Post {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub deleted_at: Option<DateTime<Utc>>,     // DateTime and Utc are in the prelude

    #[tideorm(belongs_to = "User", foreign_key = "user_id")]
    pub author: BelongsTo<User>,
}
```

- Import everything from `tideorm::prelude::*`. Do not depend on `tideorm-engine`,
  `tideorm-sql-builder` or `tideorm-driver` directly; they are implementation details.
- The macro generates typed column constants (`User::ID`, `User::EMAIL`, also `User::columns.email`)
  and relation constants (`User::POSTS`). Prefer them to string names: a typo or a value of the
  wrong type is a compile error.
- Relation wrappers: `HasOne`, `HasMany`, `BelongsTo` (`foreign_key`), `HasManyThrough<T, Pivot>`
  (`foreign_key`, `related_key`), `MorphOne`, `MorphMany`, `MorphTo` (`morph_name`), `SelfRef`,
  `SelfRefMany`. Keys are never guessed: name them.
- Your own `#[derive(..)]` (serde, `Debug`, `Clone`) goes in a separate attribute **below**
  `#[tideorm::model(..)]`. Above it, or in the same list, it conflicts (E0119).
- Composite keys: mark each key field `primary_key` and pass a tuple to `find((a, b))`.

## Connecting

```rust
TideConfig::init()
    .database(&std::env::var("DATABASE_URL")?)
    .models::<(User, Post)>() // or .auto_models()
    .connect()
    .await?;
```

`sync(true)` creates missing tables and columns for development only; deployed schemas use
migrations (`Migrator`, `Schema`). `force_sync(true)` drops every registered table on each start.
`require_db()?` returns the global database or an error; `db()` panics when there is none.

## Reading

```rust
let users = User::query()
    .where_eq(User::ACTIVE, true)
    .where_in(User::ID, &ids)
    .order_desc(User::ID)
    .limit(20)
    .get()
    .await?;
let user = User::find(1).await?;                  // Option<User>
let emails: Vec<String> = User::query().pluck(User::EMAIL).await?;
let page = User::query().paginate(1, 20).await?;   // or cursor_paginate(20, cursor)
```

- `or_where_*` calls form one OR group ANDed with the other filters:
  `where_eq(a).or_where_eq(b).or_where_eq(c)` is `a AND (b OR c)`. For other shapes use
  `begin_or() .. end_or()` or `where_any`.
- Eager loading: `User::query().with(User::POSTS).get()`, then `user.posts.get_cached()`.
  Filters come before `with(..)`; after it, `.query(|q| q.where_eq(..))` adds more.
  `user.posts.load().await?` always queries the database.
- `get()` must select every model column. To read some columns use `pluck`, `get_json()`,
  `get_as::<T>()`, or `Database::raw_as::<T>` with a `#[derive(FromQueryResult)]` struct.
- Soft-delete models leave trashed rows out of every read; `with_trashed()` / `only_trashed()`
  reach them.

## Writing

```rust
let user = User::create(User { email: "ada@example.com".into(), active: true, ..Default::default() }).await?;
let mut user = User::find(user.id).await?.unwrap();
user.active = false;
let user = user.update().await?;   // writes every column of the model
user.delete().await?;              // soft-deletes on a soft_delete model

User::update_all()
    .set(User::ACTIVE, false)
    .where_lt(User::ID, 100)
    .execute()
    .await?;
User::insert_all(users).await?;                          // one transaction, keys returned in order
User::insert_or_update(user, [User::EMAIL]).await?;      // upsert on a unique key
```

- `update()` and `save()` write the whole row, so of two requests that each change one field, the
  last one wins. For a partial write use `update_all().set(..)` with a filter on the key.
- Bulk `update`/`delete` without a filter that can exclude a row are refused by design.
- `insert_all` runs validation but no callbacks.

## Transactions

```rust
require_db()?.transaction(async |_tx| {
    let user = User::create(user).await?;      // joins the transaction
    Post::create(Post { user_id: user.id, title: "Hello".into(), ..Default::default() }).await?;
    Ok(())                                     // an Err rolls back
}).await?;
```

Calls inside the closure join the transaction without passing a handle; a nested
`transaction` is a savepoint. A task spawned with `tokio::spawn` inside does **not** join it.
`db.scope(async { .. })` runs calls on another database (a tenant, a replica) without a
transaction.

## Raw SQL and safety

- Never format values or identifiers into SQL. In a builder, `where_raw_with("price > ?", vec![10.into()])`
  takes `?` on every backend, and a fragment holding `$1` is refused.
  `Database::raw_with_params::<User>(sql, params)` and `exec_raw_with_params` send the SQL as
  written, so use the backend's markers: `$1`, `$2` on PostgreSQL, `?` on MySQL, MariaDB and SQLite.
- Two result columns with one name (`SELECT u.*, p.id ..`) cannot be read by name: alias one
  (`p.id AS post_id`).
- On SQLite, `"name"` is always an identifier: write string literals in single quotes.
- Encrypted fields (`encrypted = "phone"`) cannot be filtered, sorted or grouped by value.
- Errors are `tideorm::Error`; `is_unique_violation()`, `is_foreign_key_violation()` and
  `is_not_null_violation()` classify constraint failures. Return `tideorm::Result<T>`.

## Debugging

`query.debug()` shows the SQL and parameters a builder will run; `TIDE_LOG_QUERIES=1` prints every
executed statement (never its bound values). Tests reset global state with
`Database::reset_global()` and `TideConfig::reset()`.
