# 第一章：環境設定與 Hello World

## 安裝 Rust

Rust 使用 `rustup` 作為官方安裝工具，類似 Node.js 的 `nvm`。

### macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

下載並執行 [rustup-init.exe](https://rustup.rs/)

### 驗證安裝

```bash
rustc --version    # Rust 編譯器
cargo --version    # 套件管理工具（類似 npm）
```

## Cargo：Rust 的 npm

| npm 指令 | Cargo 指令 | 說明 |
|----------|-----------|------|
| `npm init` | `cargo new` | 建立新專案 |
| `npm install` | `cargo build` | 編譯專案（自動下載依賴） |
| `npm run start` | `cargo run` | 編譯並執行 |
| `npm test` | `cargo test` | 執行測試 |
| `npm publish` | `cargo publish` | 發布套件 |
| `package.json` | `Cargo.toml` | 專案設定檔 |
| `node_modules/` | `target/` | 依賴與編譯產物 |

## 建立第一個專案

```bash
# 建立新專案（類似 npm init）
cargo new hello_rust
cd hello_rust
```

專案結構：

```
hello_rust/
├── Cargo.toml    # 類似 package.json
└── src/
    └── main.rs   # 程式進入點（類似 index.js）
```

### Cargo.toml（類似 package.json）

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# 在這裡加入依賴，類似 npm install xxx
```

### src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

## JS vs Rust：Hello World 對比

```javascript
// JavaScript
console.log("Hello, world!");
```

```rust
// Rust
fn main() {
    println!("Hello, world!");
}
```

### 關鍵差異

1. **必須有 `main` 函式**：Rust 程式從 `main()` 開始執行
2. **`println!` 是巨集**：注意驚嘆號 `!`，這是巨集（macro）的標記
3. **分號必要**：每個陳述式結尾都需要分號 `;`
4. **需要編譯**：Rust 是編譯語言，執行前需先編譯

## 編譯與執行

```bash
# 方法 1：分開執行
cargo build          # 編譯（產出在 target/debug/）
./target/debug/hello_rust  # 執行

# 方法 2：一步到位（推薦）
cargo run            # 編譯 + 執行

# 方法 3：發布版本（最佳化）
cargo build --release    # 產出在 target/release/
```

### 開發 vs 發布

| 模式 | 指令 | 編譯速度 | 執行速度 | 用途 |
|------|------|---------|---------|------|
| Debug | `cargo build` | 快 | 較慢 | 開發階段 |
| Release | `cargo build --release` | 慢 | 極快 | 正式發布 |

## VS Code 設定

### 必裝擴充套件

1. **rust-analyzer**：Rust 官方語言伺服器，提供：
   - 自動完成
   - 跳轉定義
   - 即時錯誤檢查
   - 型別提示

2. **CodeLLDB**：偵錯工具

### settings.json 建議設定

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

## 實用指令整理

```bash
# 檢查程式碼（不編譯，速度快）
cargo check

# 格式化程式碼（類似 prettier）
cargo fmt

# 程式碼品質檢查（類似 eslint）
cargo clippy

# 查看文件
cargo doc --open

# 更新 Rust 版本
rustup update
```

## 練習題

### 練習 1：建立並執行專案
1. 使用 `cargo new my_first_rust` 建立專案
2. 修改 `main.rs`，印出你的名字
3. 使用 `cargo run` 執行

### 練習 2：探索 Cargo
1. 執行 `cargo check` 和 `cargo build`，觀察差異
2. 執行 `cargo build --release`，比較 `target/debug` 和 `target/release` 的檔案大小

### 練習 3：故意出錯
試著把程式碼改成以下內容，觀察編譯器給你的錯誤訊息：

```rust
fn main() {
    println!("Hello, world!")   // 少了分號
    println!("Hello again!");
}
```

> 提示：Rust 編譯器的錯誤訊息非常清楚，善用它！

---

[下一章：基本語法與型別系統 →](./02-basics.md)
