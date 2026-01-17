# 附錄 A：常用 Crate 推薦

> 精選的 Rust 生態系 crate，涵蓋各種常見開發需求

---

## 錯誤處理

| Crate | 說明 | 使用場景 |
|-------|------|----------|
| **thiserror** | 自訂錯誤類型的 derive 巨集 | 函式庫開發 |
| **anyhow** | 簡化的錯誤處理 | 應用程式開發 |
| **eyre** | anyhow 的替代品，更好的錯誤報告 | CLI 應用程式 |
| **miette** | 精美的診斷錯誤報告 | 開發者工具 |

```toml
[dependencies]
thiserror = "1"
anyhow = "1"
```

```rust
// thiserror - 用於函式庫
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {msg}")]
    Parse { msg: String },
}

// anyhow - 用於應用程式
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    Ok(())
}
```

---

## 序列化

| Crate | 說明 | 格式 |
|-------|------|------|
| **serde** | 序列化框架 | 通用 |
| **serde_json** | JSON 支援 | JSON |
| **toml** | TOML 支援 | TOML |
| **serde_yaml** | YAML 支援 | YAML |
| **bincode** | 二進位格式 | Binary |
| **rmp-serde** | MessagePack | Binary |

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
```

---

## HTTP 與網路

| Crate | 說明 | 類型 |
|-------|------|------|
| **reqwest** | HTTP 客戶端 | Client |
| **axum** | Web 框架 | Server |
| **actix-web** | 高效能 Web 框架 | Server |
| **hyper** | 低階 HTTP 實作 | Both |
| **tower** | 中介軟體抽象 | Middleware |
| **tonic** | gRPC 框架 | RPC |

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

---

## 非同步執行時

| Crate | 說明 | 特點 |
|-------|------|------|
| **tokio** | 最流行的非同步執行時 | 功能豐富 |
| **async-std** | 標準庫風格的非同步 | 易學習 |
| **smol** | 輕量級執行時 | 簡單 |

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
# 或
async-std = { version = "1", features = ["attributes"] }
```

---

## 命令列工具

| Crate | 說明 | 使用場景 |
|-------|------|----------|
| **clap** | 參數解析 | CLI 應用程式 |
| **colored** | 彩色輸出 | 終端機輸出 |
| **indicatif** | 進度條 | 長時間操作 |
| **dialoguer** | 互動式提示 | 使用者輸入 |
| **console** | 終端機工具 | 終端機操作 |
| **termion** | 終端機控制 | TUI |
| **crossterm** | 跨平台終端機 | TUI |
| **ratatui** | TUI 框架 | TUI 應用程式 |

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
colored = "2"
indicatif = "0.17"
dialoguer = "0.11"
```

---

## 日誌與追蹤

| Crate | 說明 | 使用場景 |
|-------|------|----------|
| **tracing** | 結構化日誌 | 現代應用程式 |
| **tracing-subscriber** | tracing 的訂閱者 | 日誌輸出 |
| **log** | 日誌 facade | 函式庫 |
| **env_logger** | 環境變數配置 | 簡單日誌 |
| **fern** | 靈活的日誌配置 | 複雜日誌需求 |

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

```rust
use tracing::{info, warn, error, instrument};
use tracing_subscriber;

#[instrument]
fn process_data(id: u64) {
    info!("Processing data");
    // ...
}

fn main() {
    tracing_subscriber::init();
    process_data(42);
}
```

---

## 日期與時間

| Crate | 說明 | 特點 |
|-------|------|------|
| **chrono** | 日期時間處理 | 功能完整 |
| **time** | 輕量級時間處理 | 較小體積 |

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
```

```rust
use chrono::{DateTime, Utc, Local, Duration};

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    let tomorrow = now + Duration::days(1);

    println!("Now: {}", now.format("%Y-%m-%d %H:%M:%S"));
}
```

---

## 資料庫

