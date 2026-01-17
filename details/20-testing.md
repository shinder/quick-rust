# 第 20 章：測試

> Rust 內建強大的測試框架，讓你能夠輕鬆撰寫和執行單元測試、整合測試和文件測試

---

## 20.1 單元測試

單元測試用於測試程式碼中的小型、獨立的單元。

### #[test] 屬性

```rust
// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除以零"))
    } else {
        Ok(a / b)
    }
}

// 測試模組
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, 3), 1);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }
}
```

執行測試：

```bash
# 執行所有測試
cargo test

# 只執行特定測試
cargo test test_add

# 只執行符合模式的測試
cargo test divide

# 顯示測試輸出
cargo test -- --nocapture

# 單執行緒執行（避免平行測試的問題）
cargo test -- --test-threads=1
```

### assert! 巨集系列

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_assertions() {
        // 基本斷言
        assert!(true);
        assert!(1 + 1 == 2);

        // 相等斷言
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);

        // 帶訊息的斷言
        assert!(1 < 2, "1 應該小於 2");
        assert_eq!(2 + 2, 4, "加法運算錯誤");
        assert_ne!(2 + 2, 5, "2 + 2 不應該等於 5");

        // 使用格式化訊息
        let expected = 4;
        let actual = 2 + 2;
        assert_eq!(actual, expected, "預期 {}，但得到 {}", expected, actual);
    }

    // 測試 panic
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("這個測試應該 panic");
    }

    // 測試特定的 panic 訊息
    #[test]
    #[should_panic(expected = "除以零")]
    fn test_panic_message() {
        panic!("除以零錯誤");
    }

    // 使用 Result 回傳值
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("數學壞掉了"))
        }
    }
}
```

### 測試私有函式

```rust
// 私有函式
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(n: i32) -> i32 {
    internal_adder(n, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 可以測試私有函式
    #[test]
    fn test_internal_adder() {
        assert_eq!(internal_adder(2, 3), 5);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(3), 5);
    }
}
```

### 測試組織

```rust
// src/lib.rs
pub mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn test_subtract() {
            assert_eq!(subtract(5, 3), 2);
        }
    }
}

pub mod converter {
    pub fn to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_to_celsius() {
            assert!((to_celsius(32.0) - 0.0).abs() < 0.001);
            assert!((to_celsius(212.0) - 100.0).abs() < 0.001);
        }
    }
}
```

### 忽略測試

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn expensive_test() {
        // 耗時的測試
    }

    #[test]
    #[ignore = "需要外部服務"]
    fn test_with_external_service() {
        // 需要外部服務的測試
    }
}
```

```bash
# 只執行被忽略的測試
cargo test -- --ignored

# 執行所有測試（包括被忽略的）
cargo test -- --include-ignored
```

---

## 20.2 整合測試

整合測試用於測試函式庫的公開 API，模擬外部使用者的使用方式。

### tests/ 目錄

```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/                  # 整合測試目錄
    ├── integration_test.rs
    └── common/             # 共用測試工具
        └── mod.rs
```

**tests/integration_test.rs**：
```rust
// 整合測試檔案是獨立的 crate
use my_project::add;
use my_project::calculator::subtract;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5, 3), 2);
}

mod helper_tests {
    use super::*;

    #[test]
    fn test_combined_operations() {
        let result = add(subtract(10, 3), 5);
        assert_eq!(result, 12);
    }
}
```

### 共用測試工具

**tests/common/mod.rs**：
```rust
// 共用的測試輔助函式
pub fn setup() {
    // 測試前的設定
    println!("設定測試環境");
}

pub fn teardown() {
    // 測試後的清理
    println!("清理測試環境");
}

pub struct TestContext {
    pub data: Vec<i32>,
}

impl TestContext {
    pub fn new() -> Self {
        TestContext {
            data: vec![1, 2, 3, 4, 5],
        }
    }
}
```

**tests/integration_test.rs**：
```rust
mod common;

use common::{setup, TestContext};

#[test]
fn test_with_context() {
    setup();
    let ctx = TestContext::new();
    assert_eq!(ctx.data.len(), 5);
}
```

### 測試二進位專案

對於只有 `main.rs` 的專案，需要將邏輯放入函式庫：

```rust
// src/lib.rs
pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // 主要邏輯
    Ok(())
}

// src/main.rs
use my_project::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run(&args) {
        eprintln!("錯誤: {}", e);
        std::process::exit(1);
    }
}
```

