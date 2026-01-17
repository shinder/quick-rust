# 附錄 D：Rust 速查表

> 快速參考 Rust 語法、常用 trait 和標準函式庫

---

## 變數與型別

```rust
// 變數宣告
let x = 5;              // 不可變
let mut y = 10;         // 可變
const MAX: u32 = 100;   // 常數
static NAME: &str = "Rust";  // 靜態變數

// 型別標註
let x: i32 = 5;
let s: &str = "hello";
let v: Vec<i32> = Vec::new();

// 型別別名
type Kilometers = i32;
```

### 基本型別

| 類型 | 型別 | 範例 |
|------|------|------|
| 整數 | `i8, i16, i32, i64, i128, isize` | `let x: i32 = -42;` |
| 無號整數 | `u8, u16, u32, u64, u128, usize` | `let x: u32 = 42;` |
| 浮點數 | `f32, f64` | `let x: f64 = 3.14;` |
| 布林 | `bool` | `let b: bool = true;` |
| 字元 | `char` | `let c: char = '中';` |
| 字串 | `&str, String` | `let s = "hello";` |
| 陣列 | `[T; N]` | `let a: [i32; 3] = [1, 2, 3];` |
| 元組 | `(T1, T2, ...)` | `let t: (i32, &str) = (1, "a");` |
| 切片 | `&[T]` | `let s: &[i32] = &a[..];` |

---

## 函式

```rust
// 基本函式
fn add(a: i32, b: i32) -> i32 {
    a + b  // 無分號 = 返回值
}

// 無返回值
fn print_value(x: i32) {
    println!("{}", x);
}

// 泛型函式
fn first<T>(list: &[T]) -> Option<&T> {
    list.first()
}

// 帶 trait 約束
fn print_it<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}

// where 子句
fn complex<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

// 閉包
let add = |a, b| a + b;
let add_typed = |a: i32, b: i32| -> i32 { a + b };
```

---

## 控制流程

```rust
// if 表達式
let result = if x > 5 { "big" } else { "small" };

// if let
if let Some(x) = option {
    println!("{}", x);
}

// match
match value {
    0 => println!("zero"),
    1 | 2 => println!("one or two"),
    3..=9 => println!("three to nine"),
    n if n > 100 => println!("big: {}", n),
    _ => println!("other"),
}

// loop
let result = loop {
    if condition {
        break 42;  // 返回值
    }
};

// while
while condition {
    // ...
}

// while let
while let Some(x) = iter.next() {
    println!("{}", x);
}

// for
for i in 0..10 {
    println!("{}", i);
}

for (index, value) in vec.iter().enumerate() {
    println!("{}: {}", index, value);
}
```

---

## 結構體與列舉

```rust
// 結構體
struct Point {
    x: f64,
    y: f64,
}

// 元組結構體
struct Color(u8, u8, u8);

// 單元結構體
struct Marker;

// 實作方法
impl Point {
    // 關聯函式（建構子）
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // 方法
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // 可變方法
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// 列舉
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

// 實作列舉方法
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        }
    }
}
```

---

## Trait

```rust
// 定義 trait
trait Summary {
    fn summarize(&self) -> String;

    // 預設實作
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..50])
    }
}

// 實作 trait
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.title)
    }
}

// trait 約束
fn notify(item: &impl Summary) { }
fn notify<T: Summary>(item: &T) { }
fn notify<T: Summary + Display>(item: &T) { }

// trait 作為返回值
fn create() -> impl Summary { }

// trait 物件
fn process(items: &[Box<dyn Summary>]) { }
```

### 常用 Derive Trait

```rust
#[derive(Debug)]           // {:?} 和 {:#?} 格式化
#[derive(Clone)]           // .clone() 方法
#[derive(Copy)]            // 複製語義（需要 Clone）
#[derive(PartialEq, Eq)]   // == 和 != 比較
#[derive(PartialOrd, Ord)] // <, >, <=, >= 比較
#[derive(Hash)]            // 可作為 HashMap 的 key
#[derive(Default)]         // Default::default()
```

### 常用標準 Trait

| Trait | 用途 | 主要方法 |
|-------|------|----------|
| `Display` | 格式化輸出 | `fmt(&self, f: &mut Formatter)` |
| `Debug` | 除錯輸出 | `fmt(&self, f: &mut Formatter)` |
| `Clone` | 深複製 | `clone(&self) -> Self` |
| `Copy` | 隱式複製 | 標記 trait |
| `Default` | 預設值 | `default() -> Self` |
| `From<T>` | 型別轉換 | `from(T) -> Self` |
| `Into<T>` | 型別轉換 | `into(self) -> T` |
| `AsRef<T>` | 引用轉換 | `as_ref(&self) -> &T` |
| `Deref` | 解引用 | `deref(&self) -> &Target` |
| `Drop` | 析構函式 | `drop(&mut self)` |
| `Iterator` | 迭代器 | `next(&mut self) -> Option<Item>` |
| `IntoIterator` | 轉為迭代器 | `into_iter(self) -> Iterator` |
| `Fn` | 可呼叫（不可變） | `call(&self, args)` |
| `FnMut` | 可呼叫（可變） | `call_mut(&mut self, args)` |
| `FnOnce` | 可呼叫（消耗） | `call_once(self, args)` |

