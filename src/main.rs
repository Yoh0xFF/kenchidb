use crate::{common::DatabaseError, database::Database};

mod database;
mod common;
mod macros;
mod storage;
mod schema;

define_schema! {
    User {
        id: long,
        name: string,
        email: string,
        age: int,
        is_active: boolean,
        balance: double,
    }
}

fn main() -> Result<(), DatabaseError> {
    println!("🗄️  KenchiDB Demo");

    // Create database
    let mut db = Database::new();

    // Create collections with schemas
    db.create_collection("users".to_string(), User::schema())?;

    // Insert users
    let users = db.collection("users").unwrap();

    let user1 = User::create()
        .set("id", 1i64)
        .set("name", "Alice Johnson")
        .set("email", "alice@example.com")
        .set("age", 28i32)
        .set("is_active", true)
        .set("balance", 1250.75f64)
        .build();

    let user2 = User::create()
        .set("id", 2i64)
        .set("name", "Bob Smith")
        .set("email", "bob@example.com")
        .set("age", 35i32)
        .set("is_active", false)
        .set("balance", 500.0f64)
        .build();

    let user3 = User::create()
        .set("id", 3i64)
        .set("name", "Carol Davis")
        .set("email", "carol@example.com")
        .set("age", 22i32)
        .set("is_active", true)
        .set("balance", 3200.50f64)
        .build();

    let user1_id = users.insert(user1)?;
    let user2_id = users.insert(user2)?;
    let user3_id = users.insert(user3)?;

    println!(
        "✅ Inserted users with IDs: {}, {}, {}",
        user1_id, user2_id, user3_id
    );

    return Ok(());
}
