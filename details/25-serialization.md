# 第 25 章：序列化與反序列化

> Serde 是 Rust 生態系中最重要的序列化框架，支援多種資料格式

---

## 25.1 Serde 基礎

```toml
# Cargo.toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    #[serde(default)]
    active: bool,
}

fn main() -> Result<(), serde_json::Error> {
    // 序列化
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        active: true,
    };

    let json = serde_json::to_string(&user)?;
    println!("JSON: {}", json);

    let pretty = serde_json::to_string_pretty(&user)?;
    println!("Pretty:\n{}", pretty);

    // 反序列化
    let json_str = r#"{"id":2,"name":"Bob","email":"bob@example.com"}"#;
    let user: User = serde_json::from_str(json_str)?;
    println!("User: {:?}", user);

    Ok(())
}
```

---

## 25.2 常見格式

### JSON

```rust
use serde_json::{json, Value};

fn main() {
    // 動態 JSON
    let data = json!({
        "name": "Alice",
        "age": 30,
        "friends": ["Bob", "Charlie"]
    });

    // 存取
    println!("Name: {}", data["name"]);

    // Value 型別
    let v: Value = serde_json::from_str(r#"{"key": "value"}"#).unwrap();
    if let Some(key) = v.get("key") {
        println!("Key: {}", key);
    }
}
```

### TOML

```toml
# Cargo.toml
[dependencies]
toml = "0.8"
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
}

fn main() {
    let toml_str = r#"
        [server]
        host = "localhost"
        port = 8080

        [database]
        url = "postgres://localhost/db"
    "#;

    let config: Config = toml::from_str(toml_str).unwrap();
    println!("{:?}", config);
}
```

### YAML

```toml
# Cargo.toml
[dependencies]
serde_yaml = "0.9"
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    items: Vec<String>,
}

fn main() {
    let yaml = r#"
        name: my-app
        items:
          - item1
          - item2
          - item3
    "#;

    let config: Config = serde_yaml::from_str(yaml).unwrap();
    println!("{:?}", config);
}
```

---

## 25.3 自訂序列化

```rust
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "userName")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    #[serde(default = "default_role")]
    role: String,

    #[serde(skip)]
    internal_id: u64,

    #[serde(serialize_with = "serialize_date")]
    created_at: chrono::DateTime<chrono::Utc>,
}

fn default_role() -> String {
    "user".to_string()
}

fn serialize_date<S>(
    date: &chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
}

// 扁平化
#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    #[serde(flatten)]
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
    id: u64,
    name: String,
}
```

---

## 練習題

1. 設計一個支援多種格式的設定檔載入器
2. 實作自訂序列化器處理日期格式
3. 建立 API 回應的序列化結構

---

## 延伸閱讀

- [Serde Documentation](https://serde.rs/)
- [serde_json Documentation](https://docs.rs/serde_json/)
