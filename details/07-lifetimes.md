# 第 7 章：生命週期

> 生命週期是 Rust 確保引用有效的機制。初學者常感到困惑，但理解後會發現它其實很合理。

---

## 7.1 生命週期基礎

### 為什麼需要生命週期？

生命週期確保**引用不會比其指向的資料活得更久**。

```rust
fn main() {
    let r;                      // 宣告 r

    {
        let x = 5;
        r = &x;                 // r 引用 x
    }                           // x 離開作用域，被丟棄

    // println!("{}", r);       // 錯誤！r 是懸垂引用
}
```

編譯器會追蹤每個引用的生命週期，確保這種情況不會發生。

### 生命週期視覺化

```rust
fn main() {
    let x = 5;            // ----------+-- 'a
                          //           |
    let r = &x;           // --+-- 'b  |
                          //   |       |
    println!("{}", r);    //   |       |
                          // --+       |
}                         // ----------+

// 'b 的生命週期完全在 'a 之內，所以 r 引用 x 是安全的
```

```rust
fn main() {
    let r;                // ----------+-- 'a
                          //           |
    {                     //           |
        let x = 5;        // --+-- 'b  |
        r = &x;           //   |       |
    }                     // --+       |  ← x 在這裡被丟棄
                          //           |
    // println!("{}", r); //           |  ← r 指向無效記憶體！
}                         // ----------+

// 'b 比 'a 短，r 會變成懸垂引用
```

### 生命週期標註語法

生命週期標註使用 `'` 加上名稱（通常是 `'a`、`'b`）：

```rust
&i32        // 引用
&'a i32     // 帶有生命週期 'a 的引用
&'a mut i32 // 帶有生命週期 'a 的可變引用
```

**重要觀念：**
- 生命週期標註**不會改變**引用的實際生命週期
- 它只是告訴編譯器多個引用之間的**關係**
- 讓編譯器能夠驗證借用的有效性

### 編譯器的生命週期省略規則

在許多情況下，編譯器可以自動推導生命週期，不需要明確標註。

**省略規則：**

1. **規則 1**：每個引用參數獲得自己的生命週期
   ```rust
   fn foo(x: &i32)              // 推導為 fn foo<'a>(x: &'a i32)
   fn bar(x: &i32, y: &i32)     // 推導為 fn bar<'a, 'b>(x: &'a i32, y: &'b i32)
   ```

2. **規則 2**：如果只有一個輸入生命週期，它被賦予所有輸出
   ```rust
   fn foo(x: &i32) -> &i32      // 推導為 fn foo<'a>(x: &'a i32) -> &'a i32
   ```

3. **規則 3**：如果有 `&self` 或 `&mut self`，self 的生命週期賦予所有輸出
   ```rust
   impl Foo {
       fn bar(&self, x: &i32) -> &i32  // 回傳值使用 self 的生命週期
   }
   ```

---

## 7.2 函式中的生命週期

### 需要明確標註的情況

當編譯器無法推導時，需要明確標註：

```rust
// 錯誤：編譯器無法決定回傳值的生命週期來自 x 還是 y
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// 正確：明確標註
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("長字串");
    let string2 = String::from("短");

    let result = longest(&string1, &string2);
    println!("較長的是: {}", result);
}
```

**標註的意義：**
- `'a` 表示 x、y 和回傳值的生命週期是相關的
- 回傳值的生命週期等於 x 和 y 中較短的那個
- 這讓編譯器能夠驗證借用是否有效

### 生命週期與作用域

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("長長的字串");

    {
        let string2 = String::from("短");
        let result = longest(&string1, &string2);
        println!("較長的是: {}", result);  // OK
    }

    // 如果嘗試在這裡使用 result 會失敗
    // 因為 string2 已經被丟棄
}
```

### 輸入與輸出生命週期

```rust
// 輸出生命週期只與部分輸入相關
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x  // 只回傳 x，所以只需要 x 的生命週期
}

// 多個生命週期參數
fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("y: {}", y);
    x
}

