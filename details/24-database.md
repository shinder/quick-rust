# 第 24 章：資料庫操作

> Rust 生態系提供了多種資料庫操作方式，從原生 SQL 到 ORM

---

## 24.1 SQL 資料庫

### sqlx

```toml
# Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 建立連線池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:pass@localhost/mydb")
        .await?;

    // 查詢
    let users: Vec<User> = sqlx::query_as("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await?;

    // 參數化查詢
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(1)
        .fetch_one(&pool)
        .await?;

    // 插入
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("Alice")
        .bind("alice@example.com")
        .execute(&pool)
        .await?;

    println!("插入 {} 筆", result.rows_affected());

    // 交易
    let mut tx = pool.begin().await?;

    sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
        .bind("Bob")
        .bind(1)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
```

### 編譯時期檢查

```rust
// 使用 sqlx::query! 進行編譯時期 SQL 檢查
// 需要設定 DATABASE_URL 環境變數

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    // 編譯時期驗證 SQL
    let users = sqlx::query!("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{}: {}", user.id, user.name);
    }

    Ok(())
}
```

### diesel ORM

```toml
# Cargo.toml
[dependencies]
diesel = { version = "2", features = ["postgres"] }
dotenvy = "0.15"
```

```rust
use diesel::prelude::*;

// schema.rs (由 diesel CLI 產生)
diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    name: &'a str,
    email: &'a str,
}

fn main() {
    use users::dsl::*;

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let mut conn = PgConnection::establish(&database_url).unwrap();

    // 查詢
    let results: Vec<User> = users
        .filter(name.like("%Alice%"))
        .limit(5)
        .load(&mut conn)
        .unwrap();

    // 插入
    let new_user = NewUser {
        name: "Bob",
        email: "bob@example.com",
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .unwrap();
}
```

---

## 24.2 NoSQL 資料庫

### Redis

```toml
# Cargo.toml
[dependencies]
redis = { version = "0.24", features = ["tokio-comp"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use redis::AsyncCommands;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    // 設定和取得
    con.set("key", "value").await?;
    let value: String = con.get("key").await?;

    // 過期時間
    con.set_ex("temp_key", "temp_value", 60).await?;

    // Hash
    con.hset("user:1", "name", "Alice").await?;
    con.hset("user:1", "email", "alice@example.com").await?;
    let name: String = con.hget("user:1", "name").await?;

    // List
    con.lpush("queue", "item1").await?;
    con.rpush("queue", "item2").await?;
    let item: String = con.lpop("queue", None).await?;

    Ok(())
}
```

### MongoDB

```toml
# Cargo.toml
[dependencies]
mongodb = "2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

```rust
use mongodb::{Client, options::ClientOptions, bson::doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    age: i32,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("mydb");
    let collection = db.collection::<User>("users");

    // 插入
    let user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };
    collection.insert_one(user, None).await?;

    // 查詢
    let filter = doc! { "name": "Alice" };
    if let Some(user) = collection.find_one(filter, None).await? {
        println!("找到: {:?}", user);
    }

    // 更新
    collection.update_one(
        doc! { "name": "Alice" },
        doc! { "$set": { "age": 31 } },
        None,
    ).await?;

    Ok(())
}
```

---

## 練習題

1. 使用 sqlx 建立一個完整的 CRUD 應用
2. 實作 Redis 快取層
3. 設計 MongoDB 文件結構

---

## 延伸閱讀

- [sqlx Documentation](https://docs.rs/sqlx/)
- [Diesel Documentation](https://diesel.rs/)
- [Redis Documentation](https://docs.rs/redis/)
