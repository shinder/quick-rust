# 第八章：非同步程式設計

> Rust 的 async/await 與 JavaScript 非常相似，學起來會很快！

## JS vs Rust 非同步對比

| 概念 | JavaScript | Rust |
|------|------------|------|
| 非同步函式 | `async function` | `async fn` |
| 等待結果 | `await` | `.await` |
| 非同步值 | `Promise` | `Future` |
| 執行時 | V8 Event Loop（內建） | tokio/async-std（需安裝） |
| 並行執行 | `Promise.all()` | `tokio::join!` |

## async/await 基本語法

```javascript
// JavaScript
async function fetchData() {
    const response = await fetch('/api/data');
    const data = await response.json();
    return data;
}
```

```rust
// Rust
async fn fetch_data() -> Result<Data, Error> {
    let response = reqwest::get("/api/data").await?;
    let data = response.json().await?;
    Ok(data)
}
```

### 關鍵差異

1. **`.await` 是後綴**：`response.await` 而非 `await response`
2. **需要執行時**：Rust 標準庫不包含非同步執行時
3. **Future 是惰性的**：不 await 就不會執行

## 設定 Tokio 執行時

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
// 方法 1：使用巨集
#[tokio::main]
async fn main() {
    println!("Hello from async!");
    let result = fetch_data().await;
}

// 方法 2：手動建立執行時
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello from async!");
    });
}
```

## Future 與 Promise

### JavaScript Promise

```javascript
// Promise 立即開始執行
const promise = fetch('/api/data');  // 已經開始了！
// 稍後...
const data = await promise;
```

### Rust Future

```rust
// Future 是惰性的，直到 await 才執行
let future = fetch_data();  // 還沒開始！
// 稍後...
let data = future.await;    // 現在才開始執行
```

### 手動實作 Future（了解原理）

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture;

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll::Pending = 還沒好
        // Poll::Ready(value) = 完成了
        Poll::Ready(42)
    }
}
```

## 並行執行

### JavaScript

```javascript
// 並行執行多個 Promise
const [user, posts, comments] = await Promise.all([
    fetchUser(),
    fetchPosts(),
    fetchComments()
]);

// 競速：第一個完成就回傳
const fastest = await Promise.race([fetch1(), fetch2()]);
```

### Rust

```rust
use tokio;

// 並行執行多個 Future
let (user, posts, comments) = tokio::join!(
    fetch_user(),
    fetch_posts(),
    fetch_comments()
);

// 競速
use tokio::select;
let result = tokio::select! {
    user = fetch_user() => println!("User: {:?}", user),
    posts = fetch_posts() => println!("Posts: {:?}", posts),
};
```

### 動態數量的並行任務

```javascript
// JavaScript
const urls = ['/api/1', '/api/2', '/api/3'];
const results = await Promise.all(urls.map(url => fetch(url)));
```

```rust
// Rust
use futures::future::join_all;

let urls = vec!["/api/1", "/api/2", "/api/3"];
let futures: Vec<_> = urls.iter().map(|url| fetch(url)).collect();
let results = join_all(futures).await;
```

## 非同步 HTTP 請求實戰

### 安裝 reqwest

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### GET 請求

```javascript
// JavaScript
const response = await fetch('https://api.example.com/users');
const users = await response.json();
```

```rust
// Rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let users: Vec<User> = reqwest::get("https://api.example.com/users")
        .await?
        .json()
        .await?;

    println!("{:?}", users);
    Ok(())
}
```

### POST 請求

```javascript
// JavaScript
const response = await fetch('https://api.example.com/users', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name: 'Alice' })
});
```

```rust
// Rust
use serde::Serialize;

#[derive(Serialize)]
struct NewUser {
    name: String,
}

async fn create_user() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let new_user = NewUser { name: String::from("Alice") };

    let response = client
        .post("https://api.example.com/users")
        .json(&new_user)
        .send()
        .await?;

    println!("Status: {}", response.status());
    Ok(())
}
```

### 錯誤處理

```rust
use anyhow::Result;

async fn fetch_with_error_handling() -> Result<String> {
    let response = reqwest::get("https://api.example.com/data")
        .await?;

    // 檢查狀態碼
    if !response.status().is_success() {
        anyhow::bail!("請求失敗: {}", response.status());
    }

    let body = response.text().await?;
    Ok(body)
}
```

## Spawn：背景任務

```javascript
// JavaScript - 不需要特別處理，Promise 自然並行
fetch('/api/1');  // 開始執行
fetch('/api/2');  // 也開始執行
```