fn main() {
    let a = String::from("aaa");
    let result;
    {
        let b = String::from("bbb");
        result = first(&a, &b);  // OK：result 只與 a 的生命週期相關
    }
    println!("{}", result);  // OK
}
```

### 回傳引用的限制

```rust
// 錯誤：不能回傳對局部變數的引用
fn invalid<'a>() -> &'a str {
    let s = String::from("hello");
    &s  // 錯誤！s 在函式結束時被丟棄
}

// 正確：回傳所有權
fn valid() -> String {
    String::from("hello")
}

// 正確：回傳靜態引用
fn static_ref() -> &'static str {
    "hello"  // 字串字面值有 'static 生命週期
}

// 正確：回傳輸入參數的引用
fn return_input<'a>(s: &'a str) -> &'a str {
    s
}
```

---

## 7.3 結構體中的生命週期

### 帶引用的結構體

當結構體包含引用時，必須標註生命週期：

```rust
// 錯誤：缺少生命週期標註
// struct ImportantExcerpt {
//     part: &str,
// }

// 正確
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("在很久很久以前，有一個小村莊...");
    let first_sentence = novel.split('，').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", excerpt.part);
}
```

**生命週期的意義：**
- `ImportantExcerpt<'a>` 表示結構體的實例不能比 `part` 引用的資料活得更久
- 編譯器會確保這個約束被滿足

### 結構體方法中的生命週期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 規則 3：&self 的生命週期賦予回傳值
    fn level(&self) -> i32 {
        3
    }

    // 規則 3 應用
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意：{}", announcement);
        self.part  // 回傳值生命週期與 self 相同
    }

    // 明確標註不同的生命週期
    fn with_announcement<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Part: {}", self.part);
        announcement
    }
}
```

### 多個生命週期參數

```rust
struct Context<'s, 'd> {
    source: &'s str,
    destination: &'d str,
}

impl<'s, 'd> Context<'s, 'd> {
    fn new(source: &'s str, destination: &'d str) -> Self {
        Context { source, destination }
    }

    fn source(&self) -> &'s str {
        self.source
    }

    fn destination(&self) -> &'d str {
        self.destination
    }
}
```

### 生命週期與泛型

```rust
use std::fmt::Display;

// 結合泛型、trait bounds 和生命週期
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告：{}", ann);
    if x.len() > y.len() { x } else { y }
}

// 結構體結合泛型和生命週期
struct Ref<'a, T: 'a> {
    data: &'a T,
}

// 或使用更現代的語法
struct Ref2<'a, T> {
    data: &'a T,
}
```

---

## 7.4 進階生命週期

### 靜態生命週期 `'static`

`'static` 表示引用可以在程式的整個執行期間有效：

```rust
// 字串字面值有 'static 生命週期
let s: &'static str = "我是靜態的";

// 靜態變數
static GREETING: &str = "Hello";

fn main() {
    // 'static 引用可以賦值給任何更短的生命週期
    let s: &str = "hello";  // 隱式 &'static str

    // Box::leak 可以創建 'static 引用
    let static_string: &'static str = Box::leak(String::from("leaked").into_boxed_str());
}
```

**常見的 `'static` 誤解：**

```rust
// 'static 不代表「活到程式結束」
// 它代表「可以活到程式結束」（如果需要的話）

// 錯誤：T: 'static 不代表 T 必須是引用
fn print_static<T: 'static + std::fmt::Display>(t: T) {
    println!("{}", t);
}

fn main() {
    let s = String::from("owned");
    print_static(s);  // OK！String 滿足 'static bound

    // 因為 String 不包含任何借用的引用
    // 它可以「活得足夠久」
}
```

### 生命週期子型別

較長的生命週期可以用在需要較短生命週期的地方：

```rust
fn main() {
    let r: &str;                    // 'a
    {
        let s: &'static str = "hello";  // 'static
        r = s;  // OK：'static >= 'a
    }
    println!("{}", r);  // OK
}
```

### 生命週期約束

