# 第五章：錯誤處理

## Rust 的錯誤處理哲學

JavaScript 使用 `try-catch` 和 `throw`，錯誤可能在任何地方發生。

Rust 強制你處理錯誤：

- **可恢復錯誤**：`Result<T, E>`
- **不可恢復錯誤**：`panic!`（程式崩潰）

## Result 深入

```rust
enum Result<T, E> {
    Ok(T),   // 成功時的值
    Err(E),  // 錯誤時的資訊
}
```

### 基本用法

```javascript
// JavaScript
function readFile(path) {
    try {
        return fs.readFileSync(path, 'utf-8');
    } catch (e) {
        throw new Error(`無法讀取檔案: ${e.message}`);
    }
}
```

```rust
// Rust
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

// 使用
match read_file("hello.txt") {
    Ok(content) => println!("內容: {}", content),
    Err(e) => println!("錯誤: {}", e),
}
```

### 處理 Result 的方式

```rust
let result: Result<i32, &str> = Ok(42);

// 方法 1：match
match result {
    Ok(v) => println!("值: {}", v),
    Err(e) => println!("錯誤: {}", e),
}

// 方法 2：if let
if let Ok(v) = result {
    println!("值: {}", v);
}

// 方法 3：unwrap（不推薦用於正式程式碼）
let value = result.unwrap();  // Err 會 panic

// 方法 4：expect（稍微好一點，有錯誤訊息）
let value = result.expect("取值失敗");

// 方法 5：unwrap_or
let value = result.unwrap_or(0);  // Err 時回傳預設值

// 方法 6：unwrap_or_else
let value = result.unwrap_or_else(|e| {
    println!("發生錯誤: {}", e);
    0
});
```

## ? 運算子：優雅的錯誤傳遞

這是 Rust 錯誤處理的精髓！

```javascript
// JavaScript - 需要手動檢查每一步
async function processFile(path) {
    const content = await readFile(path);
    if (!content) throw new Error("讀取失敗");

    const data = JSON.parse(content);
    if (!data) throw new Error("解析失敗");

    return data;
}
```

```rust
// Rust - 使用 ? 自動傳遞錯誤
fn process_file(path: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;  // 失敗就回傳 Err
    let data: Data = serde_json::from_str(&content)?;  // 失敗就回傳 Err
    Ok(data)
}
```

### ? 的工作原理

```rust
// 這兩種寫法等價：

// 使用 ?
let content = std::fs::read_to_string(path)?;

// 展開後
let content = match std::fs::read_to_string(path) {
    Ok(c) => c,
    Err(e) => return Err(e.into()),
};
```

### 在 main 中使用 ?

```rust
// main 也可以回傳 Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("hello.txt")?;
    println!("{}", content);
    Ok(())
}
```

## Option 與 Result 轉換

```rust
let opt: Option<i32> = Some(42);

// Option -> Result
let result: Result<i32, &str> = opt.ok_or("沒有值");

// Result -> Option
let result: Result<i32, &str> = Ok(42);
let opt: Option<i32> = result.ok();
```

## panic! 與 unwrap

### 何時使用 panic

```rust
// 1. 程式無法繼續執行的情況
panic!("發生嚴重錯誤！");

// 2. 不應該發生的情況（表示程式碼有 bug）
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除數不能為零！這是呼叫者的錯誤。");
    }
    a / b
}
```

### unwrap 和 expect

```rust
// unwrap：取值，None/Err 時 panic
let value = some_option.unwrap();
let value = some_result.unwrap();

// expect：取值，None/Err 時 panic 並顯示訊息
let value = some_option.expect("期望有值");
let value = some_result.expect("操作應該成功");
```

**使用時機**：

- **原型開發**：快速開發時可以用
- **測試程式碼**：測試中合理
- **確定不會失敗**：邏輯上確定不會是 None/Err

## 自訂錯誤類型

### 簡單錯誤

```rust
#[derive(Debug)]
struct MyError {
    message: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError {}

fn do_something() -> Result<(), MyError> {
    Err(MyError {
        message: String::from("發生錯誤"),
    })
}
```

### 使用列舉定義多種錯誤

```rust
#[derive(Debug)]
enum AppError {
    NotFound(String),
    PermissionDenied,
    InvalidInput { field: String, message: String },
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::NotFound(name) => write!(f, "找不到: {}", name),
            AppError::PermissionDenied => write!(f, "權限不足"),
            AppError::InvalidInput { field, message } => {
                write!(f, "欄位 {} 錯誤: {}", field, message)
            }
        }
    }
}

impl std::error::Error for AppError {}
```

### 使用 thiserror crate（推薦）

```rust
// Cargo.toml
// [dependencies]
// thiserror = "1.0"

use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("找不到: {0}")]
    NotFound(String),

    #[error("權限不足")]
    PermissionDenied,

    #[error("欄位 {field} 錯誤: {message}")]
    InvalidInput { field: String, message: String },

    #[error("IO 錯誤")]
    Io(#[from] std::io::Error),  // 自動實作 From
}
```

## 對比 JS 的 try-catch

| 情境           | JavaScript             | Rust                      |
| -------------- | ---------------------- | ------------------------- |
| 可能失敗的操作 | 可能 throw，也可能不會 | 回傳 `Result<T, E>`       |
| 處理錯誤       | `try-catch`            | `match`、`?`、`unwrap_or` |
| 傳遞錯誤       | `throw`                | `?` 或 `return Err(e)`    |
| 忽略錯誤       | 不 catch               | `let _ = result;`         |
| 錯誤類型       | 任意（通常 Error）     | 明確的型別 E              |
| 編譯時檢查     | 無                     | 強制處理 Result           |

### JavaScript 風格 vs Rust 風格

```javascript
// JavaScript - 錯誤可能被忽略
function riskyOperation() {
    const result = mightFail();  // 可能 throw，但編譯器不會提醒
    return result.data;
}
```

```rust
// Rust - 必須處理錯誤
fn risky_operation() -> Result<Data, Error> {
    let result = might_fail()?;  // 編譯器強制你處理
    Ok(result.data)
}
```

## 錯誤處理最佳實踐

### 1. 函式庫：回傳 Result

```rust
// 讓呼叫者決定如何處理錯誤
pub fn parse_config(path: &str) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

### 2. 應用程式：在適當層級處理

```rust
fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("錯誤: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = parse_config("config.toml")?;
    let data = process_data(&config)?;
    save_result(&data)?;
    Ok(())
}
```

### 3. 使用 anyhow 簡化應用程式錯誤處理

```rust
// Cargo.toml
// [dependencies]
// anyhow = "1.0"

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("無法讀取設定檔")?;

    let data = process(&config)
        .context("處理資料失敗")?;

    Ok(())
}
```

## 練習題

### 練習 1：基本錯誤處理

```rust
// 完成這個函式：安全地將字串轉換為數字
fn parse_number(s: &str) -> Result<i32, /* 填入錯誤型別 */> {
    // 你的程式碼
}
```

### 練習 2：使用 ? 運算子

```rust
// 重構這個函式，使用 ? 運算子
fn read_username() -> Result<String, std::io::Error> {
    let f = std::fs::File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### 練習 3：自訂錯誤

定義一個 `ValidationError` 列舉，包含：

- `Empty`：欄位為空
- `TooShort(usize)`：長度不足
- `InvalidFormat(String)`：格式錯誤

實作一個 `validate_email` 函式使用這個錯誤類型。

---

[← 上一章：結構體與列舉](./04-structs-enums.md) | [下一章：集合與迭代器 →](./06-collections.md)
