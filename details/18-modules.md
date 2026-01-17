# 第 18 章：模組系統

> Rust 的模組系統讓你能夠組織程式碼、控制可見性，並建立清晰的 API 邊界

---

## 18.1 模組基礎

模組（Module）是組織程式碼的基本單位，用於將相關的函式、結構體、trait 等分組。

### mod 關鍵字

```rust
// 在同一個檔案中定義模組
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("加入等候名單");
        }

        fn seat_at_table() {
            println!("帶位入座");
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn main() {
    // 使用絕對路徑
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相對路徑
    front_of_house::hosting::add_to_waitlist();
}
```

### 檔案即模組

Rust 支援將模組拆分到不同檔案：

```
src/
├── main.rs
├── front_of_house.rs      // 模組定義
└── front_of_house/        // 子模組目錄
    ├── hosting.rs
    └── serving.rs
```

**main.rs**：
```rust
mod front_of_house;  // 宣告模組，從 front_of_house.rs 載入

fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

**front_of_house.rs**：
```rust
pub mod hosting;  // 宣告子模組
mod serving;      // 私有子模組
```

**front_of_house/hosting.rs**：
```rust
pub fn add_to_waitlist() {
    println!("加入等候名單");
}
```

### 目錄模組

使用目錄來組織模組：

```
src/
├── main.rs
└── front_of_house/
    ├── mod.rs            // 模組根（舊風格）
    ├── hosting.rs
    └── serving.rs
```

或使用新風格（Rust 2018+）：

```
src/
├── main.rs
├── front_of_house.rs     // 模組根
└── front_of_house/
    ├── hosting.rs
    └── serving.rs
```

**front_of_house.rs**（或 mod.rs）：
```rust
pub mod hosting;
mod serving;

pub fn greet() {
    println!("歡迎光臨！");
}
```

---

## 18.2 可見性控制

Rust 預設所有項目都是私有的，需要明確使用 `pub` 關鍵字來公開。

### pub 公開

```rust
mod outer {
    pub mod inner {
        pub fn public_function() {
            println!("這是公開函式");
            private_function(); // 可以呼叫同模組的私有函式
        }

        fn private_function() {
            println!("這是私有函式");
        }
    }

    // 公開結構體
    pub struct PublicStruct {
        pub public_field: i32,
        private_field: i32,  // 私有欄位
    }

    impl PublicStruct {
        pub fn new() -> Self {
            PublicStruct {
                public_field: 0,
                private_field: 0,
            }
        }

        pub fn get_private(&self) -> i32 {
            self.private_field
        }
    }
}

fn main() {
    outer::inner::public_function();

    let s = outer::PublicStruct::new();
    println!("公開欄位: {}", s.public_field);
    // println!("{}", s.private_field); // 錯誤！私有欄位
    println!("私有欄位（透過方法）: {}", s.get_private());
}
```

### pub(crate)

限制為 crate 內可見：

```rust
mod my_module {
    // 只在當前 crate 內可見
    pub(crate) fn crate_public() {
        println!("crate 內部公開");
    }

    pub(crate) struct CratePublicStruct {
        pub(crate) field: i32,
    }
}

fn main() {
    my_module::crate_public(); // OK，同一個 crate

    let s = my_module::CratePublicStruct { field: 42 };
    println!("{}", s.field);
}

// 如果這個 crate 被其他 crate 依賴：
// 其他 crate 無法存取 crate_public() 和 CratePublicStruct
```

### pub(super)

對父模組可見：

```rust
mod parent {
    pub mod child {
        // 對父模組可見
        pub(super) fn for_parent() {
            println!("只有父模組可以呼叫");
        }

        pub fn public_fn() {
            println!("公開函式");
        }
    }

    pub fn call_child() {
        child::for_parent(); // OK
        child::public_fn();  // OK
    }
}

fn main() {
    parent::call_child();
    parent::child::public_fn();
    // parent::child::for_parent(); // 錯誤！不在父模組內
}
```

### pub(in path)

限制為特定路徑可見：

```rust
mod outer {
    pub mod inner {
        pub mod deep {
            // 只對 outer 模組可見
            pub(in crate::outer) fn restricted() {
                println!("受限函式");
            }
        }
    }

    pub fn call_restricted() {
        inner::deep::restricted(); // OK
    }
}

fn main() {
    outer::call_restricted();
    // outer::inner::deep::restricted(); // 錯誤！
}
```

### 結構體與列舉的可見性

```rust
mod my_mod {
    // 公開結構體，但欄位預設私有
    pub struct MyStruct {
        pub public_field: i32,
        private_field: String,
    }