---

## 錯誤處理

```rust
// Result 處理
fn read_file() -> Result<String, io::Error> {
    let content = fs::read_to_string("file.txt")?;
    Ok(content)
}

// Option 處理
fn find_user(id: u32) -> Option<User> {
    users.get(&id).cloned()
}

// 常用方法
result.unwrap()           // 取值或 panic
result.unwrap_or(default) // 取值或預設值
result.unwrap_or_else(|| compute())  // 取值或計算
result.expect("msg")      // 取值或帶訊息 panic
result.ok()               // Result -> Option
result.map(|x| x + 1)     // 轉換內部值
result.and_then(|x| f(x)) // 鏈式處理
result.or_else(|e| g(e))  // 錯誤處理

option.is_some()          // 檢查是否有值
option.is_none()          // 檢查是否為空
option.as_ref()           // &Option<T> -> Option<&T>
option.take()             // 取出值，留下 None
```

---

## 集合

```rust
// Vec
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
v.push(4);
v.pop();
v.len();
v.is_empty();
v.get(0);           // Option<&T>
v[0];               // 直接存取（可能 panic）
v.iter();           // 不可變迭代
v.iter_mut();       // 可變迭代
v.into_iter();      // 消耗迭代

// HashMap
use std::collections::HashMap;
let mut map: HashMap<&str, i32> = HashMap::new();
map.insert("key", 1);
map.get("key");           // Option<&V>
map.entry("key").or_insert(0);
map.contains_key("key");
map.remove("key");

// HashSet
use std::collections::HashSet;
let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
set.contains(&1);
set.remove(&1);
set.union(&other);
set.intersection(&other);

// VecDeque（雙端佇列）
use std::collections::VecDeque;
let mut deque: VecDeque<i32> = VecDeque::new();
deque.push_front(1);
deque.push_back(2);
deque.pop_front();
deque.pop_back();

// BinaryHeap（優先佇列）
use std::collections::BinaryHeap;
let mut heap = BinaryHeap::new();
heap.push(3);
heap.pop();  // 返回最大值
```

---

## 迭代器

```rust
// 建立迭代器
let iter = vec.iter();          // &T
let iter = vec.iter_mut();      // &mut T
let iter = vec.into_iter();     // T

// 常用方法
iter.map(|x| x * 2)             // 轉換
iter.filter(|x| **x > 0)        // 過濾
iter.filter_map(|x| x.ok())     // 過濾並轉換
iter.flat_map(|x| x.iter())     // 扁平化
iter.flatten()                  // 扁平化巢狀迭代器
iter.take(5)                    // 取前 n 個
iter.skip(3)                    // 跳過前 n 個
iter.step_by(2)                 // 每隔 n 個
iter.chain(other)               // 串接
iter.zip(other)                 // 配對
iter.enumerate()                // 帶索引
iter.peekable()                 // 可預看
iter.rev()                      // 反向
iter.cycle()                    // 無限循環

// 終端操作
iter.collect::<Vec<_>>()        // 收集
iter.count()                    // 計數
iter.sum::<i32>()               // 求和
iter.product::<i32>()           // 求積
iter.fold(0, |acc, x| acc + x)  // 摺疊
iter.reduce(|a, b| a + b)       // 歸約
iter.all(|x| *x > 0)            // 全部滿足
iter.any(|x| *x > 0)            // 任一滿足
iter.find(|x| **x > 5)          // 尋找
iter.position(|x| *x > 5)       // 尋找位置
iter.max()                      // 最大值
iter.min()                      // 最小值
iter.last()                     // 最後一個
iter.nth(5)                     // 第 n 個
iter.for_each(|x| println!("{}", x))  // 遍歷
```

---

## 字串處理

```rust
// 建立字串
let s = String::new();
let s = String::from("hello");
let s = "hello".to_string();
let s = format!("{} {}", "hello", "world");

// 字串操作
s.push('!');                    // 加入字元
s.push_str(" world");           // 加入字串
s.len();                        // 位元組長度
s.chars().count();              // 字元數
s.is_empty();
s.contains("ell");
s.starts_with("he");
s.ends_with("lo");
s.replace("l", "L");            // 取代
s.trim();                       // 去除空白
s.trim_start();
s.trim_end();
s.to_uppercase();
s.to_lowercase();
s.split(',');                   // 分割
s.split_whitespace();
s.lines();                      // 按行分割
s.chars();                      // 字元迭代
s.bytes();                      // 位元組迭代
&s[0..5];                       // 切片（位元組）

// &str 和 String 轉換
let s: &str = &string;
let s: String = str_ref.to_string();
let s: String = str_ref.to_owned();
let s: String = String::from(str_ref);
```