| Crate | 說明 | 類型 |
|-------|------|------|
| **sqlx** | 非同步 SQL | SQL |
| **diesel** | ORM | SQL |
| **sea-orm** | 非同步 ORM | SQL |
| **redis** | Redis 客戶端 | NoSQL |
| **mongodb** | MongoDB 客戶端 | NoSQL |

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
```

---

## 正規表達式

| Crate | 說明 |
|-------|------|
| **regex** | 標準正規表達式 |
| **fancy-regex** | 支援回溯的正規表達式 |
| **once_cell** / **lazy_static** | 靜態正規表達式 |

```toml
[dependencies]
regex = "1"
once_cell = "1"
```

```rust
use regex::Regex;
use once_cell::sync::Lazy;

static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

fn is_valid_email(email: &str) -> bool {
    EMAIL_RE.is_match(email)
}
```

---

## 隨機數

| Crate | 說明 |
|-------|------|
| **rand** | 隨機數生成 |
| **uuid** | UUID 生成 |
| **nanoid** | 短 ID 生成 |

```toml
[dependencies]
rand = "0.8"
uuid = { version = "1", features = ["v4"] }
```

```rust
use rand::Rng;
use uuid::Uuid;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=100);

    let id = Uuid::new_v4();
    println!("Random: {}, UUID: {}", n, id);
}
```

---

## 測試

| Crate | 說明 | 使用場景 |
|-------|------|----------|
| **criterion** | 基準測試 | 效能測試 |
| **proptest** | 屬性測試 | 生成測試 |
| **mockall** | Mock 框架 | 單元測試 |
| **rstest** | 參數化測試 | 測試組織 |
| **assert_cmd** | CLI 測試 | 命令列測試 |
| **tempfile** | 暫存檔案 | 測試輔助 |

```toml
[dev-dependencies]
criterion = "0.5"
proptest = "1"
mockall = "0.12"
tempfile = "3"
```

---

## 並行處理

| Crate | 說明 |
|-------|------|
| **rayon** | 資料並行 |
| **crossbeam** | 並行工具 |
| **parking_lot** | 更快的同步原語 |
| **dashmap** | 並行 HashMap |

```toml
[dependencies]
rayon = "1"
crossbeam = "0.8"
parking_lot = "0.12"
dashmap = "5"
```

```rust
use rayon::prelude::*;

fn main() {
    let sum: i64 = (1..=1_000_000)
        .into_par_iter()
        .map(|x| x * x)
        .sum();

    println!("Sum: {}", sum);
}
```

---

## 其他實用工具

| Crate | 說明 |
|-------|------|
| **itertools** | 迭代器擴充 |
| **num** | 數值處理 |
| **bytes** | 位元組處理 |
| **base64** | Base64 編解碼 |
| **hex** | 十六進位編解碼 |
| **sha2** / **md5** | 雜湊函式 |
| **ring** | 加密函式庫 |
| **url** | URL 解析 |
| **walkdir** | 遞迴目錄走訪 |
| **glob** | Glob 模式 |
| **notify** | 檔案系統監視 |
| **image** | 圖片處理 |

```toml
[dependencies]
itertools = "0.12"
bytes = "1"
base64 = "0.21"
walkdir = "2"
```

---

## 推薦的 Cargo.toml 範本

### CLI 應用程式

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
colored = "2"
indicatif = "0.17"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

### Web 服務

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio-test = "0.4"
```

### 函式庫

```toml
[dependencies]
thiserror = "1"
serde = { version = "1", features = ["derive"], optional = true }

[features]
default = []
serde = ["dep:serde"]

[dev-dependencies]
criterion = "0.5"
proptest = "1"
```

---

## 延伸資源

- [crates.io](https://crates.io/) - Rust 套件中心
- [lib.rs](https://lib.rs/) - crates.io 的替代前端
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) - 精選 Rust 資源列表
- [blessed.rs](https://blessed.rs/) - 非官方推薦 crate 列表
