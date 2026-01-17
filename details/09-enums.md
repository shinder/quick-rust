# 第 9 章：列舉與模式匹配

> Rust 的列舉是代數資料型別，比 C 的列舉強大許多，結合模式匹配可以寫出優雅且安全的程式碼。

---

## 9.1 列舉定義

### 簡單列舉

```rust
// 定義簡單列舉
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;

    match dir {
        Direction::North => println!("往北"),
        Direction::South => println!("往南"),
        Direction::East => println!("往東"),
        Direction::West => println!("往西"),
    }
}
```

### 帶資料的變體

Rust 列舉的強大之處：每個變體可以帶有不同的資料：

```rust
// 變體可以帶有不同的資料
enum Message {
    Quit,                         // 無資料
    Move { x: i32, y: i32 },      // 具名欄位
    Write(String),                // 單一值
    ChangeColor(i32, i32, i32),   // 多個值
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in messages {
        match msg {
            Message::Quit => {
                println!("退出訊息");
            }
            Message::Move { x, y } => {
                println!("移動到 ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("寫入: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("顏色: RGB({}, {}, {})", r, g, b);
            }
        }
    }
}
```

**等效的結構體寫法：**

```rust
// 如果不用列舉，需要定義多個結構體
struct QuitMessage;
struct MoveMessage { x: i32, y: i32 }
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// 列舉把這些整合成單一型別
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### 為列舉實作方法

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移動到 ({}, {})", x, y),
            Message::Write(s) => println!("訊息: {}", s),
            Message::ChangeColor(r, g, b) => println!("顏色: ({}, {}, {})", r, g, b),
        }
    }

    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

fn main() {
    let msg = Message::Write(String::from("Hello"));
    msg.call();
}
```

### 類 C 列舉

可以為變體指定數值：

```rust
// 預設從 0 開始
enum Number {
    Zero,  // 0
    One,   // 1
    Two,   // 2
}

// 指定數值
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

// 使用 repr 控制大小
#[repr(u8)]
enum SmallEnum {
    A = 0,
    B = 1,
    C = 2,
}

fn main() {
    println!("Ok = {}", HttpStatus::Ok as i32);  // 200

    // 從數值轉換回列舉需要手動實作或使用 crate
}
```

### 泛型列舉

```rust
// 泛型列舉
enum Container<T> {
    Empty,
    Single(T),
    Multiple(Vec<T>),
}

fn main() {
    let empty: Container<i32> = Container::Empty;
    let single = Container::Single(42);
    let multiple = Container::Multiple(vec![1, 2, 3]);

    match single {
        Container::Empty => println!("空的"),
        Container::Single(v) => println!("單一值: {}", v),
        Container::Multiple(v) => println!("多個值: {:?}", v),
    }
}
```

---

## 9.2 Option 類型

`Option` 是 Rust 最重要的列舉之一，用於表示「可能有值或沒有值」。

### 定義

```rust
// Option 的定義（標準庫）
enum Option<T> {
    Some(T),  // 有值
    None,     // 沒有值
}
```

### 基本使用

```rust
fn main() {
    // 建立 Option
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    // 型別推導
    let some_string = Some("hello");

    // None 需要型別標註
    let absent_number: Option<i32> = None;
}
```

### 為什麼使用 Option？

Rust 沒有 `null`，用 `Option` 取代：

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    let user = find_user(1);

    // 必須處理 None 的情況
    match user {
        Some(name) => println!("找到用戶: {}", name),
        None => println!("用戶不存在"),
    }

    // 不能直接使用 Option<T> 作為 T
    // let len = user.len();  // 錯誤！
}
```

### 常用方法

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    // 檢查
    println!("x.is_some(): {}", x.is_some());  // true
    println!("x.is_none(): {}", x.is_none());  // false

    // 取值（危險，None 會 panic）
    // let v = y.unwrap();  // panic!

    // 取值（提供錯誤訊息）
    // let v = y.expect("應該有值");  // panic with message

    // 取值或預設值
    let v = y.unwrap_or(0);  // 0
    let v = y.unwrap_or_default();  // 0（使用 Default trait）
    let v = y.unwrap_or_else(|| {
        // 懶惰計算預設值
        expensive_computation()
    });

    // 轉換
    let doubled = x.map(|n| n * 2);  // Some(10)
    println!("doubled: {:?}", doubled);

    // 鏈式轉換
    let result = x
        .map(|n| n * 2)      // Some(10)
        .map(|n| n + 1)      // Some(11)
        .filter(|&n| n > 5); // Some(11)
    println!("result: {:?}", result);

    // and_then（flatMap）
    let parsed = Some("42")
        .and_then(|s| s.parse::<i32>().ok());  // Some(42)

    // 取值並提供閉包計算預設值
    let v = y.map_or(0, |n| n * 2);  // 0
    let v = y.map_or_else(|| 0, |n| n * 2);  // 0
}

fn expensive_computation() -> i32 {
    println!("計算中...");
    42
}
```