```rust
// Rust - 使用 spawn 在背景執行
#[tokio::main]
async fn main() {
    // 產生背景任務
    let handle1 = tokio::spawn(async {
        // 這個任務在背景執行
        fetch_data().await
    });

    let handle2 = tokio::spawn(async {
        // 這個也是
        fetch_other().await
    });

    // 等待結果
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
}
```

## 非同步 Trait（進階）

目前 Rust 的 async trait 需要使用 `async-trait` crate：

```toml
[dependencies]
async-trait = "0.1"
```

```rust
use async_trait::async_trait;

#[async_trait]
trait DataFetcher {
    async fn fetch(&self, url: &str) -> Result<String, Error>;
}

struct HttpFetcher;

#[async_trait]
impl DataFetcher for HttpFetcher {
    async fn fetch(&self, url: &str) -> Result<String, Error> {
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
    }
}
```

## 常見的非同步陷阱

### 1. 忘記 await

```rust
// 錯誤：Future 不會執行
async fn wrong() {
    fetch_data();  // 這行什麼都不會做！
}

// 正確
async fn right() {
    fetch_data().await;  // 加上 .await
}
```

### 2. 阻塞操作在非同步中

```rust
// 錯誤：會阻塞整個執行緒
async fn wrong() {
    std::thread::sleep(Duration::from_secs(1));  // 阻塞！
}

// 正確：使用非同步版本
async fn right() {
    tokio::time::sleep(Duration::from_secs(1)).await;  // 非阻塞
}
```

### 3. 在 async 區塊中使用 ?

```rust
// 如果閉包需要回傳 Result
let result = tokio::spawn(async {
    let data = fetch_data().await?;  // 這裡的 ? 需要 Ok 包裝
    Ok::<_, Error>(data)  // 明確指定型別
}).await?;
```

### 4. Send + Sync 問題

```rust
// 錯誤：Rc 不是 Send，不能跨執行緒
async fn wrong() {
    let rc = std::rc::Rc::new(42);
    tokio::spawn(async move {
        println!("{}", rc);  // 編譯錯誤！
    });
}

// 正確：使用 Arc
async fn right() {
    let arc = std::sync::Arc::new(42);
    tokio::spawn(async move {
        println!("{}", arc);  // OK
    });
}
```

## 完整範例：非同步 Web 爬蟲

```rust
use reqwest;
use tokio;
use futures::future::join_all;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let urls = vec![
        "https://www.rust-lang.org",
        "https://tokio.rs",
        "https://crates.io",
    ];

    // 並行抓取所有 URL
    let futures: Vec<_> = urls
        .iter()
        .map(|url| fetch_title(url))
        .collect();

    let results = join_all(futures).await;

    for (url, result) in urls.iter().zip(results) {
        match result {
            Ok(title) => println!("{}: {}", url, title),
            Err(e) => println!("{}: 錯誤 - {}", url, e),
        }
    }

    Ok(())
}

async fn fetch_title(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    // 簡單提取 title（正式專案用 scraper crate）
    let title = body
        .find("<title>")
        .and_then(|start| {
            let start = start + 7;
            body[start..].find("</title>").map(|end| &body[start..start + end])
        })
        .unwrap_or("無標題")
        .to_string();

    Ok(title)
}
```

## 練習題

### 練習 1：基本非同步
寫一個非同步函式，延遲 1 秒後回傳 "Hello"。

### 練習 2：並行請求
同時請求 3 個不同的 API 端點，等待全部完成後回傳結果。

### 練習 3：超時處理
實作一個有超時機制的 HTTP 請求（5 秒超時）。

```rust
// 提示：使用 tokio::time::timeout
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout(url: &str) -> Result<String> {
    // 你的程式碼
}
```

### 練習 4：並行限制
實作一個爬蟲，同時最多只有 3 個請求在執行（避免過載目標伺服器）。

```rust
// 提示：使用 tokio::sync::Semaphore
```

---

## 恭喜！

你已經完成了這份 Rust 快速入門指南！

### 下一步建議

1. **實作專案**：選一個小專案實際練習
2. **深入閱讀**：[The Rust Programming Language](https://doc.rust-lang.org/book/)
3. **加入社群**：[Rust 中文社群](https://rust.tw/)
4. **持續練習**：[Rustlings](https://github.com/rust-lang/rustlings)

### 常用資源

- [Rust 官方文件](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [crates.io](https://crates.io/)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

---

[← 上一章：模組與套件管理](./07-modules.md) | [回到目錄](./README.md)
