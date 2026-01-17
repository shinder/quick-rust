# 第 10 章：錯誤處理

> Rust 的錯誤處理系統強迫你面對錯誤，讓程式更加可靠。本章深入探討如何優雅地處理錯誤。

---

## 10.1 可恢復錯誤 vs 不可恢復錯誤

Rust 將錯誤分為兩類：

| 類型 | 機制 | 使用時機 |
|------|------|---------|
| 可恢復錯誤 | `Result<T, E>` | 預期可能發生的錯誤（檔案不存在、網路逾時） |
| 不可恢復錯誤 | `panic!` | 程式 bug、無法繼續執行的情況 |

### panic! 與程式終止

```rust
fn main() {
    // 明確呼叫 panic
    // panic!("程式崩潰！");

    // 會導致 panic 的情況
    let v = vec![1, 2, 3];
    // v[99];  // index out of bounds，會 panic

    // 除以零
    // let x = 1 / 0;  // 編譯時期錯誤（整數）
    // 浮點數不會 panic，會產生 inf 或 NaN
}
```

### panic 的行為

```rust
// 預設行為：unwinding（回溯堆疊，清理資源）
// 可以在 Cargo.toml 設定為 abort（直接終止，程式較小）

// [profile.release]
// panic = 'abort'
```

### Result 與錯誤傳遞

```rust
use std::fs::File;

fn main() {
    // 回傳 Result
    let result = File::open("hello.txt");

    let file = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("無法開啟檔案: {:?}", error);
        }
    };
}
```

### 何時使用 panic vs Result

**使用 panic 的情況：**

```rust
// 1. 範例程式碼、原型、測試
fn quick_example() {
    let value: i32 = "42".parse().unwrap();  // 快速開發
}

// 2. 程式碼邏輯確保不會失敗
fn safe_access() {
    let v = vec![1, 2, 3];
    let first = v.first().unwrap();  // 已知 v 不為空
}

// 3. 違反契約（呼叫者的錯誤）
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除數不能為零！這是呼叫者的錯誤。");
    }
    a / b
}
```

**使用 Result 的情況：**

```rust
// 1. 預期可能失敗的操作
fn read_config() -> Result<Config, ConfigError> {
    // 檔案可能不存在
    // 格式可能錯誤
}

// 2. 讓呼叫者決定如何處理
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// 3. 函式庫程式碼
pub fn connect(addr: &str) -> Result<Connection, ConnectionError> {
    // 網路可能有問題
}
```

---

## 10.2 Result 深入

### ? 運算子的工作原理

```rust
use std::fs::File;
use std::io::{self, Read};

// 使用 match
fn read_file_v1(path: &str) -> Result<String, io::Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

// 使用 ?
fn read_file_v2(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 更簡潔
fn read_file_v3(path: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

// 最簡潔
fn read_file_v4(path: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}
```

### ? 與 From trait

`?` 會自動呼叫 `From::from` 進行錯誤轉換：

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

// 實作 From trait 進行自動轉換
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

fn read_and_parse(path: &str) -> Result<i32, MyError> {
    let content = std::fs::read_to_string(path)?;  // io::Error -> MyError
    let number: i32 = content.trim().parse()?;     // ParseIntError -> MyError
    Ok(number)
}
```

### 轉換錯誤類型

```rust
use std::num::ParseIntError;

// map_err：轉換錯誤類型
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("解析 '{}' 失敗: {}", s, e))
}

// ok_or / ok_or_else：Option -> Result
fn get_value(opt: Option<i32>) -> Result<i32, &'static str> {
    opt.ok_or("沒有值")
}

