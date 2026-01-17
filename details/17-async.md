# 第 17 章：非同步程式設計

> 非同步程式設計讓你能夠高效地處理大量 I/O 密集型任務，而不需要為每個任務建立獨立的執行緒

---

## 17.1 非同步基礎

非同步程式設計是一種並行處理的方式，特別適合 I/O 密集型任務。

### async/await 語法

```rust
// async 函式回傳一個 Future
async fn hello() -> String {
    String::from("Hello, async!")
}

// await 等待 Future 完成
async fn greet() {
    let message = hello().await;
    println!("{}", message);
}

// 使用 tokio 執行時期
// Cargo.toml: tokio = { version = "1", features = ["full"] }
#[tokio::main]
async fn main() {
    greet().await;
}
```

### Future Trait

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Future trait 的簡化定義
// pub trait Future {
//     type Output;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }

// 手動實作 Future
struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        if self.count < 3 {
            // 還沒完成，請稍後再輪詢
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            // 完成了
            Poll::Ready(self.count)
        }
    }
}

#[tokio::main]
async fn main() {
    let future = MyFuture { count: 0 };
    let result = future.await;
    println!("結果: {}", result);
}
```

### async 區塊

```rust
#[tokio::main]
async fn main() {
    // async 區塊創建匿名 Future
    let future = async {
        println!("執行中...");
        42
    };

    let result = future.await;
    println!("結果: {}", result);

    // 捕獲環境變數
    let name = String::from("Rust");
    let greeting = async move {
        format!("Hello, {}!", name)
    };

    println!("{}", greeting.await);
}
```

### Pin 與 Unpin

```rust
use std::pin::Pin;

#[tokio::main]
async fn main() {
    // 大多數時候不需要直接處理 Pin
    // async/await 會自動處理

    // 但在某些情況下需要手動 pin
    let future = async { 42 };

    // 使用 Box::pin
    let pinned: Pin<Box<dyn std::future::Future<Output = i32>>> =
        Box::pin(future);

    let result = pinned.await;
    println!("結果: {}", result);

    // 使用 pin! 巨集（需要 tokio 的 macros feature）
    tokio::pin!(async { 43 });
}
```

---

## 17.2 非同步執行時

Rust 標準庫只提供 Future trait，需要外部執行時（runtime）來實際執行非同步程式碼。

### Tokio

```rust
// Cargo.toml:
// tokio = { version = "1", features = ["full"] }

// 單執行緒執行時
#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("單執行緒模式");
}

// 多執行緒執行時（預設）
#[tokio::main]
async fn main() {
    println!("多執行緒模式");
}

// 自訂執行時配置
#[tokio::main(worker_threads = 4)]
async fn main() {
    println!("4 個工作執行緒");
}

// 手動建立執行時
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        println!("在執行時中執行");
    });
}
```

### async-std

```rust
// Cargo.toml:
// async-std = { version = "1", features = ["attributes"] }

use async_std::task;

#[async_std::main]
async fn main() {
    println!("使用 async-std");

    // spawn 任務
    let handle = task::spawn(async {
        42
    });

    let result = handle.await;
    println!("結果: {}", result);
}
```

### 執行時比較

| 特性 | Tokio | async-std | smol |
|------|-------|-----------|------|
| 成熟度 | 最高 | 高 | 中 |
| 效能 | 優秀 | 優秀 | 優秀 |
| 生態系 | 最大 | 中等 | 較小 |
| 學習曲線 | 中等 | 較低 | 較低 |
| 適用場景 | 通用 | 通用 | 輕量應用 |

---

## 17.3 Tokio 深入

### spawn 與 JoinHandle

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // spawn 建立非同步任務
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "任務完成"
    });

    // 同時做其他事情
    println!("主任務繼續執行...");

    // 等待任務完成
    let result = handle.await.unwrap();
    println!("{}", result);

    // spawn 多個任務
    let mut handles = vec![];
    for i in 0..5 {
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(50 * i as u64)).await;
            i * 10
        }));
    }

    // 等待所有任務
    for handle in handles {
        println!("結果: {}", handle.await.unwrap());
    }
}
```

### select! 巨集

```rust
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx.send(42).await.unwrap();
    });

    // select! 等待多個 Future，只執行第一個完成的
    tokio::select! {
        result = rx.recv() => {
            println!("收到訊息: {:?}", result);
        }
        _ = sleep(Duration::from_secs(1)) => {
            println!("超時");
        }
    }

    // 帶迴圈的 select
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    let mut count = 0;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                count += 1;
                println!("tick {}", count);
                if count >= 3 {
                    break;
                }
            }
        }
    }
}
```