### Option 與引用

```rust
fn main() {
    let text = Some(String::from("hello"));

    // as_ref：Option<T> -> Option<&T>
    let text_ref = text.as_ref();
    if let Some(s) = text_ref {
        println!("長度: {}", s.len());
    }
    // text 還可以用
    println!("{:?}", text);

    // as_mut：Option<T> -> Option<&mut T>
    let mut text = Some(String::from("hello"));
    if let Some(s) = text.as_mut() {
        s.push_str(" world");
    }
    println!("{:?}", text);  // Some("hello world")

    // take：取走值，留下 None
    let mut opt = Some(42);
    let taken = opt.take();  // Some(42)
    println!("opt: {:?}", opt);    // None
    println!("taken: {:?}", taken); // Some(42)

    // replace：替換值
    let mut opt = Some(42);
    let old = opt.replace(100);  // Some(42)
    println!("opt: {:?}", opt);  // Some(100)
}
```

---

## 9.3 Result 類型

`Result` 用於表示可能失敗的操作。

### 定義

```rust
// Result 的定義（標準庫）
enum Result<T, E> {
    Ok(T),   // 成功，帶有值
    Err(E),  // 失敗，帶有錯誤
}
```

### 基本使用

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("不能除以零"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(value) => println!("結果: {}", value),
        Err(e) => println!("錯誤: {}", e),
    }
}
```

### 錯誤傳遞 `?` 運算子

```rust
use std::fs::File;
use std::io::{self, Read};

// 手動處理
fn read_file_manual(path: &str) -> Result<String, io::Error> {
    let file = File::open(path);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

// 使用 ? 運算子（簡潔版）
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 更簡潔
fn read_file_short(path: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}
```

### 常用方法

```rust
fn main() {
    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("出錯了");

    // 檢查
    println!("ok.is_ok(): {}", ok.is_ok());    // true
    println!("ok.is_err(): {}", ok.is_err());  // false

    // 轉換為 Option
    let opt = ok.ok();   // Some(42)
    let opt = ok.err();  // None

    // 取值
    let v = ok.unwrap();           // 42
    let v = err.unwrap_or(0);      // 0
    let v = err.unwrap_or_default(); // 0

    // map：轉換成功值
    let doubled = ok.map(|n| n * 2);  // Ok(84)

    // map_err：轉換錯誤
    let new_err = err.map_err(|e| format!("錯誤: {}", e));

    // and_then：鏈式操作
    let result = ok
        .and_then(|n| if n > 0 { Ok(n * 2) } else { Err("必須是正數") })
        .and_then(|n| Ok(n + 1));

    // or_else：處理錯誤
    let recovered = err.or_else(|_| Ok(0));  // Ok(0)
}
```

### Result 與 Option 轉換

```rust
fn main() {
    // Result -> Option
    let ok: Result<i32, &str> = Ok(42);
    let opt = ok.ok();  // Some(42)

    let err: Result<i32, &str> = Err("error");
    let opt = err.ok();  // None

    // Option -> Result
    let some = Some(42);
    let result = some.ok_or("沒有值");  // Ok(42)

    let none: Option<i32> = None;
    let result = none.ok_or("沒有值");  // Err("沒有值")

    // 懶惰版本
    let result = none.ok_or_else(|| expensive_error());
}

fn expensive_error() -> String {
    String::from("計算出的錯誤")
}
```

---

## 9.4 模式匹配進階

### 解構列舉

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

fn main() {
    let msg = Message::ChangeColor(Color::Rgb(255, 128, 0));

    match msg {
        Message::Quit => {
            println!("退出");
        }
        Message::Move { x, y } => {
            println!("移動到 ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("訊息: {}", text);
        }
        // 巢狀解構
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: ({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: ({}, {}, {})", h, s, v);
        }
    }
}
```

### 巢狀模式

```rust
fn main() {
    let numbers = (Some(1), Some(2), None, Some(4));

    match numbers {
        (Some(a), Some(b), Some(c), Some(d)) => {
            println!("全部有值: {}, {}, {}, {}", a, b, c, d);
        }
        (Some(a), .., Some(d)) => {
            println!("首尾有值: {} ... {}", a, d);
        }
        (Some(a), ..) => {
            println!("至少第一個有值: {}", a);
        }
        _ => {
            println!("其他情況");
        }
    }

    // 巢狀 Option
    let nested = Some(Some(42));
    if let Some(Some(value)) = nested {
        println!("值: {}", value);
    }
}
```

### 範圍模式

```rust
fn main() {
    let x = 5;

    match x {
        1..=5 => println!("1 到 5"),
        6..=10 => println!("6 到 10"),
        _ => println!("其他"),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("a 到 j"),
        'k'..='z' => println!("k 到 z"),
        _ => println!("其他"),
    }
}
```

### 參考模式

```rust
fn main() {
    let robot_name = Some(String::from("Bors"));

    // 不獲取所有權
    match &robot_name {
        Some(name) => println!("名字: {}", name),
        None => println!("沒有名字"),
    }
    // robot_name 還可以用
    println!("{:?}", robot_name);

    // ref 關鍵字（舊寫法，現在較少用）
    match robot_name {
        Some(ref name) => println!("名字: {}", name),
        None => println!("沒有名字"),
    }

    // ref mut
    let mut robot_name = Some(String::from("Bors"));
    match &mut robot_name {
        Some(name) => name.push_str(" the Great"),
        None => (),
    }
    println!("{:?}", robot_name);  // Some("Bors the Great")
}
```

### @ 綁定

```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("id 在範圍 3-7 內: {}", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            println!("id 在範圍 10-12 內");
        }
        Message::Hello { id } => {
            println!("其他 id: {}", id);
        }
    }

    // 搭配 Option
    let x = Some(5);
    match x {
        Some(n @ 1..=10) => println!("1-10 範圍: {}", n),
        Some(n) => println!("其他: {}", n),
        None => println!("沒有值"),
    }
}
```

### if let 和 while let

```rust
fn main() {
    let some_value = Some(42);

    // if let：處理單一模式
    if let Some(x) = some_value {
        println!("值: {}", x);
    } else {
        println!("沒有值");
    }

    // while let：迴圈直到模式不匹配
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("彈出: {}", top);
    }

    // let else（Rust 1.65+）
    fn process(opt: Option<i32>) -> i32 {
        let Some(value) = opt else {
            return 0;  // 必須發散
        };
        value * 2
    }
}
```

### matches! 巨集

```rust
fn main() {
    let x = Some(5);

    // 簡化布林判斷
    let is_some_5 = matches!(x, Some(5));
    println!("是否為 Some(5): {}", is_some_5);

    // 搭配範圍
    let c = 'c';
    let is_letter = matches!(c, 'a'..='z' | 'A'..='Z');

    // 搭配守衛
    let is_large = matches!(x, Some(n) if n > 3);

    // 實用範例
    enum Status {
        Active,
        Inactive,
        Pending,
    }

    let status = Status::Active;
    let is_active = matches!(status, Status::Active);
}
```

---

## 實戰範例

### 狀態機

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }

    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}
