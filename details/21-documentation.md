# 第 21 章：文件與註解

> 好的文件是優秀函式庫的標誌，Rust 提供了強大的文件工具來幫助你撰寫清晰的 API 文件

---

## 21.1 註解類型

Rust 支援多種註解類型，各有不同用途。

### 單行註解

```rust
// 這是單行註解
let x = 5; // 行尾註解

fn main() {
    // 說明接下來的程式碼
    let name = "Rust";

    // TODO: 待完成的項目
    // FIXME: 需要修復的問題
    // NOTE: 重要說明
    // HACK: 臨時解決方案
    // XXX: 需要注意的地方
}
```

### 區塊註解

```rust
/* 這是區塊註解 */

/*
 * 多行區塊註解
 * 可以跨越多行
 */

fn example() {
    /* 區塊註解可以
       在程式碼中間使用 */
    let x = /* 也可以這樣 */ 5;
}

/* 區塊註解可以巢狀
   /* 像這樣 */
   這在暫時註解掉大段程式碼時很有用
*/
```

### 文件註解 ///

```rust
/// 計算兩個數字的和。
///
/// 這個函式接受兩個整數並回傳它們的和。
///
/// # 參數
///
/// * `a` - 第一個加數
/// * `b` - 第二個加數
///
/// # 回傳值
///
/// 回傳 `a` 和 `b` 的和。
///
/// # 範例
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 模組文件 //!

```rust
//! # My Crate
//!
//! `my_crate` 是一個用於數學運算的函式庫。
//!
//! ## 功能
//!
//! - 基本算術運算
//! - 進階數學函式
//! - 統計功能
//!
//! ## 範例
//!
//! ```rust
//! use my_crate::prelude::*;
//!
//! let sum = add(2, 3);
//! let product = multiply(4, 5);
//! ```

pub mod math;
pub mod stats;
```

---

## 21.2 rustdoc

rustdoc 是 Rust 的文件產生工具，支援 Markdown 語法。

### Markdown 支援

```rust
/// # 標題
///
/// ## 次標題
///
/// ### 更小的標題
///
/// 一般段落文字，可以包含 **粗體**、*斜體* 和 `程式碼`。
///
/// 項目列表：
/// - 項目一
/// - 項目二
///   - 子項目
/// - 項目三
///
/// 編號列表：
/// 1. 第一步
/// 2. 第二步
/// 3. 第三步
///
/// > 這是引用區塊
/// > 可以跨多行
///
/// ---
///
/// 水平線上方
///
/// 連結：[Rust 官網](https://www.rust-lang.org)
///
/// 表格：
///
/// | 欄位 1 | 欄位 2 | 欄位 3 |
/// |--------|--------|--------|
/// | 值 1   | 值 2   | 值 3   |
/// | 值 4   | 值 5   | 值 6   |
pub fn documented_function() {}
```

### 程式碼範例

```rust
/// 解析設定檔。
///
/// # 範例
///
/// 基本用法：
///
/// ```
/// use my_crate::Config;
///
/// let config = Config::from_str("key=value")?;
/// assert_eq!(config.get("key"), Some("value"));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// 處理錯誤：
///
/// ```
/// use my_crate::Config;
///
/// let result = Config::from_str("invalid");
/// assert!(result.is_err());
/// ```
///
/// 完整設定範例：
///
/// ```rust
/// use my_crate::Config;
///
/// let config_str = r#"
/// database_url = "postgres://localhost/mydb"
/// port = 8080
/// debug = true
/// "#;
///
/// let config = Config::from_str(config_str)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Config {
    // ...
}
```

### 跨引用連結

```rust
/// 建立新的 [`Connection`]。
///
/// 這個函式使用 [`Config`] 來建立連線。
/// 詳見 [`crate::config`] 模組的說明。
///
/// # 相關項目
///
/// - [`Connection::close`] - 關閉連線
/// - [`ConnectionPool`] - 連線池
/// - [`super::utils::retry`] - 重試邏輯
pub fn connect(config: &Config) -> Connection {
    todo!()
}

