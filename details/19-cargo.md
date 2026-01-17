# 第 19 章：Cargo 進階

> Cargo 是 Rust 的套件管理器和建置系統，掌握進階功能可以大幅提升開發效率

---

## 19.1 Cargo.toml 詳解

### [package] 區段

```toml
[package]
name = "my-project"              # 套件名稱（必須）
version = "0.1.0"                # 版本號，遵循 SemVer
edition = "2021"                 # Rust 版本（2015、2018、2021）
authors = ["Your Name <you@example.com>"]
description = "A brief description"
documentation = "https://docs.rs/my-project"
homepage = "https://example.com"
repository = "https://github.com/user/my-project"
readme = "README.md"
license = "MIT OR Apache-2.0"
keywords = ["cli", "tool"]       # 最多 5 個
categories = ["command-line-utilities"]
exclude = ["tests/*", "benches/*"]  # 發布時排除
include = ["src/**/*", "Cargo.toml"] # 只包含這些

# 進階選項
rust-version = "1.70"            # 最低 Rust 版本
publish = true                   # 是否允許發布到 crates.io
default-run = "main"             # 預設執行的二進位
autobins = true                  # 自動發現 bin targets
autoexamples = true              # 自動發現 examples
autotests = true                 # 自動發現 tests
autobenches = true               # 自動發現 benches
```

### [dependencies] 版本語法

```toml
[dependencies]
# 精確版本
exact = "=1.2.3"

# 相容版本（預設）
# ^1.2.3 等於 >=1.2.3, <2.0.0
caret = "^1.2.3"
caret-short = "1.2.3"

# 小版本相容
# ~1.2.3 等於 >=1.2.3, <1.3.0
tilde = "~1.2.3"

# 萬用字元
wildcard-minor = "1.2.*"
wildcard-major = "1.*"

# 範圍
range = ">=1.0, <2.0"

# 來自 Git
from-git = { git = "https://github.com/user/repo" }
from-git-branch = { git = "https://github.com/user/repo", branch = "main" }
from-git-tag = { git = "https://github.com/user/repo", tag = "v1.0.0" }
from-git-rev = { git = "https://github.com/user/repo", rev = "abc123" }

# 來自本地路徑
from-path = { path = "../my-lib" }

# 帶 features
with-features = { version = "1.0", features = ["feature1", "feature2"] }
without-default = { version = "1.0", default-features = false }

# 可選依賴
optional-dep = { version = "1.0", optional = true }

# 重新命名
renamed = { package = "original-name", version = "1.0" }
```

### [dev-dependencies]

只在開發和測試時使用：

```toml
[dev-dependencies]
# 測試框架
criterion = "0.5"      # 效能測試
proptest = "1.4"       # 屬性測試
mockall = "0.12"       # Mock 框架

# 測試工具
tempfile = "3"         # 暫存檔案
assert_cmd = "2"       # CLI 測試
predicates = "3"       # 斷言工具
```

### [build-dependencies]

build.rs 使用的依賴：

```toml
[build-dependencies]
cc = "1.0"             # C/C++ 編譯
bindgen = "0.69"       # 自動生成 FFI 綁定
prost-build = "0.12"   # Protocol Buffers
```

### Features

```toml
[features]
# 預設啟用的 features
default = ["std", "json"]

# 定義 features
std = []
json = ["dep:serde_json"]
yaml = ["dep:serde_yaml"]
full = ["json", "yaml", "logging"]
logging = ["dep:tracing"]

# 依賴的 feature
async = ["tokio/full", "reqwest/json"]

[dependencies]
serde = "1"
serde_json = { version = "1", optional = true }
serde_yaml = { version = "0.9", optional = true }
tracing = { version = "0.1", optional = true }
tokio = { version = "1", optional = true }
reqwest = { version = "0.11", optional = true }
```

使用 features：

```bash
# 使用預設 features
cargo build

# 不使用預設 features
cargo build --no-default-features

# 指定 features
cargo build --features "json yaml"

# 啟用所有 features
cargo build --all-features
```

### [profile] 配置

```toml
# Debug 配置（cargo build）
[profile.dev]
opt-level = 0          # 最佳化等級 (0-3, s, z)
debug = true           # 除錯資訊
debug-assertions = true
overflow-checks = true
lto = false            # Link-Time Optimization
panic = "unwind"       # 或 "abort"
incremental = true     # 增量編譯

# Release 配置（cargo build --release）
[profile.release]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = true
panic = "abort"
codegen-units = 1      # 更好的最佳化，但編譯更慢
strip = true           # 移除符號

# 自訂配置
[profile.release-with-debug]
inherits = "release"
debug = true

# 針對特定套件的配置
[profile.dev.package.image]
opt-level = 3

# 針對所有依賴的配置
[profile.dev.package."*"]
opt-level = 2
```

