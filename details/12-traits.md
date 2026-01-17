# 第 12 章：Trait 系統

> Trait 是 Rust 定義共享行為的方式，類似於其他語言的介面（interface）

---

## 12.1 Trait 基礎

Trait 定義了一組方法簽名，表示某種能力或行為。任何型別都可以實作 trait，從而獲得這些能力。

### 定義 Trait

```rust
// 定義一個 trait
trait Greet {
    fn greet(&self) -> String;
}

// 也可以有多個方法
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
    fn age(&self) -> u32;
}
```

### 實作 Trait

```rust
trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

struct Dog {
    name: String,
}

// 為 Person 實作 Greet
impl Greet for Person {
    fn greet(&self) -> String {
        format!("嗨，我是 {}！", self.name)
    }
}

// 為 Dog 實作 Greet
impl Greet for Dog {
    fn greet(&self) -> String {
        format!("汪汪！我是 {}！", self.name)
    }
}

fn main() {
    let person = Person { name: String::from("小明") };
    let dog = Dog { name: String::from("小黑") };

    println!("{}", person.greet()); // 嗨，我是 小明！
    println!("{}", dog.greet());    // 汪汪！我是 小黑！
}
```

### 預設實作

Trait 方法可以提供預設實作：

```rust
trait Summary {
    // 必須實作的方法
    fn title(&self) -> &str;

    // 有預設實作的方法
    fn summarize(&self) -> String {
        format!("（閱讀更多關於 {} ...）", self.title())
    }

    // 預設實作可以呼叫其他方法
    fn summarize_author(&self) -> String {
        String::from("未知作者")
    }

    fn full_summary(&self) -> String {
        format!("{} - {}", self.summarize(), self.summarize_author())
    }
}

struct Article {
    title: String,
    author: String,
    content: String,
}

// 只實作必要的方法，其他使用預設實作
impl Summary for Article {
    fn title(&self) -> &str {
        &self.title
    }

    // 覆寫預設實作
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn title(&self) -> &str {
        &self.content
    }

    // 覆寫 summarize
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust 入門"),
        author: String::from("rustacean"),
        content: String::from("..."),
    };

    println!("{}", article.summarize());      // （閱讀更多關於 Rust 入門 ...）
    println!("{}", article.full_summary());   // （閱讀更多...） - @rustacean

    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Hello, Rust!"),
    };

    println!("{}", tweet.summarize());        // rust_lang: Hello, Rust!
}
```

### 孤兒規則（Orphan Rule）

實作 trait 時有一個重要限制：

```rust
// 可以為自己的型別實作外部 trait
use std::fmt::Display;

struct MyType(i32);

impl Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyType({})", self.0)
    }
}

// 可以為外部型別實作自己的 trait
trait MyTrait {
    fn my_method(&self);
}

impl MyTrait for Vec<i32> {
    fn my_method(&self) {
        println!("Vec 的長度: {}", self.len());
    }
}

// 不能為外部型別實作外部 trait！
// impl Display for Vec<i32> { }  // 錯誤！
```

**孤兒規則**：你只能在以下情況實作 trait：
1. 該 trait 是你定義的，或
2. 該型別是你定義的

這個規則確保了程式碼的一致性，避免多個 crate 對同一個型別-trait 組合有不同的實作。

---

## 12.2 常用標準 Trait

Rust 標準庫定義了許多常用的 trait，了解它們對於編寫慣用的 Rust 程式碼非常重要。

### Debug 與 Display

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// Debug：用於除錯輸出，可以 derive
#[derive(Debug)]
struct PointDebug {
    x: i32,
    y: i32,
}

// Display：用於面向使用者的輸出，必須手動實作
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    let pd = PointDebug { x: 3, y: 4 };

    println!("{}", p);      // (3, 4)         - Display
    println!("{:?}", pd);   // PointDebug { x: 3, y: 4 } - Debug
    println!("{:#?}", pd);  // 美化的 Debug 輸出
}
```

### Clone 與 Copy

```rust
// Clone：可以明確複製
#[derive(Debug, Clone)]
struct Clonable {
    data: String,
}

