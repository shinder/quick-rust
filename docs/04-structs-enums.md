# 第四章：結構體與列舉

## 結構體（Struct）

結構體類似 JavaScript 的物件或 class。

### 定義與建立

```javascript
// JavaScript
class User {
    constructor(name, email, age) {
        this.name = name;
        this.email = email;
        this.age = age;
        this.active = true;
    }
}

const user = new User("Alice", "alice@example.com", 30);
```

```rust
// Rust
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

// 建立實例
let user = User {
    name: String::from("Alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};
```

### 欄位簡寫

```rust
fn create_user(name: String, email: String) -> User {
    User {
        name,       // 等同於 name: name
        email,      // 等同於 email: email
        age: 25,
        active: true,
    }
}
```

### 結構體更新語法

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    ..user  // 其餘欄位從 user 複製（注意：String 會移動）
};
```

### 存取與修改欄位

```rust
// 存取
println!("{}", user.name);

// 修改（整個結構體必須是 mut）
let mut user = User { /* ... */ };
user.age = 31;
```

## 實作方法（impl）

```javascript
// JavaScript
class Rectangle {
    constructor(width, height) {
        this.width = width;
        this.height = height;
    }

    area() {
        return this.width * this.height;
    }

    static square(size) {
        return new Rectangle(size, size);
    }
}
```

```rust
// Rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法：第一個參數是 self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可變方法
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    // 關聯函式（類似靜態方法，沒有 self）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 使用
let rect = Rectangle { width: 30, height: 50 };
println!("面積: {}", rect.area());

let square = Rectangle::square(10);  // 注意：用 :: 呼叫
```

### self 的三種形式

```rust
impl MyStruct {
    fn take_ownership(self) { }     // 取得所有權，呼叫後不能再用
    fn borrow(&self) { }            // 不可變借用，唯讀
    fn borrow_mut(&mut self) { }    // 可變借用，可修改
}
```

## 元組結構體

當你只需要型別區分，不需要欄位名稱：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// 存取用索引
println!("R: {}", black.0);
```

## 列舉（Enum）

Rust 的列舉比 JS 強大很多！

### 基本列舉

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Up;

match dir {
    Direction::Up => println!("往上"),
    Direction::Down => println!("往下"),
    Direction::Left => println!("往左"),
    Direction::Right => println!("往右"),
}
```

### 帶資料的列舉

這是 Rust 列舉的殺手功能！

```javascript
// JavaScript 可能這樣表示
const message = {
    type: "Move",
    x: 10,
    y: 20
};
// 或
const message = {
    type: "Write",
    text: "Hello"
};
```

```rust
// Rust 列舉可以直接帶資料
enum Message {
    Quit,                       // 無資料
    Move { x: i32, y: i32 },    // 具名欄位
    Write(String),              // 單一值
    ChangeColor(i32, i32, i32), // 多個值
}

let msg = Message::Move { x: 10, y: 20 };
let msg2 = Message::Write(String::from("Hello"));

// 用 match 解構
match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移動到 ({}, {})", x, y),
    Message::Write(text) => println!("訊息: {}", text),
    Message::ChangeColor(r, g, b) => println!("顏色: ({}, {}, {})", r, g, b),
}
```

### 為列舉實作方法

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Write(text) => println!("{}", text),
            _ => println!("其他訊息"),
        }
    }
}
```

## Option：取代 null/undefined

Rust 沒有 null！用 `Option` 表示可能沒有值：

```rust
enum Option<T> {
    Some(T),  // 有值
    None,     // 沒有值
}
```

### 使用 Option

```javascript
// JavaScript
function findUser(id) {
    const user = database[id];
    return user || null;  // 可能是 null
}

const user = findUser(1);
if (user) {
    console.log(user.name);
}
```

