# 第七章：模組與套件管理

## 模組系統概念

| JavaScript 概念 | Rust 對應 |
|----------------|----------|
| `package.json` | `Cargo.toml` |
| `node_modules/` | `~/.cargo/registry/` |
| npm | Cargo + crates.io |
| `import/export` | `mod/use/pub` |
| ES Modules | Rust 模組系統 |

## 模組（mod）

### 基本模組定義

```javascript
// JavaScript (math.js)
export function add(a, b) {
    return a + b;
}

// main.js
import { add } from './math.js';
```

```rust
// Rust - 方法 1：在同一檔案內定義模組
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = math::add(1, 2);
}
```

### 檔案即模組

```
src/
├── main.rs
└── math.rs
```

```rust
// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// src/main.rs
mod math;  // 載入 math.rs

fn main() {
    let result = math::add(1, 2);
}
```

### 目錄模組

```
src/
├── main.rs
└── math/
    ├── mod.rs      // 或 math.rs（Rust 2018+）
    ├── basic.rs
    └── advanced.rs
```

```rust
// src/math/mod.rs
pub mod basic;
pub mod advanced;

// src/math/basic.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// src/main.rs
mod math;

fn main() {
    let result = math::basic::add(1, 2);
}
```

## 可見性（pub）

```rust
mod outer {
    pub mod inner {
        pub fn public_function() {}
        fn private_function() {}  // 只在 inner 內可見

        pub struct PublicStruct {
            pub public_field: i32,
            private_field: i32,  // 欄位預設私有！
        }
    }

    fn use_inner() {
        inner::public_function();   // OK
        // inner::private_function();  // 錯誤！
    }
}
```

### pub 的變體

```rust
mod my_mod {
    pub fn public_everywhere() {}      // 完全公開
    pub(crate) fn public_in_crate() {} // 只在此 crate 內公開
    pub(super) fn public_in_parent() {} // 只對父模組公開
    pub(in crate::some_mod) fn limited() {} // 指定範圍
}
```

## use 關鍵字

```rust
// 完整路徑
std::collections::HashMap::new();

// 使用 use 簡化
use std::collections::HashMap;
let map = HashMap::new();

// 重新命名
use std::collections::HashMap as Map;
let map = Map::new();

// 匯入多個
use std::collections::{HashMap, HashSet, BTreeMap};

// 匯入全部（慎用）
use std::collections::*;

// 巢狀匯入
use std::{
    collections::HashMap,
    io::{self, Read, Write},
};
```

### 慣例：什麼時候 use 到哪層？

```rust
// 函式：use 到父模組
use std::collections::HashMap;
let map = HashMap::new();  // 清楚知道 HashMap 來自哪裡

// 結構體/列舉：use 到型別本身
use std::io::Result;
fn read() -> Result<()> { Ok(()) }

// 同名衝突：用 as 或保留父模組
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
```

## Cargo.toml：專案設定

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A sample project"
license = "MIT"

[dependencies]
serde = "1.0"                           # 從 crates.io
serde_json = { version = "1.0", features = ["preserve_order"] }
tokio = { version = "1", features = ["full"] }
my_local_lib = { path = "../my_lib" }   # 本地路徑
git_lib = { git = "https://github.com/user/repo" }  # Git

[dev-dependencies]
# 只在測試時使用
criterion = "0.5"

[build-dependencies]
# 建置時使用
cc = "1.0"
```

### 版本語法

```toml
[dependencies]
# 等同於 "^1.0.0"：1.0.0 <= version < 2.0.0
serde = "1.0"

# 精確版本
exact = "=1.2.3"

# 範圍
range = ">=1.0, <2.0"

# 通配符
wildcard = "1.*"

# 最小版本
caret = "^1.2.3"  # 預設行為
tilde = "~1.2.3"  # 1.2.3 <= version < 1.3.0
```

## crates.io：Rust 的 npm

### 搜尋套件

```bash
# 命令列搜尋
cargo search serde

# 或直接到 https://crates.io 搜尋
```

### 新增依賴

```bash
# 方法 1：手動編輯 Cargo.toml