// Copy：可以隱式複製（位元組複製）
// Copy 必須同時實作 Clone
#[derive(Debug, Clone, Copy)]
struct Copyable {
    x: i32,
    y: i32,
}

fn main() {
    // Clone 需要明確呼叫
    let a = Clonable { data: String::from("hello") };
    let b = a.clone();  // 明確複製
    println!("{:?}", a); // a 仍然有效
    println!("{:?}", b);

    // Copy 會隱式複製
    let p1 = Copyable { x: 1, y: 2 };
    let p2 = p1;  // 隱式複製，不是移動
    println!("{:?}", p1); // p1 仍然有效
    println!("{:?}", p2);
}
```

**何時可以實作 Copy**：
- 型別的所有欄位都必須是 Copy
- 不能包含 String、Vec、Box 等堆積資料
- 不能實作 Drop trait

### PartialEq、Eq、PartialOrd、Ord

```rust
// PartialEq：部分相等（可能有不可比較的值，如 NaN）
// Eq：完全相等（所有值都可比較）
#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

// PartialOrd：部分排序
// Ord：完全排序
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Score {
    value: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    println!("{}", p1 == p2); // true
    println!("{}", p1 == p3); // false

    let s1 = Score { value: 90 };
    let s2 = Score { value: 85 };

    println!("{}", s1 > s2);  // true
    println!("{}", s1 < s2);  // false
}
```

**手動實作比較 trait**：

```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// 只比較 age
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

impl Eq for Person {}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.age.partial_cmp(&other.age)
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age)
    }
}

fn main() {
    let mut people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 35 },
    ];

    people.sort(); // 按 age 排序

    for p in &people {
        println!("{}: {}", p.name, p.age);
    }
    // Bob: 25
    // Alice: 30
    // Charlie: 35
}
```

### Default

```rust
#[derive(Debug, Default)]
struct Config {
    debug: bool,
    port: u16,
    host: String,
}

// 自訂 Default 實作
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: String::from("localhost"),
            port: 8080,
            max_connections: 100,
        }
    }
}

fn main() {
    // 使用 derive 的 Default
    let config: Config = Default::default();
    println!("{:?}", config); // Config { debug: false, port: 0, host: "" }

    // 使用自訂 Default
    let server = ServerConfig::default();
    println!("{:?}", server);

    // 部分覆寫 Default 值
    let custom_server = ServerConfig {
        port: 3000,
        ..Default::default()
    };
    println!("{:?}", custom_server);
}
```

### From 與 Into

```rust
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(f64);

// 實作 From，自動獲得 Into
impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        Millimeters((m.0 * 1000.0) as u32)
    }
}

// 從字串建立
impl From<&str> for Millimeters {
    fn from(s: &str) -> Self {
        let value: u32 = s.parse().unwrap_or(0);
        Millimeters(value)
    }
}

fn main() {
    let meters = Meters(1.5);

    // 使用 From
    let mm1 = Millimeters::from(meters);
    println!("{:?}", mm1); // Millimeters(1500)

    // 使用 Into（需要型別提示）
    let meters2 = Meters(2.0);
    let mm2: Millimeters = meters2.into();
    println!("{:?}", mm2); // Millimeters(2000)

    // 從字串
    let mm3 = Millimeters::from("500");
    println!("{:?}", mm3); // Millimeters(500)
}
```

### AsRef 與 AsMut

```rust
use std::path::Path;

// AsRef 用於低成本的引用轉換
fn print_path(path: impl AsRef<Path>) {
    println!("路徑: {:?}", path.as_ref());
}