```rust
// Rust
fn find_user(id: u32) -> Option<User> {
    // 找到回傳 Some(user)，找不到回傳 None
    if id == 1 {
        Some(User { name: String::from("Alice"), /* ... */ })
    } else {
        None
    }
}

let user = find_user(1);

// 方法 1：match
match user {
    Some(u) => println!("找到: {}", u.name),
    None => println!("找不到用戶"),
}

// 方法 2：if let（只處理 Some 的情況）
if let Some(u) = user {
    println!("找到: {}", u.name);
}

// 方法 3：常用方法
let name = user.map(|u| u.name);  // Option<String>
let name = user.unwrap();         // 取出值，None 會 panic
let name = user.unwrap_or(default_user);  // 取出值或預設
let name = user.expect("用戶不存在");     // 取出值，None 會 panic 並顯示訊息
```

### Option 的實用方法

```rust
let x: Option<i32> = Some(5);

// 檢查
x.is_some()  // true
x.is_none()  // false

// 轉換
x.map(|n| n * 2)           // Some(10)
x.and_then(|n| Some(n * 2)) // Some(10)，可以回傳 Option

// 取值
x.unwrap()                  // 5（None 會 panic）
x.unwrap_or(0)              // 5（None 回傳 0）
x.unwrap_or_else(|| 0)      // 5（None 執行閉包）

// 比較 JavaScript
// x?.map?.(n => n * 2) ?? 0
```

## Result：取代 try-catch

`Result` 表示可能成功或失敗的操作：

```rust
enum Result<T, E> {
    Ok(T),   // 成功，帶有值
    Err(E),  // 失敗，帶有錯誤
}
```

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除以零錯誤"))
    } else {
        Ok(a / b)
    }
}

let result = divide(10.0, 2.0);

match result {
    Ok(value) => println!("結果: {}", value),
    Err(e) => println!("錯誤: {}", e),
}
```

> 下一章會深入介紹錯誤處理！

## 模式匹配進階

### 解構結構體

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

let Point { x, y } = p;  // x = 0, y = 7

// 或重新命名
let Point { x: a, y: b } = p;  // a = 0, b = 7

// match 中解構
match p {
    Point { x: 0, y } => println!("在 y 軸上，y = {}", y),
    Point { x, y: 0 } => println!("在 x 軸上，x = {}", x),
    Point { x, y } => println!("點 ({}, {})", x, y),
}
```

### 匹配守衛

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("小於 5: {}", x),
    Some(x) => println!("大於等於 5: {}", x),
    None => (),
}
```

### @ 綁定

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("id 在範圍內: {}", id_variable)
    }
    Message::Hello { id: 10..=12 } => {
        println!("id 在另一個範圍")
    }
    Message::Hello { id } => {
        println!("其他 id: {}", id)
    }
}
```

## 從 JS 物件思維轉換

| JavaScript 概念  | Rust 對應            |
| ---------------- | -------------------- |
| `class`          | `struct` + `impl`    |
| `constructor`    | 關聯函式（如 `new`） |
| `this`           | `self`               |
| 物件字面值       | 結構體實例           |
| `null/undefined` | `Option<T>`          |
| 聯合型別         | `enum`               |
| `instanceof`     | `match` 模式匹配     |

## 練習題

### 練習 1：建立結構體

建立一個 `Book` 結構體，包含 title、author、pages，並實作：

- `new` 關聯函式
- `summary` 方法回傳書籍描述

### 練習 2：Option 練習

```rust
// 完成這個函式：找出陣列中的最大值
fn find_max(numbers: &[i32]) -> Option<i32> {
    // 你的程式碼
}
```

### 練習 3：列舉練習

定義一個 `Shape` 列舉，包含：

- `Circle { radius: f64 }`
- `Rectangle { width: f64, height: f64 }`
- `Triangle { base: f64, height: f64 }`

實作 `area` 方法計算面積。

---

[← 上一章：所有權與借用](./03-ownership.md) | [下一章：錯誤處理 →](./05-error-handling.md)