---

## 19.2 工作空間

### 基本工作空間

```toml
# 根 Cargo.toml
[workspace]
members = [
    "crates/core",
    "crates/client",
    "crates/server",
]

# 排除某些目錄
exclude = [
    "experimental/*",
]

# 工作空間層級的依賴
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
thiserror = "1"
anyhow = "1"

# 工作空間層級的 package 資訊
[workspace.package]
version = "0.1.0"
authors = ["Your Name"]
edition = "2021"
license = "MIT"
repository = "https://github.com/user/project"
```

### 子專案配置

```toml
# crates/core/Cargo.toml
[package]
name = "my-core"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true

# 本地依賴
# 在工作空間內可以直接使用相對路徑

[dependencies]
my-utils = { path = "../utils" }
```

### 工作空間命令

```bash
# 在根目錄執行，影響所有成員
cargo build
cargo test
cargo check

# 只對特定專案執行
cargo build -p my-core
cargo test -p my-client

# 對所有專案執行
cargo build --workspace
cargo test --workspace

# 排除某些專案
cargo test --workspace --exclude expensive-tests
```

### 共享 Cargo.lock

工作空間中的所有成員共享一個 `Cargo.lock`，確保版本一致：

```
my-workspace/
├── Cargo.toml
├── Cargo.lock      # 共享的 lock 檔案
├── core/
│   └── Cargo.toml
└── cli/
    └── Cargo.toml
```

---

## 19.3 Cargo 擴充

### cargo-edit

簡化依賴管理：

```bash
# 安裝
cargo install cargo-edit

# 新增依賴
cargo add serde
cargo add tokio --features full
cargo add my-lib --path ../my-lib

# 移除依賴
cargo rm serde

# 升級依賴
cargo upgrade
cargo upgrade serde
cargo upgrade --incompatible  # 包含不相容的更新
```

### cargo-watch

自動重新編譯：

```bash
# 安裝
cargo install cargo-watch

# 監視變更並重新編譯
cargo watch

# 監視變更並執行
cargo watch -x run

# 監視變更並執行測試
cargo watch -x test

# 多個命令
cargo watch -x check -x test -x run

# 忽略某些檔案
cargo watch -i "*.txt" -i "docs/*"

# 延遲執行
cargo watch --delay 2

# 清除畫面
cargo watch -c -x run
```

### cargo-expand

展開巨集：

```bash
# 安裝
cargo install cargo-expand

# 展開所有巨集
cargo expand

# 展開特定模組
cargo expand my_module

# 展開特定函式
cargo expand my_module::my_function
```

### cargo-audit

安全性審計：

```bash
# 安裝
cargo install cargo-audit

# 掃描已知漏洞
cargo audit

# 產生報告
cargo audit --json > audit.json

# 忽略特定漏洞
cargo audit --ignore RUSTSEC-2020-0001
```

### cargo-outdated

檢查過時的依賴：

```bash
# 安裝
cargo install cargo-outdated

# 檢查過時依賴
cargo outdated

# 只檢查根專案
cargo outdated --root-deps-only

# 只檢查工作空間
cargo outdated --workspace
```

### 其他實用擴充

```bash
# cargo-flamegraph - 效能分析火焰圖
cargo install flamegraph
cargo flamegraph

# cargo-bloat - 分析二進位大小
cargo install cargo-bloat
cargo bloat --release

# cargo-deny - 依賴審計
cargo install cargo-deny
cargo deny check

# cargo-nextest - 更快的測試執行器
cargo install cargo-nextest
cargo nextest run

# cargo-llvm-lines - LLVM IR 行數分析
cargo install cargo-llvm-lines
cargo llvm-lines

# cargo-tree - 依賴樹
cargo tree
cargo tree --duplicates
cargo tree --invert some-crate
```

---

## 19.4 發布 Crate

### 準備發布

```toml
# 確保 Cargo.toml 有必要的資訊
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
description = "A brief description of what this crate does"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/my-crate"
documentation = "https://docs.rs/my-crate"
readme = "README.md"
keywords = ["keyword1", "keyword2"]
categories = ["category"]
```

### crates.io 帳號

```bash
# 取得 API token（從 crates.io 網站）
cargo login <your-api-token>

# token 儲存在 ~/.cargo/credentials.toml
```

### 發布前檢查

