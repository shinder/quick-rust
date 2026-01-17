# 第 11 章：泛型

> 泛型讓你的程式碼更靈活、可重用，同時保持型別安全

---

## 11.1 泛型基礎

泛型（Generics）是 Rust 中實現程式碼重用的核心機制。它允許你編寫可以處理多種型別的程式碼，而不需要為每種型別都寫一份重複的程式碼。

### 為什麼需要泛型？

假設我們需要找出整數切片中的最大值：

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

如果還需要處理 `f64` 型別，就必須寫另一個幾乎相同的函式：

```rust
fn largest_f64(list: &[f64]) -> &f64 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

這種重複的程式碼正是泛型要解決的問題。

### 泛型函式

使用泛型，我們可以將上面的程式碼統一成一個函式：

```rust
// T 是泛型型別參數
// PartialOrd 約束表示 T 必須可以比較大小
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大的數字是 {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大的字元是 {}", largest(&chars));
}
```

**泛型語法解釋**：
- `<T>` 宣告一個泛型型別參數，命名慣例是單個大寫字母
- `T: PartialOrd` 是 trait bound，表示 T 必須實作 `PartialOrd` trait
- 可以有多個型別參數：`<T, U, V>`

```rust
// 多個泛型參數
fn combine<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    let result = combine(42, "hello");
    println!("{:?}", result); // (42, "hello")
}
```

### 泛型結構體

結構體也可以使用泛型：

```rust
// 單一泛型參數
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // 錯誤！x 和 y 必須是相同型別
    // let mixed = Point { x: 5, y: 4.0 };
}
```

如果需要 `x` 和 `y` 可以是不同型別：

```rust
// 兩個泛型參數
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mixed = Point { x: 5, y: 4.0 };     // Point<i32, f64>
    let integer = Point { x: 5, y: 10 };     // Point<i32, i32>
    let float = Point { x: 1.0, y: 4.0 };    // Point<f64, f64>
}
```

### 泛型列舉

Rust 標準庫中最常見的泛型列舉是 `Option<T>` 和 `Result<T, E>`：

```rust
// 標準庫的定義
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

自訂泛型列舉：

```rust
// 可能包含一個或兩個值
enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

// 帶有不同型別的列舉
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let one: OneOrTwo<i32> = OneOrTwo::One(42);
    let two: OneOrTwo<String> = OneOrTwo::Two(
        String::from("hello"),
        String::from("world"),
    );

    let left: Either<i32, &str> = Either::Left(42);
    let right: Either<i32, &str> = Either::Right("hello");
}
```

### 泛型方法

在結構體或列舉上定義泛型方法：

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 為所有 Point<T> 實作方法
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// 只為 Point<f64> 實作的方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point::new(3.0, 4.0);
    println!("x = {}", p.x());
    println!("距離原點: {}", p.distance_from_origin()); // 5.0

    let int_point = Point::new(1, 2);
    println!("x = {}", int_point.x());
    // int_point.distance_from_origin(); // 錯誤！Point<i32> 沒有這個方法
}
```

**混合泛型參數**：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 方法可以引入自己的泛型參數
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,   // 來自 self
            y: other.y,  // 來自 other
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
}
```

---

## 11.2 泛型約束

泛型約束（Trait Bounds）用於限制泛型型別必須具備某些能力。

### Trait Bounds 基礎語法

```rust
use std::fmt::Display;

// 語法 1：冒號後面直接寫約束
fn print_item<T: Display>(item: T) {
    println!("{}", item);
}

// 語法 2：使用 impl Trait（簡潔寫法）
fn print_item_v2(item: impl Display) {
    println!("{}", item);
}
```

### 多重約束

當需要多個 trait 約束時，使用 `+` 連接：

```rust
use std::fmt::{Display, Debug};

// 使用 + 連接多個約束
fn compare_and_display<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("{} 大於 {}", a, b);
    } else {
        println!("{} 小於或等於 {}", a, b);
    }
}

// 多個泛型參數各自有約束
fn process<T: Display + Clone, U: Debug>(item: T, other: U) {
    let cloned = item.clone();
    println!("item: {}", cloned);
    println!("other: {:?}", other);
}
```

### where 子句

當約束變得複雜時，使用 `where` 子句可以提高可讀性：

```rust
use std::fmt::{Display, Debug};

// 不使用 where（較難閱讀）
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
    42
}

// 使用 where（更清晰）
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
    42
}
```

**where 子句的優勢**：

