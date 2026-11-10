//! Typed Columns Demo
//!
//! This example demonstrates these TideORM features:
//!
//! 1. **Strongly-Typed Columns** - Compile-time type safety for queries
//! 2. **Nested ActiveModel** - Cascade save operations for related models
//! 3. **Self-Referencing Relations** - Hierarchical data support
//! 4. **Join Result Consolidation** - Nest flat join results
//! 5. **Linked Partial Select** - Select specific columns from related tables
//! 6. **Eager Loading Builder** - Load related records in batches
//!
//! The demo runs without a database. The calls that need one live in the
//! `snippets` module at the end of this file, which the compiler checks; the
//! demo prints them from this file's own source, so what it shows is real API.
//!
//! Run with: `cargo run --bin typed_columns_demo`

use tideorm::columns::{ColumnEq, ColumnIn, ColumnLike, ColumnNullable, ColumnOrd};
use tideorm::prelude::*;

// =============================================================================
// MODEL DEFINITIONS
// =============================================================================

// Plain structs for the parts of the demo that run without a database; the
// models are in `snippets`.

/// Employee row - self-referencing (org chart)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Employee {
    id: i64,
    name: String,
    title: String,
    manager_id: Option<i64>,
}

/// Order model for join examples
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct Order {
    id: i64,
    customer_name: String,
    total: i64,
}

/// LineItem model - belongs to Order
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct LineItem {
    id: i64,
    order_id: i64,
    product_name: String,
    quantity: i32,
    price: i64,
}

// =============================================================================
// FEATURE 1: STRONGLY-TYPED COLUMNS
// =============================================================================

/// Define typed columns for compile-time safety
mod user_columns {
    use tideorm::columns::Column;

    /// User ID column (i64)
    pub const ID: Column<i64> = Column::new("id");
    /// User name column (String)
    pub const NAME: Column<String> = Column::new("name");
    /// User age column (nullable i32)
    pub const AGE: Column<Option<i32>> = Column::new("age");
    /// User active status column (bool)
    pub const ACTIVE: Column<bool> = Column::new("active");
}

fn demo_strongly_typed_columns() {
    println!("\n=== STRONGLY-TYPED COLUMNS ===\n");

    // Import our typed columns
    use user_columns::*;

    // Create type-safe conditions
    println!("Creating type-safe query conditions:");

    // String operations
    let name_eq = NAME.eq("Alice");
    println!(
        "  NAME.eq(\"Alice\")        -> column: {}, op: {:?}",
        name_eq.column, name_eq.operator
    );

    let name_like = NAME.contains("test");
    println!("  NAME.contains(\"test\")   -> value: {}", name_like.value);

    let name_starts = NAME.starts_with("Mr.");
    println!(
        "  NAME.starts_with(\"Mr.\") -> value: {}",
        name_starts.value
    );

    // Numeric operations
    let id_gt = ID.gt(100);
    println!(
        "  ID.gt(100)               -> column: {}, op: {:?}",
        id_gt.column, id_gt.operator
    );

    let id_between = ID.between(1, 1000);
    println!("  ID.between(1, 1000)      -> value: {}", id_between.value);

    let age_null = AGE.is_null();
    println!("  AGE.is_null()            -> op: {:?}", age_null.operator);

    let age_not_null = AGE.is_not_null();
    println!(
        "  AGE.is_not_null()        -> op: {:?}",
        age_not_null.operator
    );

    // Boolean operations
    let active_true = ACTIVE.eq(true);
    println!("  ACTIVE.eq(true)          -> value: {}", active_true.value);

    // IN operations
    let id_in = ID.is_in(vec![1, 2, 3, 4, 5]);
    println!("  ID.is_in([1,2,3,4,5])    -> op: {:?}", id_in.operator);

    let name_in = NAME.is_in(vec!["Alice", "Bob", "Charlie"]);
    println!("  NAME.is_in([...])        -> value: {}", name_in.value);

    println!("\n   Compile-time type safety:");
    println!("     - NAME.eq(\"text\")  ✓ String == &str");
    println!("     - ID.gt(100)        ✓ i64 > i64");
    println!("     - AGE.is_null()     ✓ Only on Option<T>");
    println!("     - NAME.like(\"%a%\") ✓ Only on String");
    println!("     - NAME.eq(123)      ✗ COMPILE ERROR! String != i32");
    println!("     - ID.like(\"%a%\")   ✗ COMPILE ERROR! i64 has no .like()");

    println!("\n  Usage with QueryBuilder (generated column constants):");
    show("pub async fn typed_filters(");
}