```rust
// T 必須比 'a 活得久
fn print_ref<'a, T: 'a>(t: &'a T) {
    println!("{:?}", t);
}

// 多重約束
fn complex<'a, 'b, T>(x: &'a T, y: &'b T) -> &'a T
where
    'b: 'a,  // 'b 必須至少和 'a 一樣長
    T: std::fmt::Debug,
{
    println!("y: {:?}", y);
    x
}
```

### 高階 Trait Bounds（HRTB）

用於需要「任意生命週期」的情況：

```rust
// 普通寫法
fn call_with_ref<'a, F>(f: F, s: &'a str)
where
    F: Fn(&'a str),
{
    f(s);
}

// HRTB：f 可以接受任意生命週期的引用
fn call_with_ref_hrtb<F>(f: F, s: &str)
where
    F: for<'a> Fn(&'a str),
{
    f(s);
}

fn main() {
    call_with_ref_hrtb(|s| println!("{}", s), "hello");
}
```

### 匿名生命週期 `'_`

當生命週期很明顯時，可以使用 `'_`：

```rust
// 明確
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    // ...
}

// 使用匿名生命週期
impl Iterator for StrSplit<'_> {
    type Item = &str;  // 錯誤：這裡還是需要標註
}

// 在型別標註中
fn foo(x: &'_ i32) {}  // 等同於 fn foo<'a>(x: &'a i32)
```

---

## 生命週期實戰範例

### 字串分割迭代器

```rust
struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

fn main() {
    let haystack = "a b c d e";
    let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
    println!("{:?}", letters);  // ["a", "b", "c", "d", "e"]
}
```

### 快取結構

```rust
use std::collections::HashMap;

struct Cache<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Cache<'a> {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }

    fn insert(&mut self, key: &'a str, value: &'a str) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&&'a str> {
        self.data.get(key)
    }
}

fn main() {
    let mut cache = Cache::new();

    let key = String::from("name");
    let value = String::from("Alice");

    cache.insert(&key, &value);

    if let Some(v) = cache.get("name") {
        println!("找到: {}", v);
    }
}
```

---

## 本章重點回顧

1. **生命週期基礎**
   - 確保引用不會比資料活得更久
   - 使用 `'a` 語法標註
   - 省略規則減少手動標註

2. **函式中的生命週期**
   - 當編譯器無法推導時需要標註
   - 表示多個引用之間的關係
   - 回傳值的生命週期必須與某個輸入相關

3. **結構體中的生命週期**
   - 包含引用的結構體需要標註
   - 結構體實例不能比其引用的資料活得久
   - 方法可以使用結構體的生命週期

4. **進階概念**
   - `'static`：程式執行期間有效
   - 生命週期子型別
   - HRTB：任意生命週期

---

## 練習題

### 練習 1：基本生命週期

修復以下程式碼：

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

### 練習 2：結構體生命週期

完成以下結構體定義，使程式碼能編譯：

```rust
struct TextExcerpt {
    content: &str,
    author: &str,
}

impl TextExcerpt {
    fn new(content: &str, author: &str) -> TextExcerpt {
        TextExcerpt { content, author }
    }

    fn summary(&self) -> String {
        format!("「{}」 —— {}", self.content, self.author)
    }
}
```

### 練習 3：複雜生命週期

解釋為什麼以下程式碼無法編譯，並修復它：

```rust
fn get_first_word<'a>(s: &'a str) -> &'a str {
    let result = s.split_whitespace().next().unwrap();
    result
}

fn main() {
    let result;
    {
        let s = String::from("hello world");
        result = get_first_word(&s);
    }
    println!("{}", result);
}
```

### 練習 4：實作帶生命週期的迭代器

```rust
// 實作一個迭代器，回傳字串中所有子字串的引用
struct Substrings<'a> {
    // 你的欄位
}

impl<'a> Substrings<'a> {
    fn new(s: &'a str) -> Self {
        // 你的程式碼
    }
}

impl<'a> Iterator for Substrings<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // 你的程式碼
    }
}
```

---

## 延伸閱讀

- [生命週期](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [生命週期省略規則](https://doc.rust-lang.org/nomicon/lifetime-elision.html)
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

---

[← 上一章：所有權系統](./06-ownership.md) | [下一章：結構體 →](./08-structs.md)