fn main() {
    // 可以傳入多種型別
    print_path("/usr/local");              // &str
    print_path(String::from("/home/user")); // String
    print_path(Path::new("/tmp"));          // &Path

    // AsRef 的另一個例子
    fn print_bytes(data: impl AsRef<[u8]>) {
        println!("bytes: {:?}", data.as_ref());
    }

    print_bytes("hello");      // &str
    print_bytes(b"hello");     // &[u8; 5]
    print_bytes(vec![1, 2, 3]); // Vec<u8>
}
```

### Hash

```rust
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

// 可以 derive
#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

// 手動實作（只使用部分欄位）
#[derive(Debug)]
struct Person {
    id: u64,
    name: String,
    age: u32,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state); // 只用 id 做雜湊
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Person {}

fn main() {
    let mut points = HashSet::new();
    points.insert(Point { x: 1, y: 2 });
    points.insert(Point { x: 3, y: 4 });
    points.insert(Point { x: 1, y: 2 }); // 重複，不會加入

    println!("點的數量: {}", points.len()); // 2
}
```

---

## 12.3 Trait 作為參數

### impl Trait 語法

```rust
use std::fmt::Display;

// impl Trait 作為參數（語法糖）
fn print_item(item: impl Display) {
    println!("{}", item);
}

// 等價於泛型 trait bound
fn print_item_generic<T: Display>(item: T) {
    println!("{}", item);
}

fn main() {
    print_item(42);
    print_item("hello");
    print_item_generic(3.14);
}
```

### Trait Bounds

```rust
use std::fmt::{Display, Debug};

// 多重 bounds
fn process<T: Display + Debug + Clone>(item: T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
    let _ = item.clone();
}

// 使用 where 子句
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
{
    format!("{} - {:?}", t, u)
}
```

### 靜態分派 vs 動態分派

```rust
use std::fmt::Display;

// 靜態分派：編譯時決定呼叫哪個方法
fn static_dispatch(item: impl Display) {
    println!("{}", item);
}

// 動態分派：執行時決定呼叫哪個方法
fn dynamic_dispatch(item: &dyn Display) {
    println!("{}", item);
}

fn main() {
    // 靜態分派 - 會為 i32 和 &str 各生成一份函式
    static_dispatch(42);
    static_dispatch("hello");

    // 動態分派 - 只有一份函式，透過 vtable 查找方法
    let x: i32 = 42;
    let s: &str = "hello";
    dynamic_dispatch(&x);
    dynamic_dispatch(&s);
}
```

**比較**：

| 特性 | 靜態分派 (impl Trait) | 動態分派 (dyn Trait) |
|------|----------------------|---------------------|
| 效能 | 更快（可內聯） | 較慢（vtable 查找） |
| 二進位大小 | 較大（單態化） | 較小 |
| 型別資訊 | 編譯時已知 | 編譯時未知 |
| 使用時機 | 效能關鍵 | 需要型別抹除 |

---

## 12.4 Trait 物件

Trait 物件允許在執行時處理不同型別的值，是 Rust 中實現多態的方式。

### dyn Trait

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("繪製圓形，半徑: {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("繪製矩形，{}x{}", self.width, self.height);
    }
}

fn main() {
    // 使用 trait 物件儲存不同型別
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];

    for shape in shapes {
        shape.draw();
    }
    // 繪製圓形，半徑: 5
    // 繪製矩形，10x20
    // 繪製圓形，半徑: 3
}
```

### 物件安全（Object Safety）

不是所有 trait 都可以作為 trait 物件使用。trait 必須是「物件安全」的：

```rust
// 物件安全的 trait
trait SafeTrait {
    fn method(&self);
    fn another_method(&self, x: i32);
}

// 非物件安全的 trait
trait NotSafe {
    // 回傳 Self 不是物件安全的
    fn clone(&self) -> Self;

    // 泛型方法不是物件安全的
    fn generic_method<T>(&self, x: T);

    // 沒有 self 參數不是物件安全的
    fn static_method();
}

// Clone trait 不是物件安全的，因為它回傳 Self
// let x: Box<dyn Clone> = ...; // 錯誤！
```

