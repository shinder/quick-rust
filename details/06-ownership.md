# 第 6 章：所有權系統

> 所有權是 Rust 最獨特的特性，也是理解 Rust 的關鍵。掌握所有權，你就掌握了 Rust 的精髓。

---

## 6.1 所有權基礎

### 為什麼需要所有權？

在大多數程式語言中，記憶體管理有兩種主要方式：

1. **手動管理**（C/C++）
   - 程式設計師負責 `malloc`/`free`
   - 容易出錯：記憶體洩漏、懸垂指標、重複釋放

2. **垃圾回收**（Java、Go、JavaScript）
   - 執行時自動回收
   - 有效能代價：GC 暫停、記憶體開銷

Rust 選擇第三條路：**所有權系統**
- 在編譯時期決定記憶體何時釋放
- 無 GC 開銷
- 編譯時期保證安全

### 所有權三大規則

```
1. Rust 中每個值都有一個變數，稱為其「所有者」（owner）
2. 同一時間只能有一個所有者
3. 當所有者離開作用域，值會被丟棄（drop）
```

```rust
fn main() {
    {                           // s 還未宣告，無效
        let s = String::from("hello");  // s 從這裡開始有效

        println!("{}", s);      // 使用 s

    }                           // 作用域結束，s 被丟棄（drop）

    // println!("{}", s);       // 錯誤！s 已經不存在
}
```

### 作用域與丟棄（Drop）

```rust
fn main() {
    let s1 = String::from("hello");  // s1 進入作用域

    {
        let s2 = String::from("world");  // s2 進入作用域
        println!("{} {}", s1, s2);
    }  // s2 離開作用域，被丟棄

    println!("{}", s1);  // s1 還可以用
    // println!("{}", s2);  // 錯誤！s2 已被丟棄

}  // s1 離開作用域，被丟棄
```

### Drop trait

當值離開作用域時，Rust 會自動呼叫 `drop` 方法：

```rust
struct CustomData {
    name: String,
}

impl Drop for CustomData {
    fn drop(&mut self) {
        println!("丟棄 CustomData: {}", self.name);
    }
}

fn main() {
    let data = CustomData {
        name: String::from("測試資料"),
    };
    println!("CustomData 已建立");

    // 作用域結束時自動呼叫 drop
}
// 輸出：
// CustomData 已建立
// 丟棄 CustomData: 測試資料
```

### 堆疊（Stack）vs 堆積（Heap）

理解記憶體配置對於理解所有權非常重要：

```
堆疊（Stack）              堆積（Heap）
┌─────────────┐           ┌─────────────────┐
│ 大小固定    │           │ 大小可變        │
│ 快速配置    │           │ 較慢配置        │
│ LIFO 順序   │           │ 任意順序        │
│ 自動清理    │           │ 需要手動管理    │
└─────────────┘           └─────────────────┘

堆疊存放：                 堆積存放：
- 基本型別 (i32, f64...)  - String
- 固定大小的陣列          - Vec
- 元組（若元素都在堆疊）   - Box<T>
- 指向堆積的指標          - 動態大小的資料
```

```rust
fn main() {
    // 堆疊配置
    let x = 5;           // i32，直接存在堆疊
    let arr = [1, 2, 3]; // 陣列，存在堆疊

    // 堆積配置
    let s = String::from("hello");  // 堆疊存指標，堆積存內容
    let v = vec![1, 2, 3];          // 同上

    // String 的記憶體佈局：
    //
    // 堆疊上的 s：          堆積上的資料：
    // ┌─────────────┐      ┌───┬───┬───┬───┬───┐
    // │ ptr ────────┼─────►│ h │ e │ l │ l │ o │
    // │ len: 5      │      └───┴───┴───┴───┴───┘
    // │ capacity: 5 │
    // └─────────────┘
}
```

---

## 6.2 移動語義（Move Semantics）

### 移動與複製

這是所有權最重要的概念之一：

```rust
fn main() {
    // 基本型別：複製
    let x = 5;
    let y = x;  // 複製值
    println!("x = {}, y = {}", x, y);  // 都可以用！

    // String：移動
    let s1 = String::from("hello");
    let s2 = s1;  // 所有權移動到 s2
    // println!("{}", s1);  // 錯誤！s1 已經無效
    println!("{}", s2);     // OK
}
```

**為什麼不同？**

```rust
// 基本型別存在堆疊，複製成本低
let x = 5;
let y = x;
// 堆疊：
// ┌───┐ ┌───┐
// │ 5 │ │ 5 │  ← 直接複製一份
// └───┘ └───┘
//   x     y

// String 存在堆積，複製成本高
let s1 = String::from("hello");
let s2 = s1;
// 如果「複製」：
// 堆疊：            堆積：
// ┌─────┐          ┌───────────┐
// │ ptr ├─────────►│  "hello"  │
// └─────┘          └───────────┘
// ┌─────┐               ▲
// │ ptr ├───────────────┘  ← 兩個指標指向同一資料！
// └─────┘                     離開作用域時會重複釋放！

// Rust 的做法：移動所有權
// 移動後：
// ┌─────┐
// │ s1  │  ← 無效
// └─────┘
// ┌─────┐          ┌───────────┐
// │ s2  ├─────────►│  "hello"  │
// └─────┘          └───────────┘
```