// =============================================================================
// FEATURE 2: NESTED ACTIVE MODEL (CASCADE SAVE)
// =============================================================================

fn demo_nested_active_model() {
    println!("\n=== NESTED ACTIVE MODEL (CASCADE SAVE) ===\n");

    println!("NestedSave saves a parent and its children in one transaction,");
    println!("setting each child's foreign key to the parent's key:\n");
    show("pub async fn nested_saves(");

    println!("NestedSaveBuilder collects children of several models:\n");
    show("pub async fn nested_builder(");
}

// =============================================================================
// FEATURE 3: SELF-REFERENCING RELATIONS
// =============================================================================

fn demo_self_referencing_relations() {
    println!("\n=== SELF-REFERENCING RELATIONS ===\n");

    // Example org chart
    let employees = vec![
        Employee {
            id: 1,
            name: "CEO".to_string(),
            title: "Chief Executive".to_string(),
            manager_id: None,
        },
        Employee {
            id: 2,
            name: "VP Engineering".to_string(),
            title: "Vice President".to_string(),
            manager_id: Some(1),
        },
        Employee {
            id: 3,
            name: "VP Sales".to_string(),
            title: "Vice President".to_string(),
            manager_id: Some(1),
        },
        Employee {
            id: 4,
            name: "Tech Lead".to_string(),
            title: "Lead Engineer".to_string(),
            manager_id: Some(2),
        },
        Employee {
            id: 5,
            name: "Developer".to_string(),
            title: "Software Engineer".to_string(),
            manager_id: Some(4),
        },
    ];

    println!("Self-referencing relationships (like org charts):\n");

    println!("Model definition:");
    show("#[tideorm::model(table = \"employees\")]");

    println!("Sample Org Chart:");
    for emp in &employees {
        let indent = match emp.manager_id {
            None => "",
            Some(1) => "  └─ ",
            Some(2) => "      └─ ",
            Some(4) => "          └─ ",
            _ => "    ",
        };
        println!("  {}{} ({})", indent, emp.name, emp.title);
    }

    println!("\nSelfRef<E> and SelfRefMany<E> operations:");
    show("pub async fn org_chart(");
}

// =============================================================================
// FEATURE 4: JOIN RESULT CONSOLIDATION
// =============================================================================

fn demo_join_consolidation() {
    println!("\n=== JOIN RESULT CONSOLIDATION ===\n");

    // Simulate flat join results
    let order1 = Order {
        id: 1,
        customer_name: "Alice".to_string(),
        total: 150,
    };
    let order2 = Order {
        id: 2,
        customer_name: "Bob".to_string(),
        total: 75,
    };

    let flat_results: Vec<(Order, LineItem)> = vec![
        (
            order1.clone(),
            LineItem {
                id: 1,
                order_id: 1,
                product_name: "Widget".to_string(),
                quantity: 2,
                price: 50,
            },
        ),
        (
            order1.clone(),
            LineItem {
                id: 2,
                order_id: 1,
                product_name: "Gadget".to_string(),
                quantity: 1,
                price: 50,
            },
        ),
        (
            order2.clone(),
            LineItem {
                id: 3,
                order_id: 2,
                product_name: "Gizmo".to_string(),
                quantity: 3,
                price: 25,
            },
        ),
    ];

    println!("Problem: JOIN queries return flat results:\n");
    println!("  SELECT * FROM orders JOIN line_items ON ...\n");
    println!("  Flat Results:");
    for (order, item) in &flat_results {
        println!(
            "    (Order #{} '{}', LineItem #{} '{}')",
            order.id, order.customer_name, item.id, item.product_name
        );
    }

    println!("\nSolution: JoinResultConsolidator nests them:\n");

    // Use the consolidator
    let nested = JoinResultConsolidator::consolidate_two(flat_results, |o| o.id);

    println!("  JoinResultConsolidator::consolidate_two(flat, |o| o.id)");
    println!("\n  Nested Results:");
    for (order, items) in &nested {
        println!(
            "    Order #{} '{}' (${}):",
            order.id, order.customer_name, order.total
        );
        for item in items {
            println!(
                "      └─ {} x {} @ ${}",
                item.quantity, item.product_name, item.price
            );
        }
    }

    println!("\nAvailable methods:");
    println!("  consolidate_two(flat, key_fn)");
    println!("    Vec<(A, B)> -> Vec<(A, Vec<B>)>\n");

    println!("  consolidate_two_optional(flat, key_fn)");
    println!("    Vec<(A, Option<B>)> -> Vec<(A, Vec<B>)>");
    println!("    (handles LEFT JOIN nulls)\n");

    println!("  consolidate_three(flat, key_a_fn, key_b_fn)");
    println!("    Vec<(A, B, C)> -> Vec<(A, Vec<(B, Vec<C>)>)>\n");

    println!("  consolidate_three_optional(flat, key_a_fn, key_b_fn)");
    println!("    Vec<(A, B, Option<C>)> -> Vec<(A, Vec<(B, Vec<C>)>)>");
}