// 組合多種錯誤
fn complex_operation() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("data.txt")?;
    let number: i32 = "42".parse()?;
    // Box<dyn Error> 可以容納任何錯誤
    Ok(())
}
```

### 常用 Result 方法

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("錯誤");

    // map：轉換成功值
    let doubled = ok.map(|n| n * 2);  // Ok(84)

    // map_err：轉換錯誤
    let new_err = err.map_err(|e| format!("錯誤: {}", e));

    // and：如果 Ok，回傳第二個 Result
    let and_result = ok.and(Ok(100));  // Ok(100)

    // and_then：鏈式操作
    let chain = ok.and_then(|n| {
        if n > 0 { Ok(n * 2) } else { Err("必須是正數") }
    });

    // or：如果 Err，回傳第二個 Result
    let or_result = err.or(Ok(0));  // Ok(0)

    // or_else：錯誤時執行閉包
    let recovered = err.or_else(|_| Ok(0));  // Ok(0)

    // unwrap_or_else：成功回傳值，失敗執行閉包
    let value = err.unwrap_or_else(|e| {
        println!("發生錯誤: {}", e);
        0
    });

    // inspect / inspect_err（Rust 1.65+）
    let result = ok
        .inspect(|v| println!("成功: {}", v))
        .inspect_err(|e| println!("錯誤: {}", e));

    Ok(())
}
```

---

## 10.3 自訂錯誤類型

### 基本錯誤類型

```rust
use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError {}

// 使用
fn might_fail() -> Result<(), MyError> {
    Err(MyError {
        message: String::from("發生錯誤"),
    })
}
```

### 列舉錯誤（推薦）

```rust
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    // 無資料
    NotFound,

    // 帶訊息
    InvalidInput(String),

    // 包裝其他錯誤
    Io(io::Error),
    Parse(ParseIntError),

    // 帶多個欄位
    Validation {
        field: String,
        message: String,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "找不到資源"),
            AppError::InvalidInput(msg) => write!(f, "無效輸入: {}", msg),
            AppError::Io(err) => write!(f, "IO 錯誤: {}", err),
            AppError::Parse(err) => write!(f, "解析錯誤: {}", err),
            AppError::Validation { field, message } => {
                write!(f, "欄位 '{}' 驗證失敗: {}", field, message)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Parse(err) => Some(err),
            _ => None,
        }
    }
}

// 實作 From 進行自動轉換
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}
```

### 錯誤鏈

```rust
use std::error::Error;

fn process() -> Result<(), Box<dyn Error>> {
    let inner_error = std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "底層錯誤",
    );

    // 印出錯誤鏈
    let mut err: &dyn Error = &inner_error;
    loop {
        println!("錯誤: {}", err);
        match err.source() {
            Some(source) => err = source,
            None => break,
        }
    }

    Ok(())
}
```

---

## 10.4 錯誤處理工具箱

### thiserror（函式庫錯誤）

```toml
[dependencies]
thiserror = "1.0"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("找不到資料: {0}")]
    NotFound(String),

    #[error("無效的格式: {field}")]
    InvalidFormat { field: String },

    #[error("IO 錯誤")]
    Io(#[from] std::io::Error),

    #[error("解析錯誤: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("驗證失敗")]
    Validation(#[source] ValidationError),
}

#[derive(Error, Debug)]
#[error("欄位 '{field}' 驗證失敗: {message}")]
struct ValidationError {
    field: String,
    message: String,
}

// 使用
fn load_data(path: &str) -> Result<i32, DataError> {
    let content = std::fs::read_to_string(path)?;  // 自動轉換
    let number: i32 = content.trim().parse()?;      // 自動轉換

    if number < 0 {
        return Err(DataError::Validation(ValidationError {
            field: String::from("number"),
            message: String::from("必須是正數"),
        }));
    }

    Ok(number)
}
```

### anyhow（應用程式錯誤）

```toml
[dependencies]
anyhow = "1.0"
```

```rust
use anyhow::{anyhow, bail, Context, Result};

// Result 是 anyhow::Result 的別名
fn process_file(path: &str) -> Result<i32> {
    let content = std::fs::read_to_string(path)
        .context("無法讀取檔案")?;

    let number: i32 = content.trim().parse()
        .context("無法解析數字")?;

    if number < 0 {
        bail!("數字必須是正數");  // 等同於 return Err(anyhow!(...))
    }

    Ok(number)
}

fn main() -> Result<()> {
    let result = process_file("data.txt")
        .context("處理檔案失敗")?;

    println!("結果: {}", result);
    Ok(())
}

// 錯誤訊息會是：
// Error: 處理檔案失敗
//
// Caused by:
//     0: 無法讀取檔案
//     1: No such file or directory (os error 2)
```

