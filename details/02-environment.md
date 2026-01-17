# 第 2 章：環境建置

> 工欲善其事，必先利其器。本章將帶你建立完整的 Rust 開發環境。

---

## 2.1 安裝 Rust

### rustup：Rust 的官方安裝工具

`rustup` 是 Rust 官方的工具鏈管理器，類似於 Node.js 的 `nvm` 或 Python 的 `pyenv`。它可以：

- 安裝和更新 Rust
- 管理多個 Rust 版本
- 安裝額外的元件（如 rust-src、rustfmt）
- 管理編譯目標（cross-compilation）

### macOS / Linux 安裝

打開終端機，執行以下命令：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安裝過程中會詢問安裝選項，一般選擇預設（按 Enter）即可：

```
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

安裝完成後，重新載入環境變數：

```bash
# 方法 1：重新開啟終端機

# 方法 2：手動載入
source $HOME/.cargo/env
```

### Windows 安裝

1. 下載 [rustup-init.exe](https://rustup.rs/)
2. 執行安裝程式
3. 按照指示完成安裝

**注意**：Windows 需要安裝 Visual Studio Build Tools：
- 下載 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- 選擇「使用 C++ 的桌面開發」工作負載

### 驗證安裝

```bash
# 檢查 Rust 編譯器版本
rustc --version
# rustc 1.75.0 (82e1608df 2023-12-21)

# 檢查 Cargo（套件管理器）版本
cargo --version
# cargo 1.75.0 (1d8b05cdd 2023-11-20)

# 檢查 rustup 版本
rustup --version
# rustup 1.26.0 (5af9b9484 2023-04-05)
```

### 多版本管理

Rust 有三個發布頻道：

| 頻道 | 說明 | 更新頻率 |
|------|------|---------|
| **stable** | 穩定版，正式專案使用 | 每 6 週 |
| **beta** | 測試版，下一個 stable 的候選 | 每 6 週 |
| **nightly** | 每日建置，包含實驗性功能 | 每天 |

```bash
# 安裝不同版本
rustup install stable
rustup install beta
rustup install nightly

# 切換預設版本
rustup default stable

# 查看已安裝的版本
rustup show

# 為特定專案使用特定版本
cd my-project
rustup override set nightly

# 使用特定版本執行命令
rustup run nightly cargo build
# 或使用 +版本 語法
cargo +nightly build
```

### 更新 Rust

```bash
# 更新所有已安裝的工具鏈
rustup update

# 更新特定版本
rustup update stable
```

### 安裝額外元件

```bash
# 查看可用元件
rustup component list

# 安裝常用元件
rustup component add rustfmt      # 程式碼格式化
rustup component add clippy       # 程式碼檢查
rustup component add rust-src     # Rust 原始碼（IDE 需要）
rustup component add rust-analyzer # 語言伺服器
```

---

## 2.2 開發工具設定

### VS Code（推薦）

VS Code 是目前最流行的 Rust 開發環境，設定簡單且功能完整。

#### 必裝擴充套件

1. **rust-analyzer**
   - Rust 官方語言伺服器
   - 提供自動完成、跳轉定義、即時錯誤檢查、型別提示
   - 搜尋 `rust-analyzer` 並安裝

2. **CodeLLDB**
   - 偵錯工具
   - 支援設定中斷點、單步執行、變數檢視
   - 搜尋 `CodeLLDB` 並安裝

3. **Even Better TOML**
   - TOML 檔案語法高亮
   - Cargo.toml 編輯更方便

4. **crates**
   - 在 Cargo.toml 中顯示 crate 版本資訊
   - 提示最新版本

#### VS Code 設定

在 `.vscode/settings.json` 中加入：

```json
{
    // rust-analyzer 設定
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "rust-analyzer.inlayHints.chainingHints.enable": true,

    // 儲存時自動格式化
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.tabSize": 4
    },

    // 檔案關聯
    "files.associations": {
        "*.rs": "rust"
    }
}
```

#### 偵錯設定

建立 `.vscode/launch.json`：

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable",
            "cargo": {
                "args": [
                    "build",
                    "--bin=${workspaceFolderBasename}",
                    "--package=${workspaceFolderBasename}"
                ],
                "filter": {
                    "name": "${workspaceFolderBasename}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=${workspaceFolderBasename}",
                    "--package=${workspaceFolderBasename}"
                ],
                "filter": {
                    "name": "${workspaceFolderBasename}",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

### IntelliJ IDEA / CLion

JetBrains 的 IDE 也提供優秀的 Rust 支援。

#### 安裝方式

1. **IntelliJ IDEA**（免費社群版可用）
   - 安裝 `Rust` 插件
   - File → Settings → Plugins → 搜尋 "Rust"

2. **CLion**（付費，但功能最完整）
   - 原生支援 Rust
   - 更好的偵錯體驗

#### 優點

- 強大的重構功能
- 完整的偵錯工具
- 內建終端機
- 版本控制整合

### Vim / Neovim

對於 Vim 使用者，可以使用以下設定：

#### 使用 rust-analyzer（推薦）

配合 LSP 客戶端（如 coc.nvim 或 nvim-lspconfig）：

```lua
-- Neovim with nvim-lspconfig
require'lspconfig'.rust_analyzer.setup{
    settings = {
        ["rust-analyzer"] = {
            checkOnSave = {
                command = "clippy"
            }
        }
    }
}
```

#### 常用插件

```vim
" vim-plug 設定範例
call plug#begin()

