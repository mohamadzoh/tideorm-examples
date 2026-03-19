//! TideORM Profiling Demo
//!
//! This example demonstrates TideORM's profiling utilities without requiring a
//! live database connection.
//!
//! ## Features Demonstrated
//!
//! - `Profiler` for manual query timing sessions
//! - `ProfiledQuery` metadata helpers (`with_table`, `with_rows`, `cached`)
//! - `ProfileReport` summaries and slow query filtering
//! - `GlobalProfiler` aggregate statistics
//! - `QueryAnalyzer` optimization suggestions and complexity estimation
//!
//! Run with: `cargo run --bin profiling_demo`

use std::thread;
use std::time::Duration;
use tideorm::prelude::{GlobalProfiler, ProfiledQuery, Profiler, QueryAnalyzer};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║               TideORM Profiling Demo                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    manual_profiler_demo();
    global_profiler_demo();
    query_analyzer_demo();
}

fn manual_profiler_demo() {
    section("1. MANUAL PROFILER");

    let mut profiler = Profiler::start();

    simulate_query(
        &mut profiler,
        ProfiledQuery::new(
            "SELECT id, name, email FROM users WHERE active = true ORDER BY created_at DESC LIMIT 20",
            Duration::from_millis(14),
        )
        .with_table("users")
        .with_rows(20),
    );

    simulate_query(
        &mut profiler,
        ProfiledQuery::new(
            "SELECT * FROM posts WHERE user_id = 42 ORDER BY created_at DESC",
            Duration::from_millis(126),
        )
        .with_table("posts")
        .with_rows(8),
    );

    simulate_query(
        &mut profiler,
        ProfiledQuery::new(
            "SELECT COUNT(*) FROM comments WHERE post_id = 7",
            Duration::from_millis(5),
        )
        .with_table("comments")
        .with_rows(1)
        .cached(),
    );

    println!("   Queries recorded so far: {}", profiler.query_count());
    println!("   Elapsed wall time: {:.2}ms", profiler.elapsed().as_secs_f64() * 1000.0);

    let report = profiler.stop();
    println!("\n{}", report);

    let slow_queries = report.queries_slower_than(Duration::from_millis(100));
    println!("\n   Slow queries (>= 100ms): {}", slow_queries.len());
    for query in slow_queries {
        println!(
            "     - {}ms on {:?}: {}",
            query.duration.as_millis(),
            query.table,
            preview_sql(&query.sql)
        );
    }

    println!("\n   Report suggestions:");
    for suggestion in report.suggestions() {
        println!("     - {}", suggestion);
    }
}

fn global_profiler_demo() {
    section("2. GLOBAL PROFILER");

    GlobalProfiler::reset();
    GlobalProfiler::set_slow_threshold(75);
    GlobalProfiler::enable();

    record_global_query(Duration::from_millis(21));
    record_global_query(Duration::from_millis(84));
    record_global_query(Duration::from_millis(133));

    let stats = GlobalProfiler::stats();
    println!("{}", stats);

    GlobalProfiler::disable();
    GlobalProfiler::reset();
}

fn query_analyzer_demo() {
    section("3. QUERY ANALYZER");

    let sql = "SELECT * FROM users WHERE LOWER(email) LIKE '%@example.com' OR status = 'active' ORDER BY created_at";

    println!("   SQL: {}", sql);

    let suggestions = QueryAnalyzer::analyze(sql);
    println!("\n   Suggestions:");
    for suggestion in suggestions {
        println!("\n{}", suggestion);
    }

    let complexity = QueryAnalyzer::estimate_complexity(sql);
    println!("\n   Complexity: {}", complexity);
}

fn simulate_query(profiler: &mut Profiler, query: ProfiledQuery) {
    thread::sleep(Duration::from_millis(10));
    println!(
        "   Recorded {:>4}ms {:<6} {}",
        query.duration.as_millis(),
        query.operation,
        preview_sql(&query.sql)
    );
    profiler.record_full(query);
}

fn record_global_query(duration: Duration) {
    thread::sleep(Duration::from_millis(5));
    GlobalProfiler::record(duration);
    println!("   Recorded global query: {:>4}ms", duration.as_millis());
}

fn preview_sql(sql: &str) -> String {
    const MAX_LEN: usize = 72;
    if sql.chars().count() <= MAX_LEN {
        sql.to_string()
    } else {
        let preview: String = sql.chars().take(MAX_LEN - 3).collect();
        format!("{}...", preview)
    }
}

fn section(title: &str) {
    println!("\n══════════════════════════════════════════════════════════════");
    println!("  {}", title);
    println!("══════════════════════════════════════════════════════════════\n");
}