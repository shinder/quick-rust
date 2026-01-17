# 附錄 B：Rust 版本演進

> Rust 透過「版本」（Edition）機制進行語言演進，同時保持向後相容性

---

## 什麼是 Rust Edition？

Rust Edition 是一種讓語言可以進行不向後相容變更的機制，同時確保現有程式碼仍然可以編譯。

- 每個 crate 可以選擇使用的 edition
- 不同 edition 的 crate 可以互相依賴
- 新 edition 大約每 3 年發布一次
- 可以使用 `cargo fix --edition` 自動遷移

```toml
# Cargo.toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"  # 使用 Rust 2021 edition
```

---

## Rust 2015

**首個穩定版本**（Rust 1.0，2015 年 5 月）

這是 Rust 的第一個穩定版本，建立了語言的核心特性：

### 核心特性

- 所有權系統（Ownership）
- 借用檢查器（Borrow Checker）
- 生命週期（Lifetimes）
- 特徵（Traits）
- 泛型（Generics）
- 模式匹配（Pattern Matching）
- 零成本抽象

### 程式碼風格

```rust
// Rust 2015
extern crate serde;  // 需要明確聲明外部 crate

use serde::Serialize;

fn main() {
    // ...
}
```

---

## Rust 2018

**發布日期**：2018 年 12 月（Rust 1.31）

這是 Rust 的第一次重大 edition 更新，帶來了許多提升開發體驗的改進。

### 主要變更

#### 1. 模組系統簡化

```rust
// Rust 2015
extern crate my_lib;
use my_lib::SomeType;

// Rust 2018
use my_lib::SomeType;  // 不需要 extern crate
use crate::module::Item;  // 使用 crate 關鍵字表示當前 crate
```

#### 2. 路徑清晰化

```rust
// Rust 2018 路徑
use crate::foo::bar;      // crate 根
use self::baz::qux;       // 當前模組
use super::parent_item;   // 父模組
```

#### 3. 生命週期省略擴展

```rust
// Rust 2018 - impl 區塊中可以省略生命週期
impl MyStruct {
    fn method(&self) -> &str {
        &self.name
    }
}
```

#### 4. NLL（Non-Lexical Lifetimes）

```rust
// Rust 2015 - 這會編譯失敗
fn main() {
    let mut x = 5;
    let y = &x;
    x = 6;  // 錯誤：y 的生命週期延伸到區塊結尾
}

// Rust 2018 - NLL 讓這段程式碼可以編譯
fn main() {
    let mut x = 5;
    let y = &x;
    println!("{}", y);  // y 的最後使用
    x = 6;  // OK：y 不再被使用
}
```

#### 5. 新的關鍵字

- `async` / `await`（保留，1.39 啟用）
- `try`（保留）
- `dyn`（trait 物件語法）

```rust
// Rust 2018 - dyn 是必須的
fn print_it(input: &dyn std::fmt::Display) {
    println!("{}", input);
}
```

#### 6. `?` 用於 `main` 和測試

```rust
// Rust 2018
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    println!("{}", content);
    Ok(())
}
```

---

## Rust 2021

**發布日期**：2021 年 10 月（Rust 1.56）

### 主要變更

#### 1. 閉包不相交捕獲

```rust
// Rust 2018 - 閉包捕獲整個結構
struct Point { x: i32, y: i32 }

let point = Point { x: 1, y: 2 };
let closure = || println!("{}", point.x);  // 捕獲整個 point

// Rust 2021 - 閉包只捕獲需要的欄位
let point = Point { x: 1, y: 2 };
let closure = || println!("{}", point.x);  // 只捕獲 point.x
drop(point.y);  // OK：point.y 沒有被閉包捕獲
```

#### 2. IntoIterator for arrays

```rust
// Rust 2021 - 陣列直接實作 IntoIterator
fn main() {
    let array = [1, 2, 3];

    // 現在可以直接迭代陣列
    for item in array {
        println!("{}", item);
    }

    // 等同於
    for item in array.into_iter() {
        println!("{}", item);
    }
}
```

#### 3. 新的 prelude

Rust 2021 prelude 新增：
- `TryFrom` / `TryInto`
- `FromIterator`

