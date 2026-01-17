# 附錄 C：常見錯誤與解決方案

> Rust 編譯器的錯誤訊息通常很有幫助，這裡列出新手常遇到的錯誤及解決方法

---

## 借用檢查器錯誤

### E0382: 使用已移動的值

```rust
// 錯誤
fn main() {
    let s = String::from("hello");
    let s2 = s;  // s 被移動
    println!("{}", s);  // 錯誤！s 已經無效
}
```

**解決方案**：

```rust
// 方案 1：使用克隆
fn main() {
    let s = String::from("hello");
    let s2 = s.clone();  // 複製一份
    println!("{}", s);   // OK
}

// 方案 2：使用引用
fn main() {
    let s = String::from("hello");
    let s2 = &s;  // 借用
    println!("{}", s);   // OK
}

// 方案 3：使用 Copy 型別
fn main() {
    let x = 5;
    let y = x;  // i32 實作了 Copy
    println!("{}", x);  // OK
}
```

---

### E0502: 同時存在可變和不可變借用

```rust
// 錯誤
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];      // 不可變借用
    v.push(4);              // 需要可變借用 - 錯誤！
    println!("{}", first);
}
```

**解決方案**：

```rust
// 確保不可變借用在可變借用之前結束
fn main() {
    let mut v = vec![1, 2, 3];
    let first = v[0];       // 複製值而非借用
    v.push(4);              // OK
    println!("{}", first);
}

// 或者重新組織程式碼
fn main() {
    let mut v = vec![1, 2, 3];
    {
        let first = &v[0];
        println!("{}", first);
    }  // first 在這裡結束
    v.push(4);  // OK
}
```

---

### E0499: 多個可變借用

```rust
// 錯誤
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;  // 錯誤！不能同時有兩個可變借用
    println!("{}, {}", r1, r2);
}
```

**解決方案**：

```rust
// 在不同作用域中使用可變借用
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" world");
    }
    let r2 = &mut s;  // OK：r1 已經離開作用域
    println!("{}", r2);
}
```

---

### E0597: 借用的值存活時間不夠長

```rust
// 錯誤
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // 錯誤！x 在區塊結束時被丟棄
    }
    println!("{}", r);  // r 引用了已丟棄的值
}
```

**解決方案**：

```rust
// 確保被引用的值存活足夠長
fn main() {
    let x = 5;
    let r = &x;
    println!("{}", r);  // OK
}

// 或者返回擁有的值
fn get_value() -> String {
    let s = String::from("hello");
    s  // 返回擁有的值，而非引用
}
```

---

## 生命週期錯誤

### E0106: 缺少生命週期標註

```rust
// 錯誤
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**解決方案**：

```rust
// 添加生命週期標註
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

### E0621: 生命週期不匹配

```rust
// 錯誤
struct Holder<'a> {
    value: &'a str,
}

impl<'a> Holder<'a> {
    fn get_value(&self) -> &str {  // 隱含返回 &'self str
        self.value  // 但 self.value 的生命週期是 'a
    }
}
```

**解決方案**：

```rust
impl<'a> Holder<'a> {
    fn get_value(&self) -> &'a str {
        self.value
    }
}
```

---

## Trait 約束錯誤

### E0277: Trait 約束不滿足

```rust
// 錯誤
fn print_it<T>(value: T) {
    println!("{}", value);  // 錯誤！T 沒有實作 Display
}
```

**解決方案**：

```rust
use std::fmt::Display;

fn print_it<T: Display>(value: T) {
    println!("{}", value);
}

// 或使用 where 子句
fn print_it<T>(value: T)
where
    T: Display,
{
    println!("{}", value);
}
```

---

### E0308: 型別不匹配

```rust
// 錯誤
fn add_one(x: i32) -> i32 {
    x + 1;  // 這是一個語句，返回 ()
}
```

**解決方案**：

```rust
fn add_one(x: i32) -> i32 {
    x + 1  // 移除分號，使其成為表達式
}

// 或明確 return
fn add_one(x: i32) -> i32 {
    return x + 1;
}
```