### 任務取消

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        loop {
            println!("工作中...");
            sleep(Duration::from_millis(100)).await;
        }
    });

    // 讓任務執行一段時間
    sleep(Duration::from_millis(350)).await;

    // 取消任務
    handle.abort();

    // 等待任務（會得到 JoinError）
    match handle.await {
        Ok(_) => println!("任務正常完成"),
        Err(e) if e.is_cancelled() => println!("任務被取消"),
        Err(e) => println!("任務失敗: {}", e),
    }
}
```

### Tokio Channel

```rust
use tokio::sync::{mpsc, oneshot, broadcast, watch};

#[tokio::main]
async fn main() {
    // mpsc：多生產者，單消費者
    let (tx, mut rx) = mpsc::channel::<i32>(32);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(value) = rx.recv().await {
        println!("mpsc: {}", value);
    }

    // oneshot：一次性通道
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("oneshot 訊息").unwrap();
    });

    println!("oneshot: {}", rx.await.unwrap());

    // broadcast：廣播通道
    let (tx, mut rx1) = broadcast::channel::<i32>(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(value) = rx1.recv().await {
            println!("rx1: {}", value);
        }
    });

    tokio::spawn(async move {
        while let Ok(value) = rx2.recv().await {
            println!("rx2: {}", value);
        }
    });

    tx.send(1).unwrap();
    tx.send(2).unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // watch：監視通道（只保留最新值）
    let (tx, mut rx) = watch::channel("初始值");

    tokio::spawn(async move {
        loop {
            rx.changed().await.unwrap();
            println!("watch: {}", *rx.borrow());
        }
    });

    tx.send("新值 1").unwrap();
    tx.send("新值 2").unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}
```

---

## 17.4 非同步 I/O

### 非同步檔案操作

```rust
use tokio::fs::{self, File};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 寫入檔案
    let mut file = File::create("test.txt").await?;
    file.write_all(b"Hello, async!").await?;

    // 讀取檔案
    let mut file = File::open("test.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("內容: {}", contents);

    // 使用便捷函式
    fs::write("test2.txt", "便捷寫入").await?;
    let data = fs::read_to_string("test2.txt").await?;
    println!("data: {}", data);

    // 讀取目錄
    let mut entries = fs::read_dir(".").await?;
    while let Some(entry) = entries.next_entry().await? {
        println!("檔案: {:?}", entry.file_name());
    }

    // 清理
    fs::remove_file("test.txt").await?;
    fs::remove_file("test2.txt").await?;

    Ok(())
}
```

### 非同步網路

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP 伺服器
async fn server() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("伺服器監聽中...");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("新連線: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return, // 連線關閉
                    Ok(n) => n,
                    Err(_) => return,
                };

                // Echo 回去
                if socket.write_all(&buf[0..n]).await.is_err() {
                    return;
                }
            }
        });
    }
}

// TCP 客戶端
async fn client() -> tokio::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    stream.write_all(b"Hello, server!").await?;

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("收到: {}", String::from_utf8_lossy(&buf[0..n]));

    Ok(())
}

#[tokio::main]
async fn main() {
    // 啟動伺服器（在背景）
    tokio::spawn(async {
        if let Err(e) = server().await {
            eprintln!("伺服器錯誤: {}", e);
        }
    });

    // 等待伺服器啟動
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // 執行客戶端
    if let Err(e) = client().await {
        eprintln!("客戶端錯誤: {}", e);
    }
}
```

### AsyncRead 與 AsyncWrite

```rust
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 使用 BufReader 逐行讀取
    let file = File::open("Cargo.toml").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }

    // 使用 copy 複製資料
    let mut source = File::open("Cargo.toml").await?;
    let mut dest = File::create("Cargo.toml.bak").await?;
    io::copy(&mut source, &mut dest).await?;

    // 清理
    tokio::fs::remove_file("Cargo.toml.bak").await?;

    Ok(())
}
```

---

## 17.5 非同步模式

### join! 並行執行

```rust
use tokio::time::{sleep, Duration};

async fn task1() -> i32 {
    sleep(Duration::from_millis(100)).await;
    1
}

async fn task2() -> i32 {
    sleep(Duration::from_millis(200)).await;
    2
}

async fn task3() -> i32 {
    sleep(Duration::from_millis(150)).await;
    3
}

#[tokio::main]
async fn main() {
    // 順序執行（約 450ms）
    let start = std::time::Instant::now();
    let a = task1().await;
    let b = task2().await;
    let c = task3().await;
    println!("順序: {}ms", start.elapsed().as_millis());

    // 並行執行（約 200ms）
    let start = std::time::Instant::now();
    let (a, b, c) = tokio::join!(task1(), task2(), task3());
    println!("並行: {}ms", start.elapsed().as_millis());
    println!("結果: {}, {}, {}", a, b, c);

    // try_join!：任一失敗就回傳 Err
    async fn may_fail(success: bool) -> Result<i32, &'static str> {
        if success {
            Ok(42)
        } else {
            Err("失敗")
        }
    }

    match tokio::try_join!(may_fail(true), may_fail(true)) {
        Ok((a, b)) => println!("成功: {}, {}", a, b),
        Err(e) => println!("錯誤: {}", e),
    }
}
```

