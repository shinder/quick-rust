# 第 23 章：網路程式設計

> Rust 提供了強大的網路程式設計能力，從低階 TCP/UDP 到高階 HTTP 框架

---

## 23.1 HTTP 客戶端

### reqwest 基礎

```toml
# Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Post {
    id: i32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[derive(Debug, Serialize)]
struct NewPost {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET 請求
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?;

    println!("狀態碼: {}", response.status());

    let post: Post = response.json().await?;
    println!("標題: {}", post.title);

    // POST 請求
    let client = reqwest::Client::new();
    let new_post = NewPost {
        title: "My Post".to_string(),
        body: "Post content".to_string(),
        user_id: 1,
    };

    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;

    println!("建立結果: {}", response.status());

    Ok(())
}
```

### 進階用法

```rust
use reqwest::{Client, header};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 建立自訂客戶端
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("MyApp/1.0")
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("application/json"),
            );
            headers
        })
        .build()?;

    // 帶標頭的請求
    let response = client
        .get("https://api.example.com/data")
        .header("Authorization", "Bearer token123")
        .query(&[("page", "1"), ("limit", "10")])
        .send()
        .await?;

    // 處理回應
    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;

    println!("狀態: {}", status);
    println!("Content-Type: {:?}", headers.get("content-type"));
    println!("內容: {}", body);

    Ok(())
}
```

### 錯誤處理與重試

```rust
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

async fn fetch_with_retry(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<String, reqwest::Error> {
    let mut retries = 0;

    loop {
        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => {
                return response.text().await;
            }
            Ok(response) if response.status().is_server_error() && retries < max_retries => {
                retries += 1;
                let delay = Duration::from_secs(2u64.pow(retries));
                eprintln!("伺服器錯誤，{}秒後重試...", delay.as_secs());
                sleep(delay).await;
            }
            Ok(response) => {
                return Err(response.error_for_status().unwrap_err());
            }
            Err(e) if retries < max_retries => {
                retries += 1;
                let delay = Duration::from_secs(2u64.pow(retries));
                eprintln!("請求失敗: {}，{}秒後重試...", e, delay.as_secs());
                sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let content = fetch_with_retry(&client, "https://example.com", 3).await?;
    println!("{}", content);
    Ok(())
}
```

---

## 23.2 HTTP 伺服器

### axum 框架

```toml
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

// 資料模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    page: Option<u32>,
    limit: Option<u32>,
}

// 應用狀態
type AppState = Arc<RwLock<HashMap<u64, User>>>;

// 處理器
async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Json<Vec<User>> {
    let users = state.read().unwrap();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    let users: Vec<User> = users.values().cloned().collect();
    Json(users)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.read().unwrap();
    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let mut users = state.write().unwrap();
    let id = users.len() as u64 + 1;

    let user = User {
        id,
        name: payload.name,
        email: payload.email,
    };

    users.insert(id, user.clone());
    (StatusCode::CREATED, Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut users = state.write().unwrap();
    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    // 初始化追蹤
    tracing_subscriber::init();

    // 建立共享狀態
    let state: AppState = Arc::new(RwLock::new(HashMap::new()));

    // 建立路由
    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .with_state(state);

    // 啟動伺服器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("伺服器運行在 http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### 中介軟體

```rust
use axum::{
    Router,
    middleware::{self, Next},
    response::Response,
    http::Request,
    body::Body,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::time::Instant;

// 自訂中介軟體
async fn logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    println!("{} {} - {:?}", method, uri, duration);

    response
}

fn create_app() -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { "Hello!" }))
        .layer(middleware::from_fn(logging_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
```

---

## 23.3 其他協定

### WebSocket

```rust
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::Response,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(text)) = msg {
            println!("收到: {}", text);

            // 回傳訊息
            if socket.send(Message::Text(format!("Echo: {}", text))).await.is_err() {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

### TCP/UDP

```rust
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP 伺服器
async fn tcp_server() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("新連線: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 { return; }
                socket.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
}

// UDP 伺服器
async fn udp_server() -> tokio::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = [0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("收到 {} bytes 從 {}", len, addr);
        socket.send_to(&buf[0..len], addr).await?;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(tcp_server());
    // 或
    // udp_server().await.unwrap();
}
```

---

## 練習題

### 練習 1：REST API

建立一個完整的 TODO REST API：
- CRUD 操作
- 驗證
- 錯誤處理

### 練習 2：WebSocket 聊天室

實作一個簡單的聊天室伺服器。

### 練習 3：HTTP 代理

建立一個簡單的 HTTP 代理伺服器。

---

## 本章小結

- **HTTP 客戶端**：使用 reqwest 發送請求
- **HTTP 伺服器**：使用 axum 建立 API
- **WebSocket**：雙向即時通訊
- **TCP/UDP**：低階網路程式設計

---

## 延伸閱讀

- [Axum Documentation](https://docs.rs/axum/)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