# 方法 2：使用 cargo add（需要 cargo-edit）
cargo install cargo-edit
cargo add serde
cargo add serde --features derive
cargo add tokio -F full  # -F 是 --features 的縮寫
```

### 更新依賴

```bash
cargo update          # 更新所有依賴
cargo update serde    # 只更新 serde
```

## 常用 Crate 推薦

### 必備工具

| 用途 | Crate | 說明 |
|------|-------|------|
| 序列化 | `serde`, `serde_json` | JSON/YAML 等格式處理 |
| 非同步 | `tokio` | 非同步執行時 |
| HTTP 客戶端 | `reqwest` | 類似 axios |
| HTTP 伺服器 | `axum`, `actix-web` | Web 框架 |
| CLI | `clap` | 命令列參數解析 |
| 日誌 | `tracing`, `log` | 記錄日誌 |
| 錯誤處理 | `anyhow`, `thiserror` | 簡化錯誤處理 |
| 日期時間 | `chrono` | 日期時間處理 |
| 正則表達式 | `regex` | 正則表達式 |
| 隨機數 | `rand` | 隨機數生成 |

### 類比 JavaScript 生態

| JavaScript | Rust |
|------------|------|
| axios | reqwest |
| express | axum, actix-web |
| commander | clap |
| lodash | itertools |
| moment/dayjs | chrono |
| uuid | uuid |
| dotenv | dotenvy |
| jest | 內建測試 + criterion |

## 專案結構最佳實踐

### 小型專案

```
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

### 中型專案

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs        # 程式進入點
    ├── lib.rs         # 程式庫（可選）
    ├── config.rs      # 設定模組
    ├── error.rs       # 錯誤定義
    └── models/
        ├── mod.rs
        ├── user.rs
        └── product.rs
```

### 大型專案（Workspace）

```
my_workspace/
├── Cargo.toml         # workspace 設定
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── api/
│   │   ├── Cargo.toml
│   │   └── src/
│   └── cli/
│       ├── Cargo.toml
│       └── src/
```

```toml
# my_workspace/Cargo.toml
[workspace]
members = [
    "crates/core",
    "crates/api",
    "crates/cli",
]
```

### lib.rs vs main.rs

```
src/
├── lib.rs    # 程式庫，可被其他專案使用
└── main.rs   # 執行檔，使用 lib.rs
```

```rust
// src/lib.rs
pub mod config;
pub mod models;

pub fn do_something() {}

// src/main.rs
use my_project::do_something;

fn main() {
    do_something();
}
```

## 重新匯出（Re-export）

```rust
// src/lib.rs
mod internal {
    pub struct InternalStruct;
}

// 重新匯出，讓外部可以用更短的路徑
pub use internal::InternalStruct;

// 外部使用
// use my_crate::InternalStruct;  // 而非 my_crate::internal::InternalStruct
```

## 實用指令整理

```bash
# 專案管理
cargo new my_project      # 建立執行檔專案
cargo new my_lib --lib    # 建立程式庫專案
cargo init               # 在現有目錄初始化

# 建置與執行
cargo build              # 建置（debug）
cargo build --release    # 建置（release）
cargo run                # 執行
cargo run --release      # 執行（release）
cargo run -- arg1 arg2   # 帶參數執行

# 檢查與測試
cargo check              # 快速檢查（不產生執行檔）
cargo test               # 執行測試
cargo test test_name     # 執行特定測試
cargo bench              # 執行基準測試

# 依賴管理
cargo update             # 更新依賴
cargo tree               # 查看依賴樹

# 程式碼品質
cargo fmt                # 格式化
cargo clippy             # Lint 檢查
cargo doc --open         # 產生並開啟文件
```

## 練習題

### 練習 1：建立模組結構
建立以下結構並讓它能編譯：

```
src/
├── main.rs
└── utils/
    ├── mod.rs
    ├── string.rs   # pub fn reverse(s: &str) -> String
    └── math.rs     # pub fn factorial(n: u32) -> u32
```

### 練習 2：使用外部 crate
1. 新增 `rand` 依賴
2. 寫一個函式產生隨機密碼

### 練習 3：重新匯出
修改練習 1，讓使用者可以這樣使用：
```rust
use my_crate::{reverse, factorial};
```
而非：
```rust
use my_crate::utils::string::reverse;
use my_crate::utils::math::factorial;
```

---

[← 上一章：集合與迭代器](./06-collections.md) | [下一章：非同步程式設計 →](./08-async.md)