### Copy trait

實作 `Copy` trait 的型別會在賦值時自動複製：

```rust
fn main() {
    // 這些型別實作了 Copy：
    let a: i32 = 42;       // 所有整數型別
    let b: f64 = 3.14;     // 所有浮點數型別
    let c: bool = true;    // 布林
    let d: char = 'A';     // 字元
    let e: (i32, i32) = (1, 2);  // 元素都是 Copy 的元組
    let f: [i32; 3] = [1, 2, 3]; // 元素是 Copy 的陣列

    // 這些型別不是 Copy：
    let s: String = String::from("hello");
    let v: Vec<i32> = vec![1, 2, 3];
    // 任何需要堆積配置或有 Drop 的型別
}
```

**自訂型別實作 Copy：**

```rust
// 可以實作 Copy（所有欄位都是 Copy）
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// 不能實作 Copy（包含 String）
// #[derive(Copy, Clone)]  // 錯誤！
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;  // 複製
    println!("p1: ({}, {})", p1.x, p1.y);  // OK
    println!("p2: ({}, {})", p2.x, p2.y);  // OK
}
```

### Clone trait

`Clone` 提供明確的深層複製：

```rust
fn main() {
    let s1 = String::from("hello");

    // 明確複製
    let s2 = s1.clone();

    // 兩個都可以用
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // Vec 也一樣
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
}
```

**Copy vs Clone：**

| 特性 | Copy | Clone |
|------|------|-------|
| 呼叫方式 | 隱式（自動） | 顯式（`.clone()`） |
| 效能 | 保證快速（位元複製） | 可能昂貴 |
| 語義 | 簡單複製 | 可自訂複製行為 |
| 要求 | 型別必須全部在堆疊 | 無限制 |

### 函式與所有權

傳遞參數時也會發生移動或複製：

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);         // s 的所有權移動到函式
    // println!("{}", s);       // 錯誤！s 已經無效

    let x = 5;
    makes_copy(x);              // x 複製一份給函式
    println!("x = {}", x);      // OK！x 還可以用
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string 離開作用域，被丟棄

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer 離開作用域，但因為是 Copy，原值不受影響
```

**回傳值與所有權：**

```rust
fn main() {
    let s1 = gives_ownership();         // 函式回傳值的所有權移動給 s1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 移動到函式，函式回傳值移動給 s3
    // println!("{}", s2);               // 錯誤！s2 已經無效
    println!("{}", s3);                  // OK
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string  // 所有權移動給呼叫者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 直接回傳，所有權移動
}
```

---

## 6.3 借用與引用

每次都移動所有權太麻煩了。借用讓我們使用值而不取得所有權。

### 不可變借用 `&T`

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // 借用 s1

    println!("'{}' 的長度是 {}", s1, len);  // s1 還可以用！
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s 離開作用域，但因為沒有所有權，不會丟棄資料
```

**記憶體示意：**

```
    s1                     s（引用）
┌─────────────┐          ┌─────────────┐
│ ptr ────────┼──┐       │ ptr ────────┼──┐
│ len: 5      │  │       └─────────────┘  │
│ capacity: 5 │  │            │           │
└─────────────┘  │            └───────────┘
                 │                  │
                 ▼                  │
              ┌───┬───┬───┬───┬───┐│
              │ h │ e │ l │ l │ o ││
              └───┴───┴───┴───┴───┘│
                        ▲          │
                        └──────────┘
```

### 可變借用 `&mut T`

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);  // "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 借用規則

**核心規則：**

```
在任何時刻，你可以擁有：
- 一個可變引用（&mut T），或
- 任意數量的不可變引用（&T）

但不能同時擁有兩者。
```

```rust
fn main() {
    let mut s = String::from("hello");

    // 規則 1：多個不可變引用 OK
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);  // OK

    // 規則 2：一個可變引用 OK
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);  // OK

    // 規則 3：不能同時有可變和不可變引用
    let r4 = &s;
    // let r5 = &mut s;  // 錯誤！
    // println!("{}, {}", r4, r5);
    println!("{}", r4);
}
```

**非詞法作用域生命週期（NLL）：**

Rust 1.31+ 引入了更聰明的借用檢查：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1 和 r2 的最後使用點在這裡

    // 因為 r1, r2 不再使用，可以建立可變引用
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}
```

### 懸垂引用（Dangling References）

Rust 編譯器保證不會有懸垂引用：

```rust
fn main() {
    // let reference = dangle();  // 錯誤！
    let string = no_dangle();     // OK
}

// 錯誤：回傳對局部變數的引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // 錯誤！s 在函式結束時被丟棄
// }