```rust
// Rust 2021 - 這些 trait 自動可用
fn main() {
    let num: i32 = "42".parse().unwrap();
    let byte: u8 = num.try_into().unwrap();  // TryInto 自動可用
}
```

#### 4. Panic 巨集格式化

```rust
// Rust 2021 - panic! 現在一致使用 format_args!
panic!("{}", value);  // 總是格式化
panic!("{value}");    // 支援格式字串中的變數
```

#### 5. 保留語法

為未來擴展保留：
- `ident#`、`ident"..."`、`ident'...'`

#### 6. Or patterns

```rust
// Rust 2021
let x = Some(2);
if let Some(1 | 2) = x {
    println!("one or two");
}

// 在頂層使用
fn is_zero(x: i32) -> bool {
    matches!(x, 0 | -0)
}
```

---

## Rust 2024

**發布日期**：2024 年（Rust 1.85 預計）

### 預期變更

#### 1. RPIT（Return Position Impl Trait）生命週期捕獲規則

```rust
// Rust 2024 - 更直覺的生命週期捕獲
fn foo(x: &str) -> impl Display {
    x  // 自動捕獲生命週期
}
```

#### 2. 新的 prelude 項目

預計新增：
- `Future`
- `IntoFuture`

#### 3. `unsafe_op_in_unsafe_fn` lint 預設啟用

```rust
// Rust 2024 - unsafe fn 內部也需要 unsafe 區塊
unsafe fn dangerous_operation() {
    unsafe {
        // unsafe 操作必須在 unsafe 區塊中
    }
}
```

#### 4. 新的保留關鍵字

- `gen`（用於生成器）

---

## Edition 遷移指南

### 檢查當前 edition

```bash
# 查看 Cargo.toml
cat Cargo.toml | grep edition
```

### 自動遷移

```bash
# 遷移到新 edition
cargo fix --edition

# 預覽變更
cargo fix --edition --dry-run

# 遷移到特定 edition
cargo fix --edition-idioms
```

### 手動遷移步驟

1. **備份程式碼**
   ```bash
   git commit -am "Before edition migration"
   ```

2. **更新 Cargo.toml**
   ```toml
   edition = "2021"
   ```

3. **執行自動修復**
   ```bash
   cargo fix --edition
   ```

4. **檢查並修復剩餘問題**
   ```bash
   cargo build
   cargo test
   ```

5. **更新程式碼風格**（可選）
   ```bash
   cargo fix --edition-idioms
   ```

---

## Edition 相容性

```
┌─────────────────────────────────────────────────────┐
│                    Rust Compiler                     │
│                                                     │
│   ┌─────────┐   ┌─────────┐   ┌─────────┐          │
│   │ 2015    │   │ 2018    │   │ 2021    │          │
│   │ crate   │◄─►│ crate   │◄─►│ crate   │          │
│   └─────────┘   └─────────┘   └─────────┘          │
│        ▲            ▲             ▲                 │
│        └────────────┴─────────────┘                 │
│              可以互相依賴                            │
└─────────────────────────────────────────────────────┘
```

- 不同 edition 的 crate 可以無縫協作
- 依賴項不需要使用相同的 edition
- 編譯器會自動處理 edition 差異

---

## 重要版本里程碑

| 版本 | 發布日期 | 重要特性 |
|------|----------|----------|
| 1.0 | 2015-05 | 首個穩定版本 |
| 1.15 | 2017-02 | 自訂 derive |
| 1.26 | 2018-05 | impl Trait |
| 1.31 | 2018-12 | Rust 2018 Edition |
| 1.36 | 2019-07 | Future trait |
| 1.39 | 2019-11 | async/await |
| 1.45 | 2020-07 | 函式式程序巨集 |
| 1.51 | 2021-03 | const generics（基礎）|
| 1.56 | 2021-10 | Rust 2021 Edition |
| 1.65 | 2022-11 | GAT（Generic Associated Types）|
| 1.75 | 2023-12 | async fn in traits |

---

## 延伸閱讀

- [Edition Guide](https://doc.rust-lang.org/edition-guide/)
- [Rust Release Notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md)
- [RFC Book](https://rust-lang.github.io/rfcs/)