### thiserror vs anyhow

| 特性 | thiserror | anyhow |
|------|-----------|--------|
| 用途 | 函式庫 | 應用程式 |
| 錯誤型別 | 自訂列舉 | 動態（Box<dyn Error>） |
| 效能 | 較好 | 稍差（動態分派） |
| 易用性 | 需定義型別 | 簡單 |
| 錯誤上下文 | 需自己加 | 內建 Context |

**最佳實踐：**
- 函式庫：使用 thiserror
- 應用程式：使用 anyhow
- 兩者可以組合使用

---

## 10.5 錯誤處理模式

### 提前返回

```rust
fn process(data: &str) -> Result<i32, String> {
    if data.is_empty() {
        return Err(String::from("資料為空"));
    }

    let trimmed = data.trim();
    if trimmed.is_empty() {
        return Err(String::from("資料只有空白"));
    }

    trimmed.parse::<i32>()
        .map_err(|_| String::from("無法解析為數字"))
}
```

### 收集錯誤

```rust
fn parse_numbers(inputs: &[&str]) -> Result<Vec<i32>, Vec<String>> {
    let mut successes = Vec::new();
    let mut errors = Vec::new();

    for input in inputs {
        match input.parse::<i32>() {
            Ok(n) => successes.push(n),
            Err(_) => errors.push(format!("無法解析: {}", input)),
        }
    }

    if errors.is_empty() {
        Ok(successes)
    } else {
        Err(errors)
    }
}

// 使用 partition
fn parse_numbers_v2(inputs: &[&str]) -> (Vec<i32>, Vec<String>) {
    let (successes, errors): (Vec<_>, Vec<_>) = inputs
        .iter()
        .map(|s| s.parse::<i32>().map_err(|_| format!("無法解析: {}", s)))
        .partition(Result::is_ok);

    (
        successes.into_iter().map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    )
}
```

### 重試模式

```rust
use std::thread;
use std::time::Duration;

fn retry<T, E, F>(mut f: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut attempt = 0;
    loop {
        attempt += 1;
        match f() {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= max_attempts => return Err(e),
            Err(_) => {
                let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1));
                thread::sleep(delay);
            }
        }
    }
}

// 使用
fn flaky_operation() -> Result<String, &'static str> {
    // 可能失敗的操作
    Err("暫時失敗")
}

fn main() {
    let result = retry(flaky_operation, 3);
}
```

### 錯誤恢復

```rust
fn safe_divide(a: f64, b: f64) -> f64 {
    match divide_checked(a, b) {
        Ok(result) => result,
        Err(_) => 0.0,  // 預設值
    }
}

fn divide_checked(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("除以零")
    } else {
        Ok(a / b)
    }
}

// 使用 unwrap_or 系列方法
fn process_with_default(opt: Option<i32>) -> i32 {
    opt.unwrap_or(0)                         // 固定預設值
       // .unwrap_or_default()               // 使用 Default trait
       // .unwrap_or_else(|| expensive())    // 懶惰計算
}
```

---

## 實戰範例

### 設定檔解析

```rust
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("設定檔不存在: {0}")]
    NotFound(String),

    #[error("無法讀取設定檔")]
    ReadError(#[from] std::io::Error),

    #[error("設定格式錯誤: {0}")]
    ParseError(String),

    #[error("缺少必要欄位: {0}")]
    MissingField(String),
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    // 檢查檔案存在
    if !Path::new(path).exists() {
        return Err(ConfigError::NotFound(path.to_string()));
    }

    // 讀取檔案
    let content = fs::read_to_string(path)?;

    // 解析（簡化版）
    let mut host = None;
    let mut port = None;
    let mut debug = false;

    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "host" => host = Some(value.to_string()),
            "port" => {
                port = Some(value.parse().map_err(|_| {
                    ConfigError::ParseError(format!("無效的 port: {}", value))
                })?);
            }
            "debug" => debug = value == "true",
            _ => {}
        }
    }

    Ok(Config {
        host: host.ok_or(ConfigError::MissingField("host".to_string()))?,
        port: port.ok_or(ConfigError::MissingField("port".to_string()))?,
        debug,
    })
}
```