**tests/cli_test.rs**：
```rust
use my_project::run;

#[test]
fn test_run_with_args() {
    let args = vec![
        String::from("my_project"),
        String::from("--option"),
        String::from("value"),
    ];
    assert!(run(&args).is_ok());
}
```

---

## 20.3 文件測試

文件中的程式碼範例會自動作為測試執行。

### 文件註解中的程式碼

```rust
/// 將兩個數字相加。
///
/// # 範例
///
/// ```
/// use my_project::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # 邊界情況
///
/// ```
/// use my_project::add;
///
/// // 負數相加
/// assert_eq!(add(-1, 1), 0);
///
/// // 零相加
/// assert_eq!(add(0, 0), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 可能失敗的函式。
///
/// # 錯誤
///
/// 當除數為零時回傳錯誤：
///
/// ```
/// use my_project::divide;
///
/// assert!(divide(10, 0).is_err());
/// ```
///
/// # 範例
///
/// ```
/// use my_project::divide;
///
/// let result = divide(10, 2)?;
/// assert_eq!(result, 5);
/// # Ok::<(), String>(())
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除以零"))
    } else {
        Ok(a / b)
    }
}
```

### 隱藏程式碼行

```rust
/// 計算平均值。
///
/// ```
/// # // 以 # 開頭的行不會顯示在文件中，但會被執行
/// # fn main() {
/// use my_project::average;
///
/// let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let avg = average(&nums);
/// assert!((avg - 3.0).abs() < 0.001);
/// # }
/// ```
pub fn average(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}
```

### 不執行的範例

```rust
/// 這個函式需要網路連線。
///
/// ```no_run
/// use my_project::fetch_data;
///
/// // 這段程式碼會被編譯，但不會執行
/// let data = fetch_data("https://example.com")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 實際的網路請求
    Ok(String::new())
}

/// 這個範例只用於說明語法。
///
/// ```ignore
/// // 這段程式碼不會被編譯或執行
/// let x = complicated_setup();
/// ```
pub fn example() {}

/// 這個範例應該編譯失敗。
///
/// ```compile_fail
/// use my_project::private_function;
///
/// // 這會編譯失敗，因為函式是私有的
/// private_function();
/// ```
fn private_function() {}
```

### 自動執行範例

```bash
# 執行文件測試
cargo test --doc

# 只執行特定模組的文件測試
cargo test --doc add
```

---

## 20.4 進階測試

### 自訂測試框架

```toml
# Cargo.toml
[dev-dependencies]
rstest = "0.18"  # 參數化測試
```

```rust
use rstest::rstest;

#[rstest]
#[case(0, 0, 0)]
#[case(1, 1, 2)]
#[case(2, 3, 5)]
#[case(-1, 1, 0)]
fn test_add_parametrized(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected);
}

// 使用 fixture
#[rstest]
fn test_with_fixture(#[values(1, 2, 3)] n: i32) {
    assert!(n > 0);
}
```

### 基準測試（Benchmark）

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

**benches/my_benchmark.rs**：
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_project::fibonacci;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn comparison_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");

    for i in [10, 15, 20].iter() {
        group.bench_with_input(format!("fib {}", i), i, |b, i| {
            b.iter(|| fibonacci(black_box(*i)))
        });
    }

    group.finish();
}

criterion_group!(benches, fibonacci_benchmark, comparison_benchmark);
criterion_main!(benches);
```

```bash
# 執行基準測試
cargo bench

# 只執行特定的基準測試
cargo bench fib
```

### 模糊測試（Fuzzing）

```toml
# Cargo.toml
[dev-dependencies]
arbitrary = "1"

# 或使用 cargo-fuzz
```

使用 `cargo-fuzz`：

```bash
# 安裝
cargo install cargo-fuzz

# 初始化
cargo fuzz init

# 建立 fuzz target
cargo fuzz add my_fuzz_target
```

**fuzz/fuzz_targets/my_fuzz_target.rs**：
```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use my_project::parse_input;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_input(s);
    }
});
```

```bash
# 執行模糊測試
cargo fuzz run my_fuzz_target
```

### 屬性測試（Property-based Testing）

```toml
# Cargo.toml
[dev-dependencies]
proptest = "1.4"
```