Plug 'rust-lang/rust.vim'           " Rust 語法支援
Plug 'neoclide/coc.nvim'            " LSP 客戶端
Plug 'dense-analysis/ale'           " 非同步 Lint

call plug#end()

" rust.vim 設定
let g:rustfmt_autosave = 1          " 儲存時自動格式化
```

### 偵錯工具

#### CodeLLDB（VS Code）

前面已介紹，是 VS Code 最佳選擇。

#### LLDB（命令列）

```bash
# macOS / Linux
lldb target/debug/my_program

# 常用命令
(lldb) breakpoint set --name main    # 設定中斷點
(lldb) run                           # 執行
(lldb) step                          # 單步執行
(lldb) next                          # 下一行
(lldb) print variable_name           # 印出變數
(lldb) continue                      # 繼續執行
```

#### GDB（Linux）

```bash
gdb target/debug/my_program

# 常用命令
(gdb) break main                     # 設定中斷點
(gdb) run                            # 執行
(gdb) step                           # 單步執行
(gdb) next                           # 下一行
(gdb) print variable_name            # 印出變數
(gdb) continue                       # 繼續執行
```

#### rust-gdb / rust-lldb

Rust 提供包裝過的偵錯工具，可以更好地顯示 Rust 型別：

```bash
rust-lldb target/debug/my_program
rust-gdb target/debug/my_program
```

---

## 2.3 Cargo 套件管理工具

Cargo 是 Rust 的官方建置系統和套件管理器，類似於：
- Node.js 的 npm
- Python 的 pip + setuptools
- Java 的 Maven

### 專案建立

```bash
# 建立新的執行檔專案
cargo new my_project
cd my_project

# 建立新的函式庫專案
cargo new my_library --lib

# 在現有目錄初始化專案
mkdir existing_dir
cd existing_dir
cargo init

# 初始化為函式庫
cargo init --lib
```

### 專案結構

```
my_project/
├── Cargo.toml          # 專案設定檔（類似 package.json）
├── Cargo.lock          # 依賴版本鎖定檔（自動產生）
├── src/
│   ├── main.rs         # 程式進入點（執行檔）
│   └── lib.rs          # 函式庫進入點（函式庫）
├── tests/              # 整合測試
├── benches/            # 基準測試
├── examples/           # 範例程式
└── target/             # 編譯產出（自動產生）
```

### Cargo.toml 詳解

```toml
[package]
name = "my_project"           # 專案名稱
version = "0.1.0"             # 版本號（SemVer）
edition = "2021"              # Rust 版本（2015、2018、2021）
authors = ["Your Name <you@example.com>"]
description = "A sample project"
license = "MIT"
repository = "https://github.com/user/repo"
keywords = ["example", "rust"]
categories = ["command-line-utilities"]