```bash
# 檢查套件
cargo publish --dry-run

# 檢查文件
cargo doc --open

# 執行所有測試
cargo test --all-features

# 檢查 clippy 警告
cargo clippy --all-features

# 格式化
cargo fmt --check
```

### 發布

```bash
# 發布到 crates.io
cargo publish

# 跳過某些檢查
cargo publish --allow-dirty
```

### 版本管理

```bash
# 遵循語義化版本
# MAJOR.MINOR.PATCH
# 0.1.0 -> 初始開發
# 0.1.1 -> 修復 bug
# 0.2.0 -> 新功能（不相容）
# 1.0.0 -> 穩定 API
# 1.0.1 -> 修復 bug
# 1.1.0 -> 新功能（相容）
# 2.0.0 -> 不相容變更
```

### Yanking

撤回有問題的版本：

```bash
# 撤回版本（防止新專案使用，但不影響已使用的專案）
cargo yank --version 0.1.1

# 取消撤回
cargo yank --version 0.1.1 --undo
```

### 文件撰寫

```rust
//! # My Crate
//!
//! `my_crate` 是一個用於處理某些事情的函式庫。
//!
//! ## 快速開始
//!
//! ```rust
//! use my_crate::do_something;
//!
//! fn main() {
//!     do_something();
//! }
//! ```

/// 執行某些操作。
///
/// # 範例
///
/// ```rust
/// use my_crate::do_something;
/// do_something();
/// ```
///
/// # 錯誤
///
/// 當發生某些情況時會回傳錯誤。
///
/// # Panics
///
/// 當輸入為空時會 panic。
pub fn do_something() {
    // ...
}
```

---

## 實用技巧

### 條件編譯

```toml
# Cargo.toml
[target.'cfg(unix)'.dependencies]
nix = "0.27"

[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
```

```rust
// 程式碼中
#[cfg(unix)]
fn platform_specific() {
    // Unix 專用程式碼
}

#[cfg(windows)]
fn platform_specific() {
    // Windows 專用程式碼
}

#[cfg(feature = "async")]
async fn async_function() {
    // 只在 async feature 啟用時編譯
}
```

### Build Script

```rust
// build.rs
fn main() {
    // 告訴 Cargo 在這些檔案變更時重新執行
    println!("cargo:rerun-if-changed=src/proto/message.proto");
    println!("cargo:rerun-if-env-changed=MY_ENV_VAR");

    // 設定環境變數供程式使用
    println!("cargo:rustc-env=BUILD_TIME={}", chrono::Utc::now());

    // 連結系統函式庫
    println!("cargo:rustc-link-lib=ssl");

    // 新增搜尋路徑
    println!("cargo:rustc-link-search=/usr/local/lib");

    // 傳遞 cfg flag
    println!("cargo:rustc-cfg=my_custom_flag");
}
```

### 環境變數

```rust
// 在編譯時取得資訊
fn main() {
    // Cargo 提供的環境變數
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let authors = env!("CARGO_PKG_AUTHORS");

    // 自訂環境變數（來自 build.rs）
    let build_time = env!("BUILD_TIME");

    println!("{} v{} by {}", name, version, authors);
    println!("Built at: {}", build_time);

    // 可選的環境變數
    let debug = option_env!("DEBUG").unwrap_or("false");
}
```

### 私有 Registry

```toml
# .cargo/config.toml
[registries]
my-registry = { index = "https://my-registry.example.com/index" }

[source.crates-io]
replace-with = "my-registry"  # 替換 crates.io

# Cargo.toml
[dependencies]
my-internal-crate = { version = "1.0", registry = "my-registry" }
```

---

## 練習題

### 練習 1：設定 Features

為一個 HTTP 客戶端函式庫設計 features：
- `default`：基本功能
- `json`：JSON 支援
- `async`：非同步支援
- `tls`：TLS 支援
- `full`：所有功能

### 練習 2：工作空間設計

設計一個工作空間結構，包含：
- `core`：核心邏輯
- `cli`：命令列工具
- `api`：REST API 服務
- `shared`：共享程式碼

### 練習 3：Build Script

撰寫 build.rs 來：
- 讀取 Git commit hash
- 設定建置時間
- 在程式中顯示這些資訊

---

## 本章小結

- **Cargo.toml**：完整的配置語法和選項
- **版本管理**：SemVer 和各種版本指定方式
- **Features**：條件編譯和可選功能
- **工作空間**：管理多專案結構
- **擴充工具**：cargo-edit、cargo-watch 等
- **發布**：準備和發布到 crates.io

---

## 延伸閱讀

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cargo.toml vs Cargo.lock](https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries)
- [Cargo Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
