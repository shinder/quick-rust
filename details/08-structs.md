# 第 8 章：結構體

> 結構體是 Rust 中建立自訂型別的主要方式，類似其他語言的 class 或 record。

---

## 8.1 定義結構體

### 具名欄位結構體

最常見的結構體形式：

```rust
// 定義結構體
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 建立實例
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };

    println!("用戶名: {}", user1.username);
}
```

### 元組結構體（Tuple Struct）

沒有欄位名稱，只有型別：

```rust
// 定義元組結構體
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 使用索引存取
    println!("R: {}, G: {}, B: {}", black.0, black.1, black.2);

    // 解構
    let Color(r, g, b) = black;
    println!("RGB: {}, {}, {}", r, g, b);

    // 注意：Color 和 Point 是不同型別，即使結構相同
    // let p: Point = black;  // 錯誤！型別不同
}
```

**用途：**
- 給元組一個有意義的名稱
- 建立新型別（Newtype pattern）

```rust
// Newtype 模式：包裝現有型別以獲得型別安全
struct Meters(f64);
struct Kilometers(f64);

fn calculate_distance(meters: Meters) -> Kilometers {
    Kilometers(meters.0 / 1000.0)
}

fn main() {
    let distance = Meters(5000.0);
    let km = calculate_distance(distance);
    // calculate_distance(Kilometers(5.0));  // 錯誤！型別不匹配
}
```

### 單元結構體（Unit Struct）

沒有任何欄位的結構體：

```rust
// 定義單元結構體
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;

    // 主要用於實作 trait
}

// 實際用途：標記型別
struct Marker;

trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for Marker {
    fn do_something(&self) {
        println!("Marker doing something");
    }
}
```

---

## 8.2 實例化與存取

### 建立實例

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 完整語法
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };

    // 存取欄位
    println!("Email: {}", user1.email);
}
```

### 欄位簡寫語法

當變數名稱與欄位名稱相同時：

```rust
fn build_user(email: String, username: String) -> User {
    // 簡寫語法
    User {
        email,              // 等同於 email: email
        username,           // 等同於 username: username
        active: true,
        sign_in_count: 1,
    }
}
```

### 結構體更新語法

使用 `..` 從其他實例複製欄位：

```rust
fn main() {
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };

    // 使用更新語法建立新實例
    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1  // 其他欄位從 user1 複製
    };

    // 注意：如果複製了非 Copy 型別的欄位，原實例該欄位會失效
    // println!("{}", user1.username);  // 錯誤！username 已被移動
    println!("{}", user1.active);       // OK！bool 是 Copy
    println!("{}", user1.email);        // OK！沒有被複製
}
```

### 可變實例

```rust
fn main() {
    // 整個實例必須是可變的
    let mut user = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };

    // 修改欄位
    user.email = String::from("newemail@example.com");
    user.sign_in_count += 1;

    // Rust 不允許只有部分欄位是可變的
}
```

---

## 8.3 方法與關聯函式

### impl 區塊

使用 `impl` 為結構體定義方法：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法：第一個參數是 self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // 可以有多個參數
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("面積: {}", rect.area());
    println!("周長: {}", rect.perimeter());
}
```

### self 的三種形式

```rust
impl Rectangle {
    // 1. &self - 不可變借用（最常用）
    // 只讀取資料，不修改
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 2. &mut self - 可變借用
    // 修改資料
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    // 3. self - 取得所有權
    // 通常用於轉換，呼叫後原值不能再使用
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("面積: {}", rect.area());  // 借用

    let mut rect = Rectangle { width: 30, height: 50 };
    rect.resize(40, 60);                // 可變借用
    println!("新尺寸: {}x{}", rect.width, rect.height);

    let square = rect.into_square();    // 取得所有權
    // println!("{}", rect.width);      // 錯誤！rect 已被移動
    println!("正方形: {}x{}", square.width, square.height);
}
```

### 關聯函式（Associated Functions）

沒有 `self` 參數的函式，使用 `::` 呼叫：

```rust
impl Rectangle {
    // 關聯函式（類似其他語言的靜態方法）
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // 常見命名：new、default、from_xxx
    fn from_dimensions(dimensions: (u32, u32)) -> Rectangle {
        Rectangle {
            width: dimensions.0,
            height: dimensions.1,
        }
    }
}

fn main() {
    // 使用 :: 呼叫關聯函式
    let rect = Rectangle::new(30, 50);
    let square = Rectangle::square(10);
    let rect2 = Rectangle::from_dimensions((20, 40));

    println!("{:?}", rect);
}
```

### 多個 impl 區塊

一個型別可以有多個 `impl` 區塊：

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 常見用法：分離不同 trait 的實作
impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle({}x{})", self.width, self.height)
    }
}
```

### 自動引用和解引用

Rust 會自動添加 `&`、`&mut` 或 `*`：

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    // 這兩種寫法等價：
    let area1 = rect.area();
    let area2 = (&rect).area();

    // 編譯器自動推導需要 &、&mut 還是所有權
}
```

---

## 8.4 結構體的記憶體佈局

### 欄位對齊

結構體欄位在記憶體中會進行對齊：

```rust
use std::mem;

struct A {
    a: u8,   // 1 byte
    b: u32,  // 4 bytes
    c: u8,   // 1 byte
}

struct B {
    a: u8,   // 1 byte
    c: u8,   // 1 byte
    b: u32,  // 4 bytes
}

fn main() {
    println!("A 大小: {} bytes", mem::size_of::<A>());  // 可能是 12
    println!("B 大小: {} bytes", mem::size_of::<B>());  // 可能是 8

    // 原因：編譯器會插入填充以滿足對齊要求
    // A: [a: 1][padding: 3][b: 4][c: 1][padding: 3] = 12
    // B: [a: 1][c: 1][padding: 2][b: 4] = 8
}
```