**物件安全規則**：
1. 回傳型別不能是 `Self`
2. 不能有泛型型別參數
3. 方法必須有 `self` 參數

**繞過限制的技巧**：

```rust
trait Cloneable: Clone {
    // 回傳 Box<dyn Cloneable> 而不是 Self
    fn clone_box(&self) -> Box<dyn Cloneable>;
}

impl<T: Clone + 'static> Cloneable for T {
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }
}

fn main() {
    let original: Box<dyn Cloneable> = Box::new(String::from("hello"));
    let cloned = original.clone_box();
}
```

### 效能考量

```rust
use std::time::Instant;

trait Compute {
    fn compute(&self) -> i64;
}

struct Calculator {
    value: i64,
}

impl Compute for Calculator {
    fn compute(&self) -> i64 {
        self.value * 2
    }
}

// 靜態分派
fn static_compute(items: &[Calculator]) -> i64 {
    items.iter().map(|x| x.compute()).sum()
}

// 動態分派
fn dynamic_compute(items: &[Box<dyn Compute>]) -> i64 {
    items.iter().map(|x| x.compute()).sum()
}

fn main() {
    // 效能測試（示意）
    let static_items: Vec<Calculator> = (0..1000)
        .map(|i| Calculator { value: i })
        .collect();

    let dynamic_items: Vec<Box<dyn Compute>> = (0..1000)
        .map(|i| Box::new(Calculator { value: i }) as Box<dyn Compute>)
        .collect();

    let start = Instant::now();
    let _ = static_compute(&static_items);
    println!("靜態分派: {:?}", start.elapsed());

    let start = Instant::now();
    let _ = dynamic_compute(&dynamic_items);
    println!("動態分派: {:?}", start.elapsed());
}
```

---

## 12.5 進階 Trait

### 關聯類型（Associated Types）

```rust
// 使用泛型
trait ContainerGeneric<T> {
    fn add(&mut self, item: T);
    fn get(&self) -> Option<&T>;
}

// 使用關聯類型（更簡潔）
trait Container {
    type Item;  // 關聯類型

    fn add(&mut self, item: Self::Item);
    fn get(&self) -> Option<&Self::Item>;
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;  // 指定關聯類型

    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn get(&self) -> Option<&Self::Item> {
        self.items.last()
    }
}

// 使用關聯類型的好處：呼叫時不需要指定型別參數
fn process_container<C: Container>(container: &C) -> Option<&C::Item> {
    container.get()
}
```

**Iterator trait 的關聯類型**：

```rust
// 標準庫的 Iterator trait
pub trait Iterator {
    type Item;  // 關聯類型

    fn next(&mut self) -> Option<Self::Item>;

    // 其他方法使用 Self::Item
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B,
    { /* ... */ }
}
```

### 預設泛型參數

```rust
use std::ops::Add;

// Add trait 的定義（標準庫）
// pub trait Add<Rhs = Self> {  // Rhs 預設為 Self
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// 使用預設的 Rhs（Point + Point）
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 自訂 Rhs（Point + i32）
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, scalar: i32) -> Point {
        Point {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let p3 = p1 + p2;    // Point + Point
    println!("{:?}", p3); // Point { x: 4, y: 6 }

    let p4 = p1 + 10;    // Point + i32
    println!("{:?}", p4); // Point { x: 11, y: 12 }
}
```

### 運算子重載