// =============================================================================
// FEATURE 5: LINKED PARTIAL SELECT
// =============================================================================

fn demo_linked_partial_select() {
    println!("\n=== LINKED PARTIAL SELECT ===\n");

    println!("Select specific columns from main + related tables with auto-join:\n");

    println!("Rows are read into a struct of your own, by column name:");
    show("#[derive(serde::Deserialize)]");
    show("pub async fn linked_select(");

    println!("Benefits:");
    println!("   Only fetches columns you need (performance)");
    println!("   Auto-generates the JOIN clause");
    println!("   Type-safe column selection");
    println!("   Works with any related model");
}

// =============================================================================
// FEATURE 6: EAGER LOADING BUILDER
// =============================================================================

fn demo_eager_loading_builder() {
    println!("\n=== EAGER LOADING BUILDER ===\n");

    println!("Eager loading avoids N+1 query patterns when traversing relations:\n");

    show("pub async fn eager_loading(");

    println!("Benefits:");
    println!("   Batch-loads relation graphs instead of per-row follow-up queries");
    println!("   Supports nested relation paths like posts.comments");
    println!("   Keeps the fluent query builder syntax for filters, sort, and pagination");
}

// =============================================================================
// MAIN
// =============================================================================

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           TideORM - Typed Columns Demo                        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    demo_strongly_typed_columns();
    demo_nested_active_model();
    demo_self_referencing_relations();
    demo_join_consolidation();
    demo_linked_partial_select();
    demo_eager_loading_builder();

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                     Demo Complete!                             ║");
    println!("║                                                                 ║");
    println!("║  All of these are available in TideORM:                       ║");
    println!("║  - Strongly-typed columns with compile-time safety            ║");
    println!("║  - Nested ActiveModel for cascade saves                       ║");
    println!("║  - Self-referencing relations for hierarchical data           ║");
    println!("║  - Join result consolidation                                   ║");
    println!("║  - Linked partial select                                       ║");
    println!("║  - Eager loading builder                                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}

/// Print one item of `snippets`, from `start` to its closing brace, as this
/// file holds it.
fn show(start: &str) {
    const SOURCE: &str = include_str!("typed_columns_demo.rs");
    let module = SOURCE
        .find("\nmod snippets {")
        .expect("this file has `snippets`");
    let from = module + SOURCE[module..].find(start).expect("the snippet exists");
    // Items in `snippets` close at an indent of four spaces.
    let to = from + SOURCE[from..].find("\n    }").expect("the snippet ends") + "\n    }".len();
    println!("    {}\n", &SOURCE[from..to]);
}

// =============================================================================
// COMPILE-CHECKED SNIPPETS
// =============================================================================

/// What the demo prints. These need a database, so nothing calls them; that
/// they compile is what keeps the printed code real.
#[allow(
    dead_code,
    reason = "printed by the demo, not run: they need a database"
)]
mod snippets {
    use tideorm::prelude::*;

    #[tideorm::model(table = "users")]
    pub struct User {
        #[tideorm(primary_key, auto_increment)]
        pub id: i64,
        pub name: String,
        pub age: Option<i32>,
        pub active: bool,
        #[tideorm(has_one = "Profile", foreign_key = "user_id")]
        pub profile: HasOne<Profile>,
        #[tideorm(has_many = "Post", foreign_key = "user_id")]
        pub posts: HasMany<Post>,
    }

    #[tideorm::model(table = "profiles")]
    pub struct Profile {
        #[tideorm(primary_key, auto_increment)]
        pub id: i64,
        pub user_id: i64,
        pub bio: Option<String>,
        pub website: Option<String>,
    }