// 正確：回傳所有權
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 移動所有權
}
```

### 借用的實際應用

```rust
// 不好：取得所有權再還回來
fn bad_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // 必須把 s 還回去
}

// 好：借用
fn good_length(s: &String) -> usize {
    s.len()
}

// 更好：使用切片（更通用）
fn best_length(s: &str) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello");

    // 不好的方式
    let (s, len) = bad_length(s);

    // 好的方式
    let len = good_length(&s);

    // 最好的方式
    let len = best_length(&s);      // String
    let len = best_length("hello"); // &str
}
```

---

## 6.4 切片（Slices）

切片是對連續序列一部分的引用，不擁有資料。

### 字串切片 `&str`

```rust
fn main() {
    let s = String::from("hello world");

    // 建立切片
    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"

    // 語法糖
    let hello = &s[..5];    // 從頭開始
    let world = &s[6..];    // 到結尾
    let whole = &s[..];     // 整個字串

    println!("{} {}", hello, world);

    // 字串字面值就是切片
    let s: &str = "hello";  // s 是 &str，指向二進位檔中的資料
}
```

**String vs &str：**

```
String（擁有所有權）      &str（借用/切片）
┌─────────────┐          ┌─────────────┐
│ ptr ────────┼───┐      │ ptr ────────┼───┐
│ len: 11     │   │      │ len: 5      │   │
│ capacity: 11│   │      └─────────────┘   │
└─────────────┘   │            │           │
                  ▼            │           │
        堆積：                  ▼           │
        ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
        │ h │ e │ l │ l │ o │   │ w │ o │ r │ l │ d │
        └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
                                │
                                └─ &str 指向這裡
```

### 陣列切片 `&[T]`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];  // [2, 3]
    println!("{:?}", slice);

    // 切片作為函式參數
    fn sum(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }

    // 可以接受陣列的切片
    let arr = [1, 2, 3, 4, 5];
    println!("陣列總和: {}", sum(&arr));
    println!("部分總和: {}", sum(&arr[1..4]));

    // 也可以接受 Vec 的切片
    let vec = vec![1, 2, 3, 4, 5];
    println!("Vec 總和: {}", sum(&vec));
}
```

### 切片作為函式參數

使用切片讓函式更通用：

```rust
// 不好：只能接受 String
fn first_word_bad(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 好：可以接受 String 和 &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn main() {
    let my_string = String::from("hello world");

    // 可以接受 String 的切片
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);  // 自動轉換

    // 也可以接受字串字面值
    let word = first_word("hello world");
}
```

### 可變切片

```rust
fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    // 可變切片
    let slice = &mut arr[1..4];
    slice[0] = 20;
    slice[1] = 30;
    slice[2] = 40;

    println!("{:?}", arr);  // [1, 20, 30, 40, 5]

    // 對切片排序
    let mut numbers = [3, 1, 4, 1, 5, 9, 2, 6];
    let slice = &mut numbers[2..6];
    slice.sort();
    println!("{:?}", numbers);  // [3, 1, 1, 4, 5, 9, 2, 6]
}
```

---

## 本章重點回顧

1. **所有權規則**
   - 每個值有一個所有者
   - 一次只能有一個所有者
   - 所有者離開作用域，值被丟棄

2. **移動與複製**
   - Copy 型別：自動複製（基本型別）
   - 非 Copy 型別：移動所有權
   - Clone：明確深層複製

3. **借用規則**
   - 多個不可變引用 `&T` 或
   - 一個可變引用 `&mut T`
   - 不能同時存在

4. **切片**
   - 對序列的引用，不擁有資料
   - `&str`：字串切片
   - `&[T]`：陣列/Vec 切片
   - 讓函式更通用

---

## 練習題

### 練習 1：所有權轉移

預測以下程式碼的輸出，或說明編譯錯誤：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();

    // 哪些可以印出？
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);
}
```

### 練習 2：借用規則

修復以下程式碼的編譯錯誤：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println!("{}, {}, {}", r1, r2, r3);
}
```

### 練習 3：切片練習

實作一個函式，回傳字串中的最後一個單字：

```rust
fn last_word(s: &str) -> &str {
    // 你的程式碼
}

fn main() {
    assert_eq!(last_word("hello world"), "world");
    assert_eq!(last_word("hello"), "hello");
    assert_eq!(last_word(""), "");
}
```

### 練習 4：綜合應用

實作一個函式，接受切片並回傳第二大的數：

```rust
fn second_largest(numbers: &[i32]) -> Option<i32> {
    // 你的程式碼
}

fn main() {
    assert_eq!(second_largest(&[1, 2, 3, 4, 5]), Some(4));
    assert_eq!(second_largest(&[5]), None);
    assert_eq!(second_largest(&[]), None);
}
```

---

## 延伸閱讀

- [所有權](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [引用與借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [切片](https://doc.rust-lang.org/book/ch04-03-slices.html)

---

[← 上一章：函式與閉包](./05-functions.md) | [下一章：生命週期 →](./07-lifetimes.md)