```rust
use std::ops::{Add, Sub, Mul, Neg, Index};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// 加法
impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

// 減法
impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

// 純量乘法
impl Mul<f64> for Vector2D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector2D::new(self.x * scalar, self.y * scalar)
    }
}

// 負號
impl Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Self {
        Vector2D::new(-self.x, -self.y)
    }
}

// 索引
impl Index<usize> for Vector2D {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("索引超出範圍"),
        }
    }
}

fn main() {
    let v1 = Vector2D::new(3.0, 4.0);
    let v2 = Vector2D::new(1.0, 2.0);

    println!("v1 + v2 = {:?}", v1 + v2);     // Vector2D { x: 4.0, y: 6.0 }
    println!("v1 - v2 = {:?}", v1 - v2);     // Vector2D { x: 2.0, y: 2.0 }
    println!("v1 * 2.0 = {:?}", v1 * 2.0);   // Vector2D { x: 6.0, y: 8.0 }
    println!("-v1 = {:?}", -v1);             // Vector2D { x: -3.0, y: -4.0 }
    println!("v1[0] = {}", v1[0]);           // 3.0
    println!("|v1| = {}", v1.magnitude());  // 5.0
}
```

### 完全限定語法

當有多個 trait 有相同名稱的方法時：

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("機長說話中...");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("巫師飛起來了！");
    }
}

impl Human {
    fn fly(&self) {
        println!("人類揮動雙臂");
    }
}

fn main() {
    let person = Human;

    // 預設呼叫 Human 自己的方法
    person.fly();  // 人類揮動雙臂

    // 使用完全限定語法呼叫特定 trait 的方法
    Pilot::fly(&person);  // 機長說話中...
    Wizard::fly(&person); // 巫師飛起來了！

    // 更明確的完全限定語法
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);
}
```

**關聯函式的完全限定語法**：

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("小狗")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("幼犬")
    }
}

fn main() {
    println!("{}", Dog::baby_name());           // 小狗
    // println!("{}", Animal::baby_name());     // 錯誤！不知道是哪個型別
    println!("{}", <Dog as Animal>::baby_name()); // 幼犬
}
```

### Supertraits

當一個 trait 依賴於另一個 trait 時：

```rust
use std::fmt;

// OutlinePrint 需要 Display 作為 supertrait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); // 可以使用 Display 的方法
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// 必須先實作 Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 然後才能實作 OutlinePrint
impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
    // ********
    // * (1, 3) *
    // ********
}
```

### Newtype 模式

繞過孤兒規則，為外部型別實作外部 trait：

```rust
use std::fmt;

// 無法直接為 Vec<String> 實作 Display
// impl fmt::Display for Vec<String> {} // 錯誤！

// 使用 Newtype 包裝
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 實作 Deref 讓 Wrapper 可以像 Vec 一樣使用
use std::ops::Deref;

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
    ]);

    println!("{}", w);     // [hello, world]
    println!("長度: {}", w.len());  // 透過 Deref 存取 Vec 的方法
}
```

---

## 練習題

### 練習 1：自訂 Summary Trait

```rust
trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        String::from("Anonymous")
    }
}

// 為以下結構體實作 Summary
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct BlogPost {
    title: String,
    author: String,
    content: String,
}
```

### 練習 2：運算子重載

為 `Fraction` 結構體實作加法和乘法：

```rust
#[derive(Debug, Clone, Copy)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

// 實作 Add 和 Mul trait
// 記得約分！
```

### 練習 3：Trait 物件

建立一個可以儲存不同形狀的圖形集合，並計算總面積：

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 }

// 實作各形狀的 Shape trait
// 建立一個函式計算 Vec<Box<dyn Shape>> 的總面積
```

---

## 本章小結

- **Trait 基礎**：Trait 定義共享行為，類似介面
- **預設實作**：可以提供方法的預設實作
- **孤兒規則**：限制 trait 實作的位置
- **標準 Trait**：Debug、Display、Clone、Copy、PartialEq 等
- **Trait Bounds**：限制泛型型別必須實作的 trait
- **Trait 物件**：使用 `dyn Trait` 實現動態多態
- **物件安全**：只有物件安全的 trait 才能作為 trait 物件
- **進階特性**：關聯類型、運算子重載、Supertraits、Newtype 模式

---

## 延伸閱讀

- [The Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust By Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
- [Rust Reference - Traits](https://doc.rust-lang.org/reference/items/traits.html)
- [Object Safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