```rust
use std::fmt::Debug;

// 可以表達更複雜的約束
fn process_pair<T, U>(pair: (T, U))
where
    T: Debug,
    U: Debug,
    (T, U): Debug,  // 對元組本身也有約束
{
    println!("{:?}", pair);
}

// 約束可以引用其他參數
fn convert<T, U>(value: T) -> U
where
    T: Into<U>,
{
    value.into()
}
```

### 有條件的方法實作

使用 trait bounds 可以只為滿足特定條件的型別實作方法：

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// 為所有 T 實作 new
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只為實作了 Display 和 PartialOrd 的 T 實作 cmp_display
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的是 x = {}", self.x);
        } else {
            println!("最大的是 y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(5, 10);
    pair.cmp_display(); // 正常運作

    // 如果 T 沒有實作 Display + PartialOrd，就無法呼叫 cmp_display
}
```

### Blanket Implementations（覆蓋實作）

為所有滿足特定條件的型別實作 trait：

```rust
use std::fmt::Display;

trait PrintInfo {
    fn print_info(&self);
}

// 為所有實作了 Display 的型別實作 PrintInfo
impl<T: Display> PrintInfo for T {
    fn print_info(&self) {
        println!("值是: {}", self);
    }
}

fn main() {
    42.print_info();      // 值是: 42
    "hello".print_info(); // 值是: hello
    3.14.print_info();    // 值是: 3.14
}
```

標準庫中著名的 blanket implementation：

```rust
// 任何實作了 Display 的型別自動實作 ToString
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // ...
    }
}
```

### 回傳實作特定 Trait 的型別

```rust
// 回傳型別使用 impl Trait
fn make_iterator() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}

// 這在回傳閉包時特別有用
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn main() {
    let iter = make_iterator();
    for x in iter {
        println!("{}", x);
    }

    let add_5 = make_adder(5);
    println!("{}", add_5(10)); // 15
}
```

**限制**：`impl Trait` 回傳型別在同一函式中必須是同一個具體型別：

```rust
// 錯誤！回傳不同的具體型別
fn get_iterator(flag: bool) -> impl Iterator<Item = i32> {
    if flag {
        vec![1, 2, 3].into_iter()
    } else {
        [4, 5, 6].into_iter()  // 錯誤：型別不同
    }
}

// 解決方案：使用 trait 物件（見第 12 章）
fn get_iterator_dyn(flag: bool) -> Box<dyn Iterator<Item = i32>> {
    if flag {
        Box::new(vec![1, 2, 3].into_iter())
    } else {
        Box::new([4, 5, 6].into_iter())
    }
}
```

---

## 11.3 單態化（Monomorphization）

Rust 的泛型實作採用「單態化」策略，這意味著泛型在編譯時期就會展開成具體的型別版本。

### 什麼是單態化？

```rust
fn identity<T>(x: T) -> T {
    x
}

fn main() {
    let a = identity(5);       // identity::<i32>
    let b = identity("hello"); // identity::<&str>
    let c = identity(3.14);    // identity::<f64>
}
```

編譯器會將上面的泛型函式展開成三個具體的函式：

```rust
// 編譯後的程式碼（概念示意）
fn identity_i32(x: i32) -> i32 { x }
fn identity_str(x: &str) -> &str { x }
fn identity_f64(x: f64) -> f64 { x }

fn main() {
    let a = identity_i32(5);
    let b = identity_str("hello");
    let c = identity_f64(3.14);
}
```

### 零成本抽象

單態化實現了 Rust 的「零成本抽象」理念：

```rust
// 泛型版本
fn sum<T>(values: &[T]) -> T
where
    T: std::iter::Sum + Copy,
{
    values.iter().copied().sum()
}

// 手寫的 i32 版本
fn sum_i32(values: &[i32]) -> i32 {
    values.iter().copied().sum()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 這兩個呼叫在編譯後完全相同的效能
    let result1 = sum(&numbers);
    let result2 = sum_i32(&numbers);
}
```

**效能比較**：
- 泛型版本：編譯後與手寫版本完全相同
- 無執行時期開銷
- 無虛擬函式呼叫
- 可以進行內聯最佳化

### 程式碼膨脹問題

單態化的代價是可能導致程式碼膨脹（code bloat）：

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn set(&mut self, value: T) {
        self.value = value;
    }

    // 假設有很多方法...
}

fn main() {
    let c1: Container<i32> = Container::new(1);
    let c2: Container<i64> = Container::new(2);
    let c3: Container<f32> = Container::new(3.0);
    let c4: Container<f64> = Container::new(4.0);
    let c5: Container<String> = Container::new(String::new());

    // 每種型別都會生成一份完整的 Container 實作
    // 如果方法很多，最終二進位檔會變大
}
```

### 減少程式碼膨脹的策略

**策略 1：使用 trait 物件（動態分派）**