    impl MyStruct {
        pub fn new(value: i32) -> Self {
            MyStruct {
                public_field: value,
                private_field: String::from("secret"),
            }
        }
    }

    // 公開列舉，所有變體自動公開
    pub enum MyEnum {
        Variant1,
        Variant2(i32),
        Variant3 { x: i32, y: i32 },
    }
}

fn main() {
    // 無法直接建立結構體（因為有私有欄位）
    // let s = my_mod::MyStruct { public_field: 1, private_field: String::new() };

    // 使用建構函式
    let s = my_mod::MyStruct::new(42);
    println!("{}", s.public_field);

    // 列舉變體都是公開的
    let e1 = my_mod::MyEnum::Variant1;
    let e2 = my_mod::MyEnum::Variant2(10);
    let e3 = my_mod::MyEnum::Variant3 { x: 1, y: 2 };
}
```

---

## 18.3 use 與路徑

### 絕對路徑 vs 相對路徑

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 絕對路徑：從 crate 根開始
use crate::front_of_house::hosting;

// 相對路徑：從當前位置開始
// use front_of_house::hosting;

// self：當前模組
// use self::front_of_house::hosting;

// super：父模組
mod back_of_house {
    pub fn cook() {
        // 使用 super 存取父模組的項目
        super::front_of_house::hosting::add_to_waitlist();
    }
}

fn main() {
    hosting::add_to_waitlist();
}
```

### use 重新匯出

```rust
mod my_module {
    mod internal {
        pub fn helper() {
            println!("內部輔助函式");
        }
    }

    // 重新匯出，讓外部可以存取
    pub use internal::helper;

    // 或者給予不同名稱
    pub use internal::helper as public_helper;
}

fn main() {
    my_module::helper();
    my_module::public_helper();
}
```

### as 別名

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// 重新命名匯入
use std::collections::HashMap as Map;

fn main() {
    let _map: Map<String, i32> = Map::new();
}
```

### 巢狀路徑

```rust
// 不使用巢狀路徑
use std::cmp::Ordering;
use std::io;
use std::io::Write;

// 使用巢狀路徑
use std::{cmp::Ordering, io};
use std::io::{self, Write};

// 匯入多個項目
use std::collections::{HashMap, HashSet, BTreeMap};
```

### glob import

```rust
// 匯入所有公開項目（不建議在正式程式碼中使用）
use std::collections::*;

fn main() {
    let _map: HashMap<String, i32> = HashMap::new();
    let _set: HashSet<i32> = HashSet::new();
}

// 常用於測試模組
#[cfg(test)]
mod tests {
    use super::*; // 匯入父模組的所有項目
}
```

### prelude 模式

```rust
// 建立自己的 prelude 模組
mod prelude {
    pub use crate::error::Error;
    pub use crate::result::Result;
    pub use crate::common::{Config, Context};
}

// 使用者只需要一行就能匯入常用項目
use crate::prelude::*;
```

---

## 18.4 專案組織

### 單一檔案專案

```
my_project/
├── Cargo.toml
└── src/
    └── main.rs    # 或 lib.rs
```

### 多檔案專案

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs         # 程式進入點
    ├── lib.rs          # 函式庫根（可選）
    ├── config.rs       # 設定模組
    ├── error.rs        # 錯誤處理
    └── utils/          # 工具模組目錄
        ├── mod.rs
        ├── string.rs
        └── file.rs
```

**main.rs**：
```rust
mod config;
mod error;
mod utils;

use config::Config;
use error::Result;

fn main() -> Result<()> {
    let config = Config::load()?;
    println!("載入設定: {:?}", config);
    Ok(())
}
```

**lib.rs**（如果同時是函式庫）：
```rust
pub mod config;
pub mod error;
pub mod utils;

pub use config::Config;
pub use error::{Error, Result};
```

### 二進位與函式庫混合

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs          # 函式庫
    ├── main.rs         # 預設二進位
    └── bin/            # 其他二進位
        ├── cli.rs
        └── server.rs
```

**Cargo.toml**：
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "my_project"
path = "src/main.rs"

[[bin]]
name = "cli"
path = "src/bin/cli.rs"

[[bin]]
name = "server"
path = "src/bin/server.rs"
```

### 工作空間（Workspace）

```
my_workspace/
├── Cargo.toml          # 工作空間根
├── common/             # 共享函式庫
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── client/             # 客戶端
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── server/             # 伺服器
    ├── Cargo.toml
    └── src/
        └── main.rs
```