### `#[repr]` 屬性

控制記憶體佈局：

```rust
// 預設：Rust 可以自由重排欄位以優化
struct Default {
    a: u8,
    b: u32,
    c: u8,
}

// C 相容佈局（FFI 時必須使用）
#[repr(C)]
struct CCompatible {
    a: u8,
    b: u32,
    c: u8,
}

// 指定對齊
#[repr(align(16))]
struct Aligned {
    data: [u8; 4],
}

// 緊湊佈局（無填充）
#[repr(packed)]
struct Packed {
    a: u8,
    b: u32,
    c: u8,
}

fn main() {
    use std::mem;

    println!("Default: {} bytes", mem::size_of::<Default>());     // 可能是 8
    println!("CCompatible: {} bytes", mem::size_of::<CCompatible>()); // 12
    println!("Aligned: {} bytes", mem::size_of::<Aligned>());     // 16
    println!("Packed: {} bytes", mem::size_of::<Packed>());       // 6
}
```

### 與 FFI 互動

```rust
// 與 C 程式碼互動時使用 repr(C)
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

extern "C" {
    fn c_function(point: *const Point);
}

// 也可以使用 cbindgen 自動生成 C 標頭檔
```

---

## 常用 Derive 巨集

```rust
// Debug：允許使用 {:?} 印出
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Clone：允許呼叫 .clone()
#[derive(Clone)]
struct Data {
    value: String,
}

// Copy：允許隱式複製（需要所有欄位都是 Copy）
#[derive(Copy, Clone)]
struct SmallData {
    a: i32,
    b: i32,
}

// PartialEq, Eq：允許使用 == 比較
#[derive(PartialEq, Eq)]
struct Id {
    value: u64,
}

// PartialOrd, Ord：允許比較大小
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Score {
    value: u32,
}

// Hash：允許作為 HashMap 的 key
#[derive(Hash, PartialEq, Eq)]
struct Key {
    id: String,
}

// Default：提供預設值
#[derive(Default)]
struct Config {
    debug: bool,        // false
    max_connections: u32,  // 0
}

// 組合使用
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct User {
    id: u64,
    name: String,
}
```

---

## 實戰範例

### 建構器模式（Builder Pattern）

```rust
#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    max_connections: u32,
    timeout: u64,
}

#[derive(Default)]
struct ServerBuilder {
    host: String,
    port: u16,
    max_connections: u32,
    timeout: u64,
}

impl ServerBuilder {
    fn new() -> Self {
        ServerBuilder {
            host: String::from("localhost"),
            port: 8080,
            max_connections: 100,
            timeout: 30,
        }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            timeout: self.timeout,
        }
    }
}

fn main() {
    let server = ServerBuilder::new()
        .host("0.0.0.0")
        .port(3000)
        .max_connections(1000)
        .build();

    println!("{:?}", server);
}
```

### 工廠方法

```rust
#[derive(Debug)]
struct Connection {
    host: String,
    port: u16,
    secure: bool,
}

impl Connection {
    fn new(host: &str, port: u16) -> Self {
        Connection {
            host: host.to_string(),
            port,
            secure: false,
        }
    }

    fn secure(host: &str, port: u16) -> Self {
        Connection {
            host: host.to_string(),
            port,
            secure: true,
        }
    }

    fn from_url(url: &str) -> Option<Self> {
        // 簡化的 URL 解析
        if url.starts_with("https://") {
            let host = url.trim_start_matches("https://");
            Some(Connection::secure(host, 443))
        } else if url.starts_with("http://") {
            let host = url.trim_start_matches("http://");
            Some(Connection::new(host, 80))
        } else {
            None
        }
    }
}

fn main() {
    let conn1 = Connection::new("localhost", 8080);
    let conn2 = Connection::secure("example.com", 443);
    let conn3 = Connection::from_url("https://rust-lang.org");

    println!("{:?}", conn1);
    println!("{:?}", conn2);
    println!("{:?}", conn3);
}
```

---

## 本章重點回顧

1. **結構體種類**
   - 具名欄位結構體：最常用
   - 元組結構體：用於 Newtype 模式
   - 單元結構體：用於標記

2. **實例化**
   - 欄位簡寫語法
   - 結構體更新語法 `..`
   - 整個實例的可變性

3. **方法與關聯函式**
   - `&self`、`&mut self`、`self`
   - 關聯函式使用 `::` 呼叫
   - 可以有多個 impl 區塊

4. **記憶體佈局**
   - 欄位對齊與填充
   - `#[repr(C)]` 用於 FFI
   - 使用 derive 派生常用 trait

---

## 練習題

### 練習 1：定義結構體

定義一個 `Book` 結構體並實作方法：

```rust
// 欄位：title, author, pages, price
// 方法：new、is_long（超過 300 頁）、discounted_price（打折後價格）
```

### 練習 2：建構器模式

為以下結構體實作建構器模式：

```rust
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}
```

### 練習 3：derive 巨集

為以下結構體添加適當的 derive：

```rust
struct Product {
    id: u64,
    name: String,
    price: f64,
}

// 需求：
// - 可以印出（Debug）
// - 可以比較相等（PartialEq）
// - 可以複製（Clone）
// - 可以作為 HashMap 的 key（Hash, Eq）
```

### 練習 4：實作 Display

為 Rectangle 實作 Display trait：

```rust
use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

// 實作 Display，輸出格式：「30x50 的矩形」
```

---

## 延伸閱讀

- [定義結構體](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [方法語法](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust 資料佈局](https://doc.rust-lang.org/reference/type-layout.html)

---

[← 上一章：生命週期](./07-lifetimes.md) | [下一章：列舉與模式匹配 →](./09-enums.md)