[dependencies]
# 從 crates.io 安裝
serde = "1.0"                              # 最新 1.x 版本
serde_json = "1.0.108"                     # 精確版本
tokio = { version = "1", features = ["full"] }  # 帶功能標誌

# 從 Git 安裝
my_crate = { git = "https://github.com/user/repo" }
my_crate = { git = "https://github.com/user/repo", branch = "dev" }
my_crate = { git = "https://github.com/user/repo", tag = "v1.0.0" }

# 從本地路徑安裝
my_local = { path = "../my_local_crate" }

[dev-dependencies]
# 只在開發/測試時使用
criterion = "0.5"              # 基準測試框架
mockall = "0.12"               # Mock 框架

[build-dependencies]
# 建置腳本使用的依賴
cc = "1.0"                     # C 編譯器

[features]
# 功能標誌（條件編譯）
default = ["std"]
std = []
full = ["feature1", "feature2"]
feature1 = []
feature2 = []

[[bin]]
# 額外的執行檔
name = "another_binary"
path = "src/bin/another.rs"

[[example]]
# 範例程式
name = "my_example"
path = "examples/my_example.rs"
```

### 編譯與執行

```bash
# 檢查程式碼（不產生執行檔，速度最快）
cargo check

# 編譯（Debug 模式）
cargo build

# 編譯（Release 模式，最佳化）
cargo build --release

# 編譯並執行
cargo run

# 帶參數執行
cargo run -- arg1 arg2

# Release 模式執行
cargo run --release
```

### 測試

```bash
# 執行所有測試
cargo test

# 執行特定測試
cargo test test_name

# 執行特定模組的測試
cargo test module_name::

# 顯示 println! 輸出
cargo test -- --nocapture

# 執行被忽略的測試
cargo test -- --ignored

# 並行測試控制
cargo test -- --test-threads=1
```

### 程式碼品質

```bash
# 格式化程式碼
cargo fmt

# 檢查格式（CI 用）
cargo fmt -- --check

# 執行 Clippy（進階 Lint）
cargo clippy

# 嚴格模式（警告視為錯誤）
cargo clippy -- -D warnings

# 自動修復
cargo clippy --fix
```

### 文件

```bash
# 產生文件
cargo doc

# 產生並開啟文件
cargo doc --open

# 包含私有項目
cargo doc --document-private-items

# 包含依賴的文件
cargo doc --no-deps
```

### 其他常用命令

```bash
# 清理建置產物
cargo clean

# 更新依賴
cargo update

# 查看依賴樹
cargo tree

# 搜尋 crate
cargo search serde

# 安裝執行檔 crate
cargo install ripgrep

# 反安裝
cargo uninstall ripgrep

# 發布到 crates.io
cargo publish