**根 Cargo.toml**：
```toml
[workspace]
members = [
    "common",
    "client",
    "server",
]

# 共享的依賴版本
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

**子專案 Cargo.toml**：
```toml
[package]
name = "client"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { path = "../common" }
serde.workspace = true
tokio.workspace = true
```

### 常見專案結構

```
my_project/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── README.md
├── LICENSE
├── examples/           # 範例程式
│   └── basic.rs
├── benches/            # 效能測試
│   └── benchmark.rs
├── tests/              # 整合測試
│   └── integration.rs
└── src/
    ├── lib.rs          # 函式庫根
    ├── main.rs         # 二進位進入點（可選）
    ├── error.rs
    ├── config.rs
    ├── api/
    │   ├── mod.rs
    │   ├── handlers.rs
    │   └── routes.rs
    ├── models/
    │   ├── mod.rs
    │   ├── user.rs
    │   └── post.rs
    └── utils/
        ├── mod.rs
        └── helpers.rs
```

---

## 實際範例

### 完整的模組組織

**src/lib.rs**：
```rust
//! My Project Library
//!
//! 這是專案的主要文件說明。

pub mod config;
pub mod error;
pub mod api;
pub mod models;

// 重新匯出常用項目
pub use config::Config;
pub use error::{Error, Result};

// Prelude
pub mod prelude {
    pub use crate::config::Config;
    pub use crate::error::{Error, Result};
    pub use crate::models::*;
}
```

**src/error.rs**：
```rust
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Config(String),
    Io(std::io::Error),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(msg) => write!(f, "設定錯誤: {}", msg),
            Error::Io(e) => write!(f, "IO 錯誤: {}", e),
            Error::Parse(msg) => write!(f, "解析錯誤: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
```

**src/config.rs**：
```rust
use crate::error::{Error, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub debug: bool,
}

impl Config {
    pub fn load() -> Result<Self> {
        Self::from_file("config.toml")
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }

    fn parse(_content: &str) -> Result<Self> {
        // 簡化的解析邏輯
        Ok(Config {
            database_url: String::from("postgres://localhost/mydb"),
            port: 8080,
            debug: true,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: String::new(),
            port: 3000,
            debug: false,
        }
    }
}
```

**src/models/mod.rs**：
```rust
mod user;
mod post;

pub use user::User;
pub use post::Post;

// 共用的 trait
pub trait Entity {
    fn id(&self) -> u64;
    fn created_at(&self) -> &str;
}
```

**src/models/user.rs**：
```rust
use super::Entity;

#[derive(Debug, Clone)]
pub struct User {
    id: u64,
    pub username: String,
    pub email: String,
    created_at: String,
}

impl User {
    pub fn new(id: u64, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
            created_at: String::from("2024-01-01"),
        }
    }
}

impl Entity for User {
    fn id(&self) -> u64 {
        self.id
    }

    fn created_at(&self) -> &str {
        &self.created_at
    }
}
```

**src/main.rs**：
```rust
use my_project::prelude::*;

fn main() -> Result<()> {
    let config = Config::load().unwrap_or_default();
    println!("設定: {:?}", config);

    let user = User::new(1, "alice".into(), "alice@example.com".into());
    println!("使用者: {:?}", user);

    Ok(())
}
```

---

## 練習題

### 練習 1：重構模組

將以下程式碼重構為多個模組：

```rust
// 重構前：所有程式碼在 main.rs
struct User { name: String }
struct Post { title: String, author: User }
fn create_user(name: &str) -> User { todo!() }
fn create_post(title: &str, author: User) -> Post { todo!() }
fn main() { }
```

### 練習 2：設計 API 模組

為一個 REST API 設計模組結構：
- 路由處理
- 中介軟體
- 資料驗證
- 錯誤回應

### 練習 3：工作空間設計

設計一個包含以下元件的工作空間：
- core：核心邏輯
- cli：命令列工具
- web：Web 服務
- shared：共享程式碼

---

## 本章小結

- **mod 關鍵字**：定義模組，可以內嵌或分離到檔案
- **可見性**：`pub`、`pub(crate)`、`pub(super)`、`pub(in path)`
- **use 語句**：匯入項目，支援別名和重新匯出
- **路徑**：絕對路徑（`crate::`）和相對路徑
- **專案組織**：單檔案、多檔案、工作空間

---

## 延伸閱讀

- [The Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust By Example - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
- [API Guidelines - Modules](https://rust-lang.github.io/api-guidelines/naming.html)