### HTTP 請求處理

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum RequestError {
    #[error("連線失敗: {0}")]
    ConnectionFailed(String),

    #[error("請求逾時")]
    Timeout,

    #[error("HTTP 錯誤 {status}: {message}")]
    HttpError { status: u16, message: String },

    #[error("解析回應失敗")]
    ParseError(#[from] serde_json::Error),
}

struct Response {
    status: u16,
    body: String,
}

fn fetch_data(url: &str) -> Result<serde_json::Value, RequestError> {
    // 模擬 HTTP 請求
    let response = make_request(url)?;

    // 檢查狀態碼
    if response.status >= 400 {
        return Err(RequestError::HttpError {
            status: response.status,
            message: response.body,
        });
    }

    // 解析 JSON
    let data: serde_json::Value = serde_json::from_str(&response.body)?;

    Ok(data)
}

fn make_request(_url: &str) -> Result<Response, RequestError> {
    // 模擬實作
    Ok(Response {
        status: 200,
        body: r#"{"data": "hello"}"#.to_string(),
    })
}
```

---

## 本章重點回顧

1. **錯誤分類**
   - 可恢復：Result
   - 不可恢復：panic!
   - 選擇依據：預期 vs 程式 bug

2. **Result 操作**
   - `?` 運算子傳遞錯誤
   - map、and_then 轉換
   - From trait 自動轉換

3. **自訂錯誤**
   - 實作 Display 和 Error trait
   - 列舉表示不同錯誤類型
   - thiserror 簡化定義

4. **最佳實踐**
   - 函式庫：thiserror + 自訂型別
   - 應用程式：anyhow
   - 提供有意義的錯誤訊息

---

## 練習題

### 練習 1：基本錯誤處理

```rust
// 實作安全的陣列存取
fn safe_get(arr: &[i32], index: usize) -> Result<i32, String> {
    // 你的程式碼
}
```

### 練習 2：錯誤傳遞

```rust
// 使用 ? 重構這個函式
fn process_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    let number = match content.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(number * 2)
}
```

### 練習 3：自訂錯誤

使用 thiserror 定義使用者驗證錯誤：

```rust
// 包含：EmptyUsername、InvalidEmail(String)、WeakPassword
```

### 練習 4：錯誤處理策略

實作一個函式，處理多個可能失敗的操作，並收集所有錯誤：

```rust
fn validate_user(username: &str, email: &str, password: &str)
    -> Result<User, Vec<ValidationError>>
{
    // 收集所有驗證錯誤，而非遇到第一個就返回
}
```

---

## 延伸閱讀

- [錯誤處理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)
- [thiserror 文件](https://docs.rs/thiserror)
- [anyhow 文件](https://docs.rs/anyhow)
- [Rust 錯誤處理最佳實踐](https://nick.groenen.me/posts/rust-error-handling/)

---

[← 上一章：列舉與模式匹配](./09-enums.md) | [下一章：泛型 →](./11-generics.md)

---

## 第二部分完結

恭喜你完成了 **第二部分：核心概念**！

你已經掌握了：
- 所有權系統：Rust 的核心特性
- 生命週期：確保引用有效
- 結構體：自訂資料型別
- 列舉：代數資料型別與模式匹配
- 錯誤處理：Result 與自訂錯誤

這些是 Rust 程式設計的基石。接下來的 **第三部分：進階特性** 將探討：
- 泛型
- Trait 系統
- 集合型別
- 迭代器
- 智慧指標

準備好繼續深入了嗎？
