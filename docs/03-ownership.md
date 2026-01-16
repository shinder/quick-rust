# 第三章：所有權與借用

> 這是 Rust 最獨特、最重要的概念。理解所有權後，你就掌握了 Rust 的精髓。

## 為什麼需要所有權？

**JavaScript 的記憶體管理**：
- 垃圾回收器（GC）自動清理不用的記憶體
- 你不需要關心，但有效能代價

**Rust 的做法**：
- 沒有 GC，透過「所有權規則」在編譯時期管理記憶體
- 零執行時開銷，效能極佳
- 編譯器幫你檢查，不會有記憶體洩漏

## 所有權三大規則

```
1. Rust 中每個值都有一個「所有者」（owner）
2. 一次只能有一個所有者
3. 當所有者離開作用域，值會被丟棄（drop）
```

## 移動（Move）vs 複製（Copy）

### JavaScript 的行為

```javascript
// JavaScript - 基本型別是複製
let a = 5;
let b = a;  // 複製值
console.log(a, b);  // 5, 5 都可用

// JavaScript - 物件是參考
let obj1 = { name: "Alice" };
let obj2 = obj1;  // 共享參考
obj2.name = "Bob";
console.log(obj1.name);  // "Bob" - obj1 也被改了！
```

### Rust 的行為

```rust
// Rust - 基本型別也是複製（有 Copy trait）
let a = 5;
let b = a;  // 複製
println!("{}, {}", a, b);  // 5, 5 都可用 ✓

// Rust - String 會「移動」所有權
let s1 = String::from("hello");
let s2 = s1;  // s1 的所有權移動到 s2
// println!("{}", s1);  // 錯誤！s1 已經無效
println!("{}", s2);     // OK
```

### 視覺化理解

```
JavaScript 物件：
┌─────┐     ┌─────────┐
│ obj1├────►│ "Alice" │
└─────┘     └─────────┘
              ▲
┌─────┐       │
│ obj2├───────┘  （兩個變數指向同一個資料）
└─────┘

Rust String 移動後：
移動前：
┌────┐     ┌─────────┐
│ s1 ├────►│ "hello" │
└────┘     └─────────┘

移動後：
┌────┐
│ s1 │  （無效，不能再用）
└────┘
┌────┐     ┌─────────┐
│ s2 ├────►│ "hello" │
└────┘     └─────────┘
```

### 哪些型別會複製？

```rust
// 這些型別實作了 Copy trait，賦值時會複製：
let i: i32 = 42;       // 整數
let f: f64 = 3.14;     // 浮點數
let b: bool = true;    // 布林
let c: char = 'A';     // 字元
let t: (i32, i32) = (1, 2);  // 只含 Copy 型別的元組

// 這些型別會移動所有權：
let s: String = String::from("hello");  // String
let v: Vec<i32> = vec![1, 2, 3];        // Vec
```

## 克隆（Clone）：深層複製

如果你真的想要複製一份資料：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 深層複製

println!("{}, {}", s1, s2);  // 兩個都可用！
```

## 借用（Borrowing）

移動所有權太麻煩？大多數時候我們用「借用」：

### 不可變借用 `&`

```rust
fn main() {
    let s = String::from("hello");

    // 借給函式用，用完還回來
    let len = calculate_length(&s);

    println!("'{}' 的長度是 {}", s, len);  // s 還可以用！
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 離開作用域，但因為沒有所有權，不會丟棄資料
```

### 可變借用 `&mut`

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);  // "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### 借用規則

```
1. 可以有多個不可變借用 (&T)
2. 或者只有一個可變借用 (&mut T)
3. 不能同時有可變和不可變借用
```

```rust
let mut s = String::from("hello");

// OK：多個不可變借用
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);

// OK：一個可變借用
let r3 = &mut s;
r3.push_str(" world");

// 錯誤：同時有可變和不可變借用
let r1 = &s;
let r2 = &mut s;  // 編譯錯誤！
```

### 為什麼有這個限制？

防止「資料競爭」（data race）：

```rust
// 如果允許這樣，會發生什麼？
let mut data = vec![1, 2, 3];

let first = &data[0];     // 不可變借用
data.push(4);             // 可變借用（可能重新分配記憶體）
println!("{}", first);    // first 可能指向無效記憶體！

// Rust 編譯器會阻止這種情況
```

## 生命週期入門

生命週期確保借用不會比擁有者活得更久。

### 懸垂引用（Dangling Reference）

```rust
// 這在 Rust 中不可能發生：
fn dangle() -> &String {       // 錯誤！
    let s = String::from("hello");
    &s  // 回傳 s 的引用
}   // s 在這裡被丟棄，引用會指向無效記憶體

// 正確做法：回傳所有權
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 移動所有權出去
}
```

### 生命週期標註

有時編譯器需要你明確標註生命週期：

```rust
// 編譯器不知道回傳的引用來自 x 還是 y
fn longest(x: &str, y: &str) -> &str {  // 錯誤！
    if x.len() > y.len() { x } else { y }
}

// 加上生命週期標註
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` 讀作「生命週期 a」，表示：回傳值的生命週期與參數相同。

> 初學者提示：生命週期通常可以交給編譯器推導，遇到錯誤時再學習標註。

## 常見錯誤與解決方式

### 錯誤 1：移動後使用

```rust
// 錯誤
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // error: value borrowed after move

// 解法 1：使用 clone
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}", s1);  // OK

// 解法 2：使用借用
let s1 = String::from("hello");
let s2 = &s1;
println!("{}", s1);  // OK
```

### 錯誤 2：同時可變和不可變借用

```rust
// 錯誤
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);
println!("{}", first);  // error

// 解法：確保不可變借用結束後再修改
let mut v = vec![1, 2, 3];
let first = v[0];  // 複製值而非借用
v.push(4);
println!("{}", first);  // OK
```

### 錯誤 3：回傳局部變數的引用

```rust
// 錯誤
fn get_string() -> &String {
    let s = String::from("hello");
    &s  // error: returns a reference to data owned by the current function
}

// 解法：回傳所有權
fn get_string() -> String {
    String::from("hello")
}
```

## JS vs Rust：記憶體管理對比

| 情境 | JavaScript | Rust |
|------|------------|------|
| 基本型別賦值 | 複製值 | 複製值（Copy trait） |
| 物件/陣列賦值 | 共享參考 | 移動所有權 |
| 函式傳參 | 複製/參考 | 移動或借用 |
| 記憶體釋放 | GC 自動回收 | 離開作用域自動丟棄 |
| 資料競爭防護 | 無（要小心） | 編譯時保證 |

## 練習題

### 練習 1：所有權追蹤
以下程式碼能否編譯？為什麼？

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

### 練習 2：修復借用錯誤
修復以下程式碼：

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, {}", r1, r2, r3);
}
```

### 練習 3：實作函式
完成以下函式，不要取得字串的所有權：

```rust
// 回傳字串的第一個單字
fn first_word(s: /* 填入型別 */) -> &str {
    // 你的程式碼
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一個單字: {}", word);
    println!("原始字串: {}", s);  // s 應該還能用
}
```

---

[← 上一章：基本語法](./02-basics.md) | [下一章：結構體與列舉 →](./04-structs-enums.md)