```rust
// 泛型版本（靜態分派，會膨脹）
fn process_generic<T: Display>(items: Vec<T>) {
    for item in items {
        println!("{}", item);
    }
}

// trait 物件版本（動態分派，不膨脹）
fn process_dynamic(items: Vec<Box<dyn Display>>) {
    for item in items {
        println!("{}", item);
    }
}
```

**策略 2：將非泛型邏輯抽離**

```rust
struct Wrapper<T> {
    value: T,
    // 與 T 無關的欄位
    count: usize,
    name: String,
}

impl<T> Wrapper<T> {
    // 這個方法會為每個 T 生成一份
    fn get_value(&self) -> &T {
        &self.value
    }
}

// 將與 T 無關的邏輯移到非泛型 impl 中
impl<T> Wrapper<T> {
    // 這些方法實際上只會生成一份（最佳化後）
    fn get_count(&self) -> usize {
        self.count
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

// 或者更好的做法：使用內部非泛型結構
struct WrapperInner {
    count: usize,
    name: String,
}

struct BetterWrapper<T> {
    value: T,
    inner: WrapperInner,
}

impl WrapperInner {
    // 這些方法只有一份
    fn get_count(&self) -> usize { self.count }
    fn get_name(&self) -> &str { &self.name }
}
```

**策略 3：使用 `#[inline(never)]` 避免過度內聯**

```rust
impl<T: Display> Container<T> {
    #[inline(never)]  // 提示編譯器不要內聯這個函式
    fn complex_operation(&self) {
        // 複雜的操作...
    }
}
```

### 泛型 vs Trait 物件比較

| 特性 | 泛型（靜態分派） | Trait 物件（動態分派） |
|------|------------------|------------------------|
| 效能 | 最優（可內聯） | 有 vtable 查找開銷 |
| 二進位大小 | 可能較大 | 較小 |
| 靈活性 | 編譯時確定型別 | 執行時可混合型別 |
| 使用場景 | 效能關鍵路徑 | 需要型別抹除時 |

```rust
use std::fmt::Display;

// 靜態分派：編譯時確定型別
fn print_all_static<T: Display>(items: &[T]) {
    for item in items {
        println!("{}", item);
    }
}

// 動態分派：可以混合不同型別
fn print_all_dynamic(items: &[&dyn Display]) {
    for item in items {
        println!("{}", item);
    }
}

fn main() {
    // 靜態分派：所有元素必須是同一型別
    print_all_static(&[1, 2, 3]);
    print_all_static(&["a", "b", "c"]);

    // 動態分派：可以混合不同型別
    let items: Vec<&dyn Display> = vec![&1, &"hello", &3.14];
    print_all_dynamic(&items);
}
```

---

## 練習題

### 練習 1：泛型棧（Stack）

實作一個泛型的棧資料結構：

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // 實作以下方法
    fn new() -> Self { todo!() }
    fn push(&mut self, item: T) { todo!() }
    fn pop(&mut self) -> Option<T> { todo!() }
    fn peek(&self) -> Option<&T> { todo!() }
    fn is_empty(&self) -> bool { todo!() }
    fn len(&self) -> usize { todo!() }
}

// 只為實作了 Clone 的 T 實作
impl<T: Clone> Stack<T> {
    fn peek_clone(&self) -> Option<T> { todo!() }
}
```

### 練習 2：泛型 MinMax

實作一個能找出最小值和最大值的函式：

```rust
fn min_max<T: PartialOrd + Clone>(items: &[T]) -> Option<(T, T)> {
    // 回傳 (最小值, 最大值)，如果切片為空則回傳 None
    todo!()
}
```

### 練習 3：泛型 Pair 操作

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self { todo!() }
    fn swap(self) -> Pair<U, T> { todo!() }
}

// 當 T 和 U 相同時的特殊方法
impl<T: Clone> Pair<T, T> {
    fn duplicate_first(&self) -> Self { todo!() }
}
```

---

## 本章小結

- **泛型基礎**：泛型允許你編寫適用於多種型別的程式碼
- **泛型函式、結構體、列舉、方法**：都可以使用泛型參數
- **Trait Bounds**：用於限制泛型型別必須具備的能力
- **where 子句**：讓複雜的約束更易讀
- **單態化**：Rust 在編譯時將泛型展開為具體型別，實現零成本抽象
- **程式碼膨脹**：單態化可能導致二進位檔變大，可以透過 trait 物件等方式緩解

---

## 延伸閱讀

- [The Rust Book - Generic Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust By Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [Rust Performance Book - Monomorphization](https://nnethercote.github.io/perf-book/compile-times.html#monomorphization)