```

### 表達式求值器

```rust
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i32 {
        match self {
            Expr::Num(n) => *n,
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Sub(a, b) => a.eval() - b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
        }
    }
}

fn main() {
    // (2 + 3) * 4
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Num(3)),
        )),
        Box::new(Expr::Num(4)),
    );

    println!("結果: {}", expr.eval());  // 20
}
```

### JSON 值

```rust
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
}

impl JsonValue {
    fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    fn as_str(&self) -> Option<&str> {
        if let JsonValue::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    fn as_array(&self) -> Option<&Vec<JsonValue>> {
        if let JsonValue::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }
}
```

---

## 本章重點回顧

1. **列舉定義**
   - 變體可以帶有不同的資料
   - 可以為列舉實作方法
   - 泛型列舉

2. **Option**
   - `Some(T)` 或 `None`
   - 取代 null
   - 豐富的方法：map、and_then、unwrap_or

3. **Result**
   - `Ok(T)` 或 `Err(E)`
   - `?` 運算子傳遞錯誤
   - 與 Option 可互相轉換

4. **模式匹配**
   - 解構、巢狀、範圍模式
   - `@` 綁定
   - if let、while let、matches!

---

## 練習題

### 練習 1：定義列舉

定義一個 `Shape` 列舉並實作面積計算：

```rust
// 變體：Circle(f64), Rectangle(f64, f64), Triangle { base: f64, height: f64 }
// 實作 area() 方法
```

### 練習 2：Option 練習

```rust
// 實作安全的除法
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    // 你的程式碼
}

// 實作查找最大值
fn find_max(numbers: &[i32]) -> Option<&i32> {
    // 你的程式碼
}
```

### 練習 3：Result 鏈式操作

```rust
// 解析並驗證使用者輸入
fn parse_and_validate(input: &str) -> Result<i32, String> {
    // 1. 解析為數字
    // 2. 驗證範圍 1-100
    // 使用 ? 運算子和 map_err
}
```

### 練習 4：實作簡單計算器

```rust
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, &'static str> {
    // 實作四則運算，除以零回傳錯誤
}
```

---

## 延伸閱讀

- [列舉](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Option](https://doc.rust-lang.org/std/option/)
- [Result](https://doc.rust-lang.org/std/result/)
- [模式語法](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)

---

[← 上一章：結構體](./08-structs.md) | [下一章：錯誤處理 →](./10-error-handling.md)