/// 連線物件。
///
/// 使用 [`connect`] 函式建立。
pub struct Connection {
    // ...
}

impl Connection {
    /// 關閉連線。
    ///
    /// 參見 [`Self::is_connected`] 檢查連線狀態。
    pub fn close(&mut self) {
        // ...
    }

    /// 檢查連線是否仍然有效。
    pub fn is_connected(&self) -> bool {
        todo!()
    }
}
```

### 屬性標記

```rust
/// 這個函式在非同步環境中使用。
///
/// # 範例
///
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// let result = my_crate::async_operation().await;
/// # }
/// ```
pub async fn async_operation() -> i32 {
    42
}

/// 這個函式會 panic。
///
/// # Panics
///
/// 當 `n` 為零時會 panic。
///
/// # Safety
///
/// 這是一個 unsafe 函式，呼叫者必須確保：
/// - 指標有效
/// - 記憶體已正確對齊
///
/// # Errors
///
/// 回傳 `Err` 當：
/// - 檔案不存在
/// - 權限不足
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let result = my_crate::divide(10, 2)?;
/// assert_eq!(result, 5);
/// # Ok(())
/// # }
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("除以零")
    } else {
        Ok(a / b)
    }
}
```

---

## 21.3 文件最佳實踐

### 公開 API 文件

```rust
//! # HTTP Client Library
//!
//! 這個函式庫提供簡單易用的 HTTP 客戶端功能。
//!
//! ## 快速開始
//!
//! ```no_run
//! use http_client::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!     let response = client.get("https://example.com").await?;
//!     println!("狀態碼: {}", response.status());
//!     Ok(())
//! }
//! ```
//!
//! ## 功能特性
//!
//! - 同步和非同步 API
//! - 自動重試
//! - 連線池
//! - Cookie 管理
//!
//! ## Cargo Features
//!
//! - `async` - 啟用非同步支援（預設）
//! - `cookies` - 啟用 Cookie 管理
//! - `gzip` - 啟用 Gzip 壓縮

/// HTTP 客戶端。
///
/// `Client` 是這個函式庫的主要入口點。它處理連線池、
/// Cookie 管理和請求重試。
///
/// # 範例
///
/// 建立客戶端並發送請求：
///
/// ```no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use http_client::{Client, Method};
///
/// let client = Client::builder()
///     .timeout(std::time::Duration::from_secs(30))
///     .build()?;
///
/// let response = client
///     .request(Method::GET, "https://api.example.com/data")
///     .header("Authorization", "Bearer token")
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// # 效能考量
///
/// `Client` 內部使用連線池，建議在應用程式生命週期中
/// 重複使用同一個 `Client` 實例，而不是為每個請求建立新的。
pub struct Client {
    // ...
}

impl Client {
    /// 使用預設設定建立新的客戶端。
    ///
    /// # 範例
    ///
    /// ```
    /// use http_client::Client;
    ///
    /// let client = Client::new();
    /// ```
    pub fn new() -> Self {
        todo!()
    }

