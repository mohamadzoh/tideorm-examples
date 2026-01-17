//! # TideORM Examples
//!
//! This crate contains example applications demonstrating various TideORM features.
//!
//! ## Running Examples
//!
//! Examples are compiled as binary targets:
//!
//! ```bash
//! # Basic CRUD operations
//! cargo run --bin basic
//!
//! # Query builder examples
//! cargo run --bin query_builder
//!
//! # PostgreSQL-specific features
//! cargo run --bin postgres_demo
//! cargo run --bin postgres_complete
//!
//! # MySQL (requires mysql feature)
//! cargo run --bin mysql_demo --features "mysql runtime-tokio" --no-default-features
//!
//! # SQLite (requires sqlite feature)
//! cargo run --bin sqlite_demo --features "sqlite runtime-tokio" --no-default-features
//! ```
//!
//! ## Prerequisites
//!
//! Create a `.env` file with your database credentials:
//!
//! ```text
//! POSTGRESQL_DATABASE_URL=postgres://postgres:postgres@localhost:5432/tideorm_examples
//! MYSQL_DATABASE_URL=mysql://root:@localhost/tideorm_examples
//! SQLITE_DATABASE_URL=sqlite:./tideorm_examples.db
//! ```
//!
//! See the [README](https://github.com/mohamadzoh/tideorm-examples) for more details.

// This crate is examples-only, no library code
