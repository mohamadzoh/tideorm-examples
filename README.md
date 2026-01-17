# TideORM Examples

Example applications demonstrating various [TideORM](https://github.com/mohamadzoh/tideorm) features.

[![Website](https://img.shields.io/badge/website-tideorm.com-blue.svg)](https://tideorm.com)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## About

This repository contains comprehensive examples showcasing the features and capabilities of TideORM - a developer-friendly ORM for Rust with clean, expressive syntax. These examples are designed to help you learn and understand how to use TideORM effectively in your projects.

## Prerequisites

Before running any examples, ensure you have:

1. **Rust 1.85+** installed ([rustup.rs](https://rustup.rs))
2. **Database** (PostgreSQL, MySQL, or SQLite depending on the example)
3. **Environment configuration** via `.env` file

### Database Setup

Create a `.env` file in the repository root:

```env
# PostgreSQL (default)
POSTGRESQL_DATABASE_URL=postgres://postgres:postgres@localhost:5432/tideorm_examples

# MySQL (for MySQL examples)
MYSQL_DATABASE_URL=mysql://root:@localhost/tideorm_examples

# SQLite (for SQLite examples)
SQLITE_DATABASE_URL=sqlite:./tideorm_examples.db
```

### Quick PostgreSQL Setup

```bash
# Using psql
psql -U postgres -c "CREATE DATABASE tideorm_examples;"

# Or using Docker
docker run --name postgres-tideorm \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=tideorm_examples \
  -p 5432:5432 \
  -d postgres:latest
```

## Examples Overview

| Example | Description | Run Command |
|---------|-------------|-------------|
| **basic** | Core CRUD operations | `cargo run --bin basic` |
| **query_builder** | Advanced querying (WHERE, ORDER BY, LIMIT) | `cargo run --bin query_builder` |
| **where_and_or_demo** | Comprehensive WHERE & OR conditions | `cargo run --bin where_and_or_demo` |
| **upsert_demo** | Insert-or-update with conflict handling | `cargo run --bin upsert_demo` |
| **postgres_demo** | PostgreSQL-specific features | `cargo run --bin postgres_demo` |
| **postgres_complete** | Complete PostgreSQL showcase | `cargo run --bin postgres_complete` |
| **mysql_demo** | MySQL/MariaDB operations | `cargo run --bin mysql_demo --features mysql --no-default-features` |
| **sqlite_demo** | SQLite embedded database | `cargo run --bin sqlite_demo --features sqlite --no-default-features` |
| **migrations** | Database schema migrations | `cargo run --bin migrations` |
| **schema_file_demo** | SQL schema file generation | `cargo run --bin schema_file_demo` |
| **seeding_demo** | Database seeding with factories | `cargo run --bin seeding_demo` |
| **validation_demo** | Model validation system | `cargo run --bin validation_demo` |
| **caching_demo** | Query caching features | `cargo run --bin caching_demo` |
| **fulltext_demo** | Full-text search with highlighting | `cargo run --bin fulltext_demo` |
| **tokenization_demo** | Secure record ID tokenization | `cargo run --bin tokenization_demo` |
| **attachments_translations_demo** | File attachments & i18n | `cargo run --bin attachments_translations_demo` |
| **attachment_url_demo** | Attachment URL generation | `cargo run --bin attachment_url_demo` |
| **datetime_types_demo** | Date/time type handling | `cargo run --bin datetime_types_demo` |
| **seaorm2_features_demo** | SeaORM 2.0 integration features | `cargo run --bin seaorm2_features_demo` |

## Examples by Category

### 🌊 Basic Operations (CRUD)

Learn the fundamentals with simple create, read, update, and delete operations.

```bash
cargo run --bin basic
```

### 🔍 Query Building

Master the fluent query builder API for complex queries.

```bash
# Basic query building
cargo run --bin query_builder

# Comprehensive WHERE & OR conditions
cargo run --bin where_and_or_demo
```

### 🔄 Upsert Operations

Handle insert-or-update scenarios with conflict resolution.

```bash
cargo run --bin upsert_demo
```

### 🐘 PostgreSQL Features

Explore PostgreSQL-specific functionality.

```bash
# Basic PostgreSQL features
cargo run --bin postgres_demo

# Complete feature showcase
cargo run --bin postgres_complete
```

### 🐬 MySQL/MariaDB Features

```bash
cargo run --bin mysql_demo --features "mysql runtime-tokio" --no-default-features
```

### 🪶 SQLite Features

```bash
cargo run --bin sqlite_demo --features "sqlite runtime-tokio" --no-default-features
```

### 🔄 Database Migrations

Manage database schema changes with versioned migrations.

```bash
cargo run --bin migrations
```

### 📝 Schema Generation

Generate SQL schema files from your models.

```bash
cargo run --bin schema_file_demo
```

### ✅ Validation

Model validation with built-in and custom validators.

```bash
cargo run --bin validation_demo
```

### 💾 Caching

Query result caching for improved performance.

```bash
cargo run --bin caching_demo
```

### 🔍 Full-Text Search

PostgreSQL full-text search with ranking and highlighting.

```bash
cargo run --bin fulltext_demo
```

### 🔐 Tokenization

Secure, reversible record ID encryption.

```bash
cargo run --bin tokenization_demo
```

### 📎 Attachments & Translations

File attachments with metadata and i18n support.

```bash
cargo run --bin attachments_translations_demo
cargo run --bin attachment_url_demo
```

## Learning Path

If you're new to TideORM, we recommend following this order:

1. **Start with basics**: `basic` - Learn CRUD operations
2. **Query building**: `query_builder` - Master the query API
3. **Complex conditions**: `where_and_or_demo` - Advanced WHERE clauses
4. **Upsert operations**: `upsert_demo` - Handle conflicts
5. **Database features**: `postgres_demo` - Database-specific features
6. **Complete reference**: `postgres_complete` - See everything together

## Related Links

- **[TideORM Repository](https://github.com/mohamadzoh/tideorm)** - Main TideORM library
- **[TideORM CLI](https://github.com/mohamadzoh/tideorm-cli)** - Command-line tools
- **[Documentation](https://tideorm.com)** - Official website and docs
- **[API Reference](https://docs.rs/tideorm)** - Complete API documentation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! To add a new example:

1. Create `src/bin/your_example.rs`
2. Add the `[[bin]]` entry to `Cargo.toml`
3. Update this README with your example
4. Ensure it runs with `cargo run --bin your_example`
5. Submit a pull request

## Support

- 📚 [Documentation](https://tideorm.com)
- 💬 [GitHub Discussions](https://github.com/mohamadzoh/tideorm/discussions)
- 🐛 [Issue Tracker](https://github.com/mohamadzoh/tideorm-examples/issues)