    /// 建立客戶端建構器。
    ///
    /// 使用建構器可以自訂客戶端設定。
    ///
    /// # 範例
    ///
    /// ```
    /// use http_client::Client;
    /// use std::time::Duration;
    ///
    /// let client = Client::builder()
    ///     .timeout(Duration::from_secs(30))
    ///     .max_redirects(10)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ClientBuilder {
        todo!()
    }
}
```

### 範例程式碼

```rust
/// 從檔案讀取設定。
///
/// # 範例
///
/// ## 基本用法
///
/// ```no_run
/// use my_config::load_config;
///
/// let config = load_config("config.toml")?;
/// println!("資料庫: {}", config.database_url);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## 使用環境變數覆寫
///
/// ```no_run
/// use my_config::{load_config, ConfigOverrides};
///
/// std::env::set_var("DATABASE_URL", "postgres://custom/db");
///
/// let overrides = ConfigOverrides::from_env();
/// let config = load_config("config.toml")?
///     .with_overrides(overrides);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## 驗證設定
///
/// ```no_run
/// use my_config::load_config;
///
/// let config = load_config("config.toml")?;
///
/// // 驗證設定值
/// config.validate()?;
///
/// // 或者使用驗證載入
/// let validated = load_config("config.toml")?
///     .validate()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    todo!()
}
```

### 錯誤說明

```rust
/// 設定載入錯誤。
///
/// 這個列舉表示載入設定時可能發生的各種錯誤。
///
/// # 變體
///
/// - [`ConfigError::FileNotFound`] - 設定檔不存在
/// - [`ConfigError::ParseError`] - 設定檔格式錯誤
/// - [`ConfigError::ValidationError`] - 設定值無效
///
/// # 範例
///
/// ```
/// use my_config::{load_config, ConfigError};
///
/// match load_config("missing.toml") {
///     Ok(config) => println!("載入成功"),
///     Err(ConfigError::FileNotFound(path)) => {
///         eprintln!("找不到設定檔: {}", path);
///     }
///     Err(e) => eprintln!("其他錯誤: {}", e),
/// }
/// ```
#[derive(Debug)]
pub enum ConfigError {
    /// 設定檔不存在。
    ///
    /// 包含找不到的檔案路徑。
    FileNotFound(String),

    /// 設定檔格式錯誤。
    ///
    /// 包含解析錯誤的詳細資訊。
    ParseError {
        /// 錯誤發生的行號
        line: usize,
        /// 錯誤描述
        message: String,
    },

    /// 設定值驗證失敗。
    ///
    /// 包含驗證失敗的欄位和原因。
    ValidationError {
        /// 驗證失敗的欄位名稱
        field: String,
        /// 失敗原因
        reason: String,
    },
}
```

---

## 產生文件

### cargo doc 命令

```bash
# 產生文件
cargo doc

# 產生並開啟文件
cargo doc --open

# 包含私有項目
cargo doc --document-private-items

# 不包含依賴
cargo doc --no-deps

# 產生 JSON 格式
cargo doc --output-format json
```

### 設定 rustdoc

```rust
// lib.rs
#![doc(html_logo_url = "https://example.com/logo.png")]
#![doc(html_favicon_url = "https://example.com/favicon.ico")]
#![doc(html_root_url = "https://docs.rs/my_crate/0.1.0")]
#![doc(issue_tracker_base_url = "https://github.com/user/repo/issues/")]

// 隱藏某些項目
#![doc(hidden)]

// 條件性文件
#![cfg_attr(docsrs, feature(doc_cfg))]
```

### docs.rs 配置

```toml
# Cargo.toml
[package.metadata.docs.rs]
all-features = true
targets = ["x86_64-unknown-linux-gnu"]
rustdoc-args = ["--cfg", "docsrs"]
```

---

## 練習題

### 練習 1：撰寫完整文件

為以下 API 撰寫完整的文件：

```rust
pub struct Database {
    // ...
}

impl Database {
    pub fn connect(url: &str) -> Result<Self, DbError>;
    pub fn query(&self, sql: &str) -> Result<Vec<Row>, DbError>;
    pub fn execute(&self, sql: &str) -> Result<usize, DbError>;
    pub fn close(self);
}
```

### 練習 2：模組文件

為一個數學函式庫的各個模組撰寫文件：
- `arithmetic` - 基本算術
- `statistics` - 統計函式
- `linear_algebra` - 線性代數

### 練習 3：錯誤文件

為錯誤類型撰寫詳細的文件，包括：
- 每個變體的說明
- 如何處理各種錯誤
- 程式碼範例

---

## 本章小結

- **註解類型**：單行 `//`、區塊 `/* */`、文件 `///`、模組 `//!`
- **Markdown**：rustdoc 完整支援 Markdown
- **跨引用**：使用 `[`Name`]` 連結到其他項目
- **最佳實踐**：範例、錯誤說明、效能注意事項

---

## 延伸閱讀

- [The rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [Rust API Guidelines - Documentation](https://rust-lang.github.io/api-guidelines/documentation.html)
- [docs.rs](https://docs.rs/)