---

### E0369: 運算子無法使用

```rust
// 錯誤
struct Point { x: i32, y: i32 }

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // 錯誤！Point 沒有實作 Add
}
```

**解決方案**：

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // OK
    println!("{:?}", p3);
}
```

---

## 非同步錯誤

### Future 沒有被 await

```rust
// 錯誤
async fn fetch_data() -> String {
    String::from("data")
}

async fn main() {
    let data = fetch_data();  // 警告：Future 沒有被 await
    println!("{}", data);     // 錯誤：這不是 String
}
```

**解決方案**：

```rust
async fn fetch_data() -> String {
    String::from("data")
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await;  // 添加 .await
    println!("{}", data);
}
```

---

### Send / Sync 問題

```rust
// 錯誤
use std::rc::Rc;
use tokio::spawn;

async fn bad() {
    let rc = Rc::new(5);  // Rc 不是 Send
    spawn(async move {
        println!("{}", rc);  // 錯誤！
    });
}
```

**解決方案**：

```rust
use std::sync::Arc;
use tokio::spawn;

async fn good() {
    let arc = Arc::new(5);  // Arc 是 Send + Sync
    spawn(async move {
        println!("{}", arc);  // OK
    });
}
```

---

## 常見模式錯誤

### Option/Result 處理

```rust
// 不好的做法
fn bad_unwrap(x: Option<i32>) -> i32 {
    x.unwrap()  // 可能 panic
}
```

**解決方案**：

```rust
// 使用 match
fn with_match(x: Option<i32>) -> i32 {
    match x {
        Some(v) => v,
        None => 0,
    }
}

// 使用 unwrap_or
fn with_default(x: Option<i32>) -> i32 {
    x.unwrap_or(0)
}

// 使用 ? 運算子
fn with_question_mark(x: Option<i32>) -> Option<i32> {
    let value = x?;
    Some(value * 2)
}

// 使用 if let
fn with_if_let(x: Option<i32>) {
    if let Some(v) = x {
        println!("{}", v);
    }
}
```

---

### 字串轉換

```rust
// 錯誤：&str 和 String 不同
fn bad(s: &str) {
    let owned: String = s;  // 錯誤！
}
```

**解決方案**：

```rust
fn good(s: &str) {
    // 方法 1
    let owned: String = s.to_string();

    // 方法 2
    let owned: String = s.to_owned();

    // 方法 3
    let owned: String = String::from(s);

    // 方法 4
    let owned: String = s.into();
}
```

---

## 編譯錯誤快速參考

| 錯誤碼 | 說明 | 常見原因 |
|--------|------|----------|
| E0382 | 使用已移動的值 | 所有權已轉移 |
| E0502 | 借用衝突 | 同時有可變和不可變借用 |
| E0499 | 多個可變借用 | 同一時間多個 &mut |
| E0597 | 借用超出作用域 | 引用的值先被丟棄 |
| E0106 | 缺少生命週期 | 需要標註生命週期 |
| E0277 | Trait 約束不滿足 | 缺少 trait 實作 |
| E0308 | 型別不匹配 | 返回型別錯誤 |
| E0425 | 找不到值 | 變數未定義 |
| E0433 | 找不到模組 | 模組未匯入 |

---

## 除錯技巧

### 使用 dbg! 巨集

```rust
fn main() {
    let x = 2;
    let y = dbg!(x * 2);  // 印出 [src/main.rs:3] x * 2 = 4
    println!("{}", y);
}
```

### 使用 #[derive(Debug)]

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);   // Point { x: 1, y: 2 }
    println!("{:#?}", p);  // 美化輸出
}
```

### 查看完整錯誤

```bash
# 顯示完整錯誤說明
rustc --explain E0382

# 顯示更多建議
cargo build 2>&1 | head -50
```

---

## 延伸閱讀

- [Rust Error Index](https://doc.rust-lang.org/error_codes/)
- [Rust Compiler Error Messages](https://doc.rust-lang.org/rustc/)