# 登入 crates.io
cargo login
```

---

## 2.4 Hello World 詳解

### 建立並執行第一個程式

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

輸出：

```
   Compiling hello_rust v0.1.0 (/path/to/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/hello_rust`
Hello, world!
```

### 程式碼解析

```rust
// src/main.rs

fn main() {
    println!("Hello, world!");
}
```

#### `fn main()`

- `fn` 是定義函式的關鍵字
- `main` 是程式的進入點，每個執行檔必須有
- `()` 表示沒有參數
- `{}` 是函式主體

#### `println!`

- 注意結尾的 `!`，這表示它是一個**巨集**（macro），不是普通函式
- 巨集在編譯時期展開，可以接受可變數量的參數
- 類似 C 的 `printf`，但更安全

```rust
// println! 的各種用法
println!("Hello, world!");                    // 基本輸出
println!("數字: {}", 42);                      // 格式化輸出
println!("x = {}, y = {}", 10, 20);           // 多個值
println!("名字: {name}", name = "Alice");      // 具名參數
println!("{:?}", vec![1, 2, 3]);              // Debug 格式
println!("{:#?}", some_struct);               // 美化 Debug 格式
println!("{:04}", 42);                        // 補零：0042
println!("{:.2}", 3.14159);                   // 小數位數：3.14
println!("{:>10}", "right");                  // 靠右對齊
println!("{:<10}", "left");                   // 靠左對齊
```

### 編譯過程

Rust 的編譯過程：

```
原始碼 (.rs)
    ↓
詞法分析 & 語法分析
    ↓
抽象語法樹 (AST)
    ↓
高階中間表示 (HIR)
    ↓
型別檢查 & 借用檢查
    ↓
中階中間表示 (MIR)
    ↓
LLVM IR
    ↓
機器碼 (可執行檔)
```

#### 關鍵階段說明

1. **AST（抽象語法樹）**
   - 將原始碼轉換為樹狀結構
   - 巨集在此階段展開

2. **HIR（High-level IR）**
   - 去除語法糖
   - 解析名稱

3. **型別檢查 & 借用檢查**
   - Rust 特有的所有權檢查在此執行
   - 確保記憶體安全

4. **MIR（Mid-level IR）**
   - 簡化的控制流
   - 借用檢查的細節在此完成

5. **LLVM IR**
   - 轉換為 LLVM 的中間表示
   - 享受 LLVM 的最佳化

6. **機器碼**
   - 最終產出平台特定的執行檔

### 查看編譯產物

```bash
# Debug 建置（預設）
cargo build
ls -la target/debug/hello_rust
# -rwxr-xr-x  1 user  staff  4.2M  hello_rust

# Release 建置
cargo build --release
ls -la target/release/hello_rust
# -rwxr-xr-x  1 user  staff  380K  hello_rust
```

注意 Release 版本小很多且執行更快，因為：
- 移除了除錯資訊
- 應用了各種最佳化
- 移除了邊界檢查（部分）

### 檢視展開後的巨集

```bash
# 安裝 cargo-expand
cargo install cargo-expand

# 查看展開後的程式碼
cargo expand
```

`println!("Hello, world!")` 展開後類似：

```rust
{
    ::std::io::_print(
        ::core::fmt::Arguments::new_v1(
            &["Hello, world!\n"],
            &[],
        ),
    );
}
```

---

## 本章重點回顧

1. **安裝 Rust**
   - 使用 `rustup` 安裝和管理 Rust
   - 瞭解 stable、beta、nightly 三個頻道
   - 安裝必要元件：rustfmt、clippy

2. **開發環境**
   - VS Code + rust-analyzer 是最推薦的組合
   - 設定好自動格式化和 Clippy 檢查
   - 配置偵錯環境

3. **Cargo**
   - 專案建立：`cargo new`
   - 編譯執行：`cargo build`、`cargo run`
   - 程式碼品質：`cargo fmt`、`cargo clippy`
   - 測試：`cargo test`

4. **Hello World**
   - `fn main()` 是程式進入點
   - `println!` 是巨集，用於格式化輸出
   - 瞭解 Rust 的編譯過程

---

## 練習題

### 練習 1：環境驗證

執行以下命令，確認環境正確：

```bash
rustc --version
cargo --version
rustup show
```

### 練習 2：建立並執行專案

1. 使用 `cargo new greeting` 建立專案
2. 修改 `main.rs`，印出你的名字
3. 執行 `cargo run`
4. 執行 `cargo build --release`，比較 debug 和 release 版本的檔案大小

### 練習 3：使用 Cargo 工具

1. 執行 `cargo fmt` 格式化程式碼
2. 執行 `cargo clippy` 檢查程式碼
3. 執行 `cargo doc --open` 查看文件

### 練習 4：println! 練習

修改程式，使用 `println!` 的各種格式化功能：

```rust
fn main() {
    let name = "Rust";
    let version = 1.75;
    let features = vec!["safe", "fast", "concurrent"];

    // 印出：學習 Rust 1.75 版
    // 印出：特色：["safe", "fast", "concurrent"]
    // 印出版本號，保留一位小數
}
```

---

## 延伸閱讀

- [rustup 文件](https://rust-lang.github.io/rustup/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [rust-analyzer 手冊](https://rust-analyzer.github.io/manual.html)

---

[← 上一章：認識 Rust](./01-introduction.md) | [下一章：基本語法 →](./03-basics.md)