```rust
use proptest::prelude::*;

fn reverse<T: Clone>(input: &[T]) -> Vec<T> {
    input.iter().rev().cloned().collect()
}

proptest! {
    #[test]
    fn test_reverse_preserves_length(ref v in prop::collection::vec(any::<i32>(), 0..100)) {
        let reversed = reverse(v);
        prop_assert_eq!(v.len(), reversed.len());
    }

    #[test]
    fn test_reverse_twice_is_identity(ref v in prop::collection::vec(any::<i32>(), 0..100)) {
        let reversed_twice = reverse(&reverse(v));
        prop_assert_eq!(v, &reversed_twice);
    }

    #[test]
    fn test_add_commutative(a: i32, b: i32) {
        prop_assert_eq!(add(a, b), add(b, a));
    }
}
```

### Mock 測試

```toml
# Cargo.toml
[dev-dependencies]
mockall = "0.12"
```

```rust
use mockall::{automock, predicate::*};

#[automock]
trait Database {
    fn get_user(&self, id: u64) -> Option<String>;
    fn save_user(&self, id: u64, name: &str) -> Result<(), String>;
}

struct UserService<D: Database> {
    db: D,
}

impl<D: Database> UserService<D> {
    fn new(db: D) -> Self {
        UserService { db }
    }

    fn greet_user(&self, id: u64) -> String {
        match self.db.get_user(id) {
            Some(name) => format!("Hello, {}!", name),
            None => String::from("User not found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_existing_user() {
        let mut mock_db = MockDatabase::new();
        mock_db
            .expect_get_user()
            .with(eq(1))
            .times(1)
            .returning(|_| Some(String::from("Alice")));

        let service = UserService::new(mock_db);
        assert_eq!(service.greet_user(1), "Hello, Alice!");
    }

    #[test]
    fn test_greet_missing_user() {
        let mut mock_db = MockDatabase::new();
        mock_db
            .expect_get_user()
            .with(eq(999))
            .times(1)
            .returning(|_| None);

        let service = UserService::new(mock_db);
        assert_eq!(service.greet_user(999), "User not found");
    }
}
```

### 非同步測試

```rust
// 使用 tokio
#[tokio::test]
async fn test_async_function() {
    let result = async_add(2, 3).await;
    assert_eq!(result, 5);
}

// 使用 async-std
#[async_std::test]
async fn test_with_async_std() {
    let result = fetch_data().await;
    assert!(result.is_ok());
}

// 配置 tokio
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_multi_threaded() {
    // 多執行緒測試
}
```

---

## 測試最佳實踐

### 測試命名

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 描述性的測試名稱
    #[test]
    fn add_returns_sum_of_two_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_handles_negative_numbers() {
        assert_eq!(add(-2, 3), 1);
    }

    #[test]
    fn divide_returns_error_when_divisor_is_zero() {
        assert!(divide(10, 0).is_err());
    }
}
```

### 測試結構（Arrange-Act-Assert）

```rust
#[test]
fn test_user_registration() {
    // Arrange（準備）
    let mut user_service = UserService::new();
    let username = "alice";
    let email = "alice@example.com";

    // Act（執行）
    let result = user_service.register(username, email);

    // Assert（斷言）
    assert!(result.is_ok());
    assert_eq!(user_service.get_user_count(), 1);
}
```

### 測試輔助函式

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 建立測試資料的輔助函式
    fn create_test_user() -> User {
        User {
            id: 1,
            name: String::from("Test User"),
            email: String::from("test@example.com"),
        }
    }

    fn create_test_context() -> TestContext {
        TestContext {
            users: vec![create_test_user()],
            db: MockDatabase::new(),
        }
    }

    #[test]
    fn test_with_helper() {
        let ctx = create_test_context();
        assert_eq!(ctx.users.len(), 1);
    }
}
```

---

## 練習題

### 練習 1：單元測試

為以下函式撰寫完整的單元測試：

```rust
pub fn is_palindrome(s: &str) -> bool {
    // 判斷字串是否為回文
    todo!()
}
```

### 練習 2：整合測試

為一個簡單的 TODO 應用程式撰寫整合測試：
- 新增 TODO
- 列出 TODO
- 標記完成
- 刪除 TODO

### 練習 3：屬性測試

使用 proptest 為排序函式撰寫屬性測試：
- 排序後長度不變
- 排序後元素有序
- 排序後包含所有原始元素

---

## 本章小結

- **單元測試**：測試獨立的程式碼單元
- **整合測試**：測試公開 API
- **文件測試**：確保文件範例可運作
- **進階測試**：基準測試、模糊測試、屬性測試、Mock

---

## 延伸閱讀

- [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust By Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [The Cargo Book - Tests](https://doc.rust-lang.org/cargo/guide/tests.html)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