### 並行限制（Semaphore）

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 最多同時執行 3 個任務
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handles = vec![];

    for i in 0..10 {
        let sem = Arc::clone(&semaphore);
        handles.push(tokio::spawn(async move {
            // 取得許可
            let _permit = sem.acquire().await.unwrap();
            println!("任務 {} 開始", i);
            sleep(Duration::from_millis(100)).await;
            println!("任務 {} 結束", i);
            // permit 離開作用域時自動釋放
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
```

### 非同步串流（Stream）

```rust
use tokio_stream::{self as stream, StreamExt};
use tokio::time::{sleep, Duration};

// Cargo.toml:
// tokio-stream = "0.1"

#[tokio::main]
async fn main() {
    // 從迭代器建立串流
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);

    while let Some(value) = stream.next().await {
        println!("值: {}", value);
    }

    // 使用適配器
    let stream = stream::iter(1..=10);
    let sum: i32 = stream
        .filter(|x| futures::future::ready(x % 2 == 0))
        .map(|x| x * 2)
        .fold(0, |acc, x| async move { acc + x })
        .await;
    println!("總和: {}", sum);

    // 間隔串流
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    for _ in 0..3 {
        interval.tick().await;
        println!("tick");
    }
}
```

### 非同步迭代

```rust
use tokio::time::{interval, Duration};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    // 將 interval 轉換為 Stream
    let stream = IntervalStream::new(interval(Duration::from_millis(100)));

    // 只取前 5 個
    let mut stream = stream.take(5);

    while let Some(_) = stream.next().await {
        println!("處理中...");
    }

    println!("完成");
}
```

### 超時處理

```rust
use tokio::time::{timeout, Duration};

async fn slow_task() -> i32 {
    tokio::time::sleep(Duration::from_secs(5)).await;
    42
}

#[tokio::main]
async fn main() {
    // 使用 timeout
    match timeout(Duration::from_secs(1), slow_task()).await {
        Ok(result) => println!("結果: {}", result),
        Err(_) => println!("超時！"),
    }

    // 使用 select! 實作超時
    tokio::select! {
        result = slow_task() => {
            println!("結果: {}", result);
        }
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
            println!("超時！");
        }
    }
}
```

---

## 實際應用範例

### HTTP 客戶端（reqwest）

```rust
// Cargo.toml:
// reqwest = { version = "0.11", features = ["json"] }
// serde = { version = "1", features = ["derive"] }

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Post {
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 簡單 GET 請求
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("狀態: {}", response.status());

    // JSON 反序列化
    let post: Post = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json()
        .await?;
    println!("標題: {}", post.title);

    // 並行請求
    let urls = vec![
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];

    let mut handles = vec![];
    for url in urls {
        handles.push(tokio::spawn(async move {
            reqwest::get(url).await?.json::<Post>().await
        }));
    }

    for handle in handles {
        match handle.await? {
            Ok(post) => println!("取得: {}", post.title),
            Err(e) => println!("錯誤: {}", e),
        }
    }

    Ok(())
}
```

### 簡單 HTTP 伺服器（axum）

```rust
// Cargo.toml:
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }

use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        username: payload.username,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("伺服器運行在 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

---

## 練習題

### 練習 1：並行下載

實作一個並行下載多個 URL 的函式：

```rust
async fn download_all(urls: Vec<&str>) -> Vec<Result<String, reqwest::Error>> {
    // 並行下載所有 URL
    // 限制同時最多 5 個請求
    todo!()
}
```

### 練習 2：非同步聊天室

使用 Tokio 實作簡單的聊天室伺服器：

```rust
// 伺服器接收訊息並廣播給所有連線的客戶端
async fn chat_server(addr: &str) -> tokio::io::Result<()> {
    todo!()
}
```

### 練習 3：速率限制器

實作一個速率限制器：

```rust
struct RateLimiter {
    // 每秒最多 N 個請求
}

impl RateLimiter {
    fn new(requests_per_second: u32) -> Self { todo!() }
    async fn acquire(&self) { todo!() }
}
```

---

## 本章小結

- **async/await**：Rust 的非同步語法
- **Future**：代表非同步計算的 trait
- **執行時**：Tokio、async-std 等提供實際執行能力
- **spawn**：建立非同步任務
- **select!**：等待多個 Future，執行第一個完成的
- **join!**：並行執行多個 Future
- **非同步 I/O**：檔案、網路操作的非同步版本
- **Stream**：非同步版本的迭代器

---

## 延伸閱讀

- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [async-std Book](https://book.async.rs/)
- [Pin and Unpin](https://doc.rust-lang.org/std/pin/index.html)