    #[tideorm::model(table = "posts")]
    pub struct Post {
        #[tideorm(primary_key, auto_increment)]
        pub id: i64,
        pub user_id: i64,
        pub title: String,
        #[tideorm(has_many = "Comment", foreign_key = "post_id")]
        pub comments: HasMany<Comment>,
    }

    #[tideorm::model(table = "comments")]
    pub struct Comment {
        #[tideorm(primary_key, auto_increment)]
        pub id: i64,
        pub post_id: i64,
        pub body: String,
    }

    #[tideorm::model(table = "employees")]
    pub struct Employee {
        #[tideorm(primary_key, auto_increment)]
        pub id: i64,
        pub name: String,
        pub manager_id: Option<i64>,

        // Parent reference (manager)
        #[tideorm(foreign_key = "manager_id")]
        pub manager: SelfRef<Employee>,

        // Children reference (direct reports)
        #[tideorm(foreign_key = "manager_id")]
        pub reports: SelfRefMany<Employee>,
    }

    pub async fn typed_filters() -> tideorm::Result<Vec<User>> {
        User::query()
            .where_col(User::NAME.starts_with("Al"))
            .where_col(User::AGE.gt(18))
            .where_col(User::ACTIVE.eq(true))
            .order_desc(User::ID)
            .get()
            .await
    }

    pub async fn nested_saves(
        user: User,
        profile: Profile,
        posts: Vec<Post>,
    ) -> tideorm::Result<u64> {
        // Inserted or updated; profile.user_id = user.id
        let (user, profile) = user.save_with_one(profile, Profile::USER_ID).await?;
        // Each post.user_id = user.id
        let (user, posts) = user.save_with_many(posts, Post::USER_ID).await?;
        // Both already stored: plain updates in one transaction
        let (user, _profile) = user.update_with_one(profile).await?;
        let (user, posts) = user.update_with_many(posts).await?;
        // Children first, then the parent; returns the rows deleted
        user.delete_with_many(posts).await
    }

    pub async fn nested_builder(
        user: User,
        profile: Profile,
        posts: Vec<Post>,
    ) -> tideorm::Result<Vec<Post>> {
        let (_user, mut saved) = NestedSaveBuilder::new(user)
            .with_one(profile, Profile::USER_ID)
            .with_many(posts, Post::USER_ID)
            .save()
            .await?;
        // One entry per with_one/with_many, in order
        let posts = saved
            .pop()
            .expect("with_many was added")
            .into_many::<Post>()?;
        Ok(posts)
    }

    pub async fn org_chart() -> tideorm::Result<()> {
        if let Some(developer) = Employee::find(5).await? {
            // Load the manager (parent)
            let _manager: Option<Employee> = developer.manager.load().await?;
            let _has_manager = developer.manager.exists().await?;
        }
        if let Some(vp) = Employee::find(2).await? {
            // Direct reports, their count, and the subtree three levels deep
            let _reports: Vec<Employee> = vp.reports.load().await?;
            let _count = vp.reports.count().await?;
            let _tree = vp.reports.load_tree(3).await?;
        }
        Ok(())
    }

    #[derive(serde::Deserialize)]
    pub struct UserBio {
        pub id: i64,
        pub name: String,
        pub bio: Option<String>,
        pub website: Option<String>,
    }

    pub async fn linked_select() -> tideorm::Result<Vec<serde_json::Value>> {
        // users.id = profiles.user_id; some columns from each table
        let _bios: Vec<UserBio> = User::query()
            .select_with_linked(
                ["id", "name"],
                "profiles",
                "id",
                "user_id",
                ["bio", "website"],
            )
            .get_as()
            .await?;
        // Every user column, plus the profile's
        User::query()
            .select_also_linked("profiles", "id", "user_id", ["bio", "website"])
            .get_json()
            .await
    }

    pub async fn eager_loading() -> tideorm::Result<()> {
        // Filters, order and paging go on the query, before `with`
        let _active = User::query()
            .where_eq(User::ACTIVE, true)
            .order_desc(User::ID)
            .with(User::PROFILE)
            .get()
            .await?;

        // Several relations, nested paths included, in one pass
        let users = User::query()
            .limit(10)
            .with_many([User::PROFILE, User::POSTS, "posts.comments"])
            .get()
            .await?;

        // Read the loaded relations from the models' own fields
        for user in &users {
            let posts = user.posts.get_cached().map(|posts| posts.len());
            println!("{}: {posts:?} posts", user.name);
        }
        Ok(())
    }
}