---

## 所有權與借用

```rust
// 所有權規則
let s1 = String::from("hello");
let s2 = s1;           // s1 被移動，不能再使用
let s3 = s2.clone();   // 複製

// 借用規則
let s = String::from("hello");
let r1 = &s;           // 不可變借用
let r2 = &s;           // 可以有多個不可變借用
let r3 = &mut s;       // 錯誤！已有不可變借用

let mut s = String::from("hello");
let r1 = &mut s;       // 可變借用
let r2 = &mut s;       // 錯誤！只能有一個可變借用

// 生命週期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct Excerpt<'a> {
    part: &'a str,
}
```

---

## 智慧指標

```rust
// Box - 堆積配置
let b = Box::new(5);
let list = Box::new(Node { value: 1, next: None });

// Rc - 引用計數
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);
Rc::strong_count(&a);

// Arc - 原子引用計數（多執行緒）
use std::sync::Arc;
let a = Arc::new(5);
let b = Arc::clone(&a);

// RefCell - 內部可變性
use std::cell::RefCell;
let cell = RefCell::new(5);
*cell.borrow_mut() += 1;
let value = *cell.borrow();

// Mutex - 互斥鎖
use std::sync::Mutex;
let m = Mutex::new(5);
let mut guard = m.lock().unwrap();
*guard += 1;

// RwLock - 讀寫鎖
use std::sync::RwLock;
let lock = RwLock::new(5);
let r = lock.read().unwrap();
let mut w = lock.write().unwrap();
```

---

## 並行處理

```rust
use std::thread;
use std::sync::mpsc;

// 建立執行緒
let handle = thread::spawn(|| {
    println!("Hello from thread!");
});
handle.join().unwrap();

// 移動所有權到執行緒
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v);
});

// 通道
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();

thread::spawn(move || {
    tx.send("hello").unwrap();
});

let received = rx.recv().unwrap();
```

---

## 非同步

```rust
// async 函式
async fn fetch_data() -> String {
    "data".to_string()
}

// .await
async fn process() {
    let data = fetch_data().await;
}

// Tokio 執行時
#[tokio::main]
async fn main() {
    let result = fetch_data().await;
}

// 並行執行
let (a, b) = tokio::join!(task1(), task2());

// 選擇第一個完成的
tokio::select! {
    result = task1() => { /* ... */ }
    result = task2() => { /* ... */ }
}

// spawn 任務
let handle = tokio::spawn(async {
    // ...
});
```

---

## 巨集

```rust
// 宣告巨集
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// 帶參數
macro_rules! create_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

// 常用標準巨集
println!("Hello, {}!", name);
format!("{} {}", a, b);
vec![1, 2, 3];
assert!(condition);
assert_eq!(a, b);
panic!("error message");
todo!();
unimplemented!();
dbg!(expression);
```

---

## 屬性

```rust
// 條件編譯
#[cfg(target_os = "linux")]
fn linux_only() { }

#[cfg(feature = "my_feature")]
fn feature_only() { }

// Derive
#[derive(Debug, Clone, PartialEq)]
struct MyStruct { }

// 測試
#[test]
fn test_something() { }

#[cfg(test)]
mod tests { }

// 文件
/// 文件註解
#[doc = "文件"]
fn documented() { }

// 其他
#[allow(unused)]
#[warn(missing_docs)]
#[deny(unsafe_code)]
#[must_use]
#[deprecated(since = "1.0", note = "use new_fn instead")]
#[inline]
#[repr(C)]
```

---

## 模組系統

```rust
// 模組宣告
mod my_module {
    pub fn public_fn() { }
    fn private_fn() { }
}

// 使用
use my_module::public_fn;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};
use crate::my_module::*;

// 可見性
pub fn public() { }           // 公開
pub(crate) fn crate_only() { }  // crate 內
pub(super) fn parent_only() { } // 父模組
fn private() { }              // 私有

// 重新匯出
pub use self::inner::Type;
```

---

## Cargo 命令

```bash
# 專案管理
cargo new my_project      # 建立新專案
cargo init               # 初始化現有目錄
cargo build              # 編譯
cargo build --release    # Release 編譯
cargo run                # 編譯並執行
cargo check              # 快速檢查

# 測試與文件
cargo test               # 執行測試
cargo test test_name     # 執行特定測試
cargo bench              # 執行基準測試
cargo doc --open         # 產生並開啟文件

# 依賴管理
cargo add serde          # 加入依賴
cargo update             # 更新依賴
cargo tree               # 顯示依賴樹

# 程式碼品質
cargo fmt                # 格式化程式碼
cargo clippy             # Lint 檢查
cargo fix                # 自動修復警告

# 發布
cargo publish            # 發布到 crates.io
cargo login              # 登入 crates.io
```

---

## 延伸閱讀

- [Rust 標準函式庫文件](https://doc.rust-lang.org/std/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Cheat Sheet](https://cheats.rs/)
