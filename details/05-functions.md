# 第 5 章：函式與閉包

> 函式是程式的基本組成單元，閉包則為函數式程式設計提供了強大的工具。

---

## 5.1 函式定義

### 基本語法

```rust
// 基本函式定義
fn function_name() {
    println!("這是一個函式");
}

// 帶參數的函式
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 帶回傳值的函式
fn add(a: i32, b: i32) -> i32 {
    a + b  // 沒有分號 = 回傳值
}

fn main() {
    function_name();
    greet("World");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
}
```

### 函式簽名

函式簽名包含：名稱、參數、回傳型別

```rust
// 完整的函式簽名
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // 函式主體
}

// 範例
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

// 無回傳值（回傳 unit 型別 ()）
fn print_info(message: &str) {
    println!("{}", message);
}

// 明確標註回傳 ()
fn print_info_explicit(message: &str) -> () {
    println!("{}", message);
}
```

### 參數與回傳值

```rust
fn main() {
    // 多個參數
    fn describe_person(name: &str, age: u32, height: f64) {
        println!("{} 是 {} 歲，身高 {} 公分", name, age, height);
    }

    describe_person("Alice", 25, 165.5);

    // 多個回傳值（使用元組）
    fn min_max(numbers: &[i32]) -> (i32, i32) {
        let min = *numbers.iter().min().unwrap();
        let max = *numbers.iter().max().unwrap();
        (min, max)
    }

    let numbers = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&numbers);
    println!("最小值: {}, 最大值: {}", min, max);

    // 回傳 Result
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("不能除以零"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("結果: {}", result),
        Err(e) => println!("錯誤: {}", e),
    }
}
```

### 表達式 vs 陳述式

這是 Rust 的重要概念：

```rust
fn main() {
    // 陳述式（Statement）：執行動作，不回傳值
    let x = 5;  // 這是陳述式
    // let y = (let x = 5);  // 錯誤！陳述式不回傳值

    // 表達式（Expression）：求值並回傳結果
    let y = {
        let x = 3;
        x + 1  // 沒有分號 = 表達式，回傳 4
    };
    println!("y = {}", y);  // 4

    // 加上分號變成陳述式
    let z = {
        let x = 3;
        x + 1;  // 有分號 = 陳述式，回傳 ()
    };
    // z 是 ()

    // 函式中的表達式
    fn five() -> i32 {
        5  // 表達式，回傳 5
    }

    fn five_with_semicolon() -> i32 {
        5;  // 陳述式，回傳 ()
        // 錯誤！預期 i32，得到 ()
    }
}
```

### 提前回傳（Early Return）

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);  // 提前回傳
        }
    }
    None  // 沒找到
}

fn validate_age(age: i32) -> Result<(), String> {
    if age < 0 {
        return Err(String::from("年齡不能為負"));
    }
    if age > 150 {
        return Err(String::from("年齡不合理"));
    }
    Ok(())
}

fn main() {
    let numbers = [1, 3, 5, 4, 7];
    println!("{:?}", find_first_even(&numbers));  // Some(4)

    match validate_age(25) {
        Ok(()) => println!("年齡有效"),
        Err(e) => println!("錯誤: {}", e),
    }
}
```

### 發散函式（Diverging Functions）

永不回傳的函式，回傳型別是 `!`：

```rust
// panic! 永不回傳
fn forever_panic() -> ! {
    panic!("這個函式永不回傳");
}

// 無限迴圈
fn forever_loop() -> ! {
    loop {
        // 永遠執行
    }
}

// 實用範例：unreachable 分支
fn get_value(option: Option<i32>) -> i32 {
    match option {
        Some(v) => v,
        None => panic!("不應該發生"),  // ! 可以強制轉換為任何型別
    }
}

// exit
fn terminate() -> ! {
    std::process::exit(1);
}
```

---

## 5.2 閉包（Closures）

閉包是可以捕獲環境變數的匿名函式。

### 閉包語法

```rust
fn main() {
    // 基本閉包
    let add = |a, b| a + b;
    println!("3 + 5 = {}", add(3, 5));

    // 帶型別標註
    let add_typed: fn(i32, i32) -> i32 = |a, b| a + b;

    // 多行閉包
    let complex = |x: i32| {
        let y = x * 2;
        let z = y + 10;
        z
    };
    println!("{}", complex(5));  // 20

    // 無參數閉包
    let greet = || println!("Hello!");
    greet();

    // 比較：函式 vs 閉包
    fn add_fn(a: i32, b: i32) -> i32 { a + b }
    let add_closure = |a: i32, b: i32| -> i32 { a + b };
    let add_short = |a, b| a + b;  // 型別推導
}
```

### 環境捕獲

閉包可以捕獲其定義環境中的變數：

```rust
fn main() {
    let x = 4;

    // 捕獲 x
    let equal_to_x = |z| z == x;

    println!("4 等於 x 嗎？{}", equal_to_x(4));  // true

    // 函式不能捕獲環境
    // fn equal_to_x_fn(z: i32) -> bool { z == x }  // 錯誤！
}
```

### 三種捕獲方式

閉包可以通過三種方式捕獲變數，對應三種 trait：

```rust
fn main() {
    // 1. Fn：不可變借用（&T）
    let x = vec![1, 2, 3];
    let print_x = || println!("{:?}", x);  // 借用 x
    print_x();
    print_x();  // 可以多次呼叫
    println!("{:?}", x);  // x 還可以用

    // 2. FnMut：可變借用（&mut T）
    let mut count = 0;
    let mut increment = || {
        count += 1;  // 可變借用
        println!("count = {}", count);
    };
    increment();  // count = 1
    increment();  // count = 2
    // 在閉包使用期間，不能有其他對 count 的引用
    println!("最終 count = {}", count);  // 3

    // 3. FnOnce：取得所有權（T）
    let s = String::from("hello");
    let consume = || {
        let moved = s;  // 取得所有權
        println!("{}", moved);
    };
    consume();
    // consume();  // 錯誤！只能呼叫一次
    // println!("{}", s);  // 錯誤！s 已被移動
}
```

### move 閉包

強制閉包取得所有權：

```rust
fn main() {
    let x = vec![1, 2, 3];

    // 使用 move 強制取得所有權
    let consume = move || {
        println!("{:?}", x);
    };

    // println!("{:?}", x);  // 錯誤！x 已被移動到閉包

    consume();
    consume();  // 可以多次呼叫（因為 Vec 在閉包內）

    // 常見用途：跨執行緒傳遞資料
    use std::thread;

    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // 必須使用 move，因為執行緒可能比 main 活得久
        println!("執行緒中: {:?}", data);
    });

    handle.join().unwrap();
}
```

### 閉包作為參數

```rust
// 使用泛型和 trait bounds
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// 簡短寫法
fn apply_short<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

// 使用 impl Trait
fn apply_impl(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    let double = |x| x * 2;
    let result = apply(double, 5);
    println!("結果: {}", result);  // 10

    // 直接傳入閉包
    let result = apply(|x| x + 10, 5);
    println!("結果: {}", result);  // 15
}
```

### 不同的 Fn trait

```rust
// Fn：不可變借用，可多次呼叫
fn call_fn<F: Fn()>(f: F) {
    f();
    f();  // 可以多次呼叫
}

// FnMut：可變借用，可多次呼叫
fn call_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}

// FnOnce：取得所有權，只能呼叫一次
fn call_fn_once<F: FnOnce() -> String>(f: F) -> String {
    f()  // 只能呼叫一次
}

fn main() {
    // Fn 範例
    let x = 5;
    call_fn(|| println!("{}", x));

    // FnMut 範例
    let mut count = 0;
    call_fn_mut(|| {
        count += 1;
        println!("{}", count);
    });

    // FnOnce 範例
    let s = String::from("hello");
    let result = call_fn_once(|| {
        let mut owned = s;
        owned.push_str(" world");
        owned
    });
    println!("{}", result);
}
```

### 閉包作為回傳值

```rust
// 使用 impl Trait（最常用）
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// 使用 Box<dyn Fn>（當需要動態分派時）
fn make_adder_boxed(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

// 回傳不同閉包（需要 Box）
fn get_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "add" => Box::new(|a, b| a + b),
        "sub" => Box::new(|a, b| a - b),
        "mul" => Box::new(|a, b| a * b),
        _ => Box::new(|a, b| a / b),
    }
}

fn main() {
    let add_5 = make_adder(5);
    println!("3 + 5 = {}", add_5(3));  // 8

    let add = get_operation("add");
    let sub = get_operation("sub");
    println!("10 + 5 = {}", add(10, 5));  // 15
    println!("10 - 5 = {}", sub(10, 5));  // 5
}
```

---

## 5.3 高階函式

### 函式指標

```rust
// 函式指標型別：fn(參數) -> 回傳值
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // 函式指標
    let f: fn(i32, i32) -> i32 = add;
    println!("{}", f(5, 3));  // 8

    // 傳遞函式給其他函式
    fn apply(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        f(a, b)
    }

    println!("{}", apply(add, 10, 20));  // 30

    // 函式指標 vs 閉包
    // fn 是具體型別，Fn/FnMut/FnOnce 是 trait
    // 所有 fn 都實作了 Fn、FnMut、FnOnce

    fn takes_closure<F: Fn(i32) -> i32>(f: F) -> i32 {
        f(10)
    }

    fn double(x: i32) -> i32 { x * 2 }

    // 可以傳函式
    println!("{}", takes_closure(double));  // 20

    // 也可以傳閉包
    println!("{}", takes_closure(|x| x * 3));  // 30
}
```

### 接受函式作為參數

```rust
// map 模式
fn map_vec<F>(v: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    v.into_iter().map(f).collect()
}

// filter 模式
fn filter_vec<F>(v: Vec<i32>, predicate: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    v.into_iter().filter(|x| predicate(x)).collect()
}

// fold 模式
fn fold_vec<F, T>(v: Vec<i32>, init: T, f: F) -> T
where
    F: Fn(T, i32) -> T,
{
    v.into_iter().fold(init, f)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 使用 map
    let doubled = map_vec(numbers.clone(), |x| x * 2);
    println!("{:?}", doubled);  // [2, 4, 6, 8, 10]

    // 使用 filter
    let evens = filter_vec(numbers.clone(), |x| x % 2 == 0);
    println!("{:?}", evens);  // [2, 4]

    // 使用 fold
    let sum = fold_vec(numbers, 0, |acc, x| acc + x);
    println!("{}", sum);  // 15
}
```

### 回傳函式

```rust
// 回傳函式指標
fn get_math_op(op: char) -> fn(i32, i32) -> i32 {
    fn add(a: i32, b: i32) -> i32 { a + b }
    fn sub(a: i32, b: i32) -> i32 { a - b }

    match op {
        '+' => add,
        '-' => sub,
        _ => add,
    }
}

// 回傳閉包
fn counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

// 工廠模式
fn create_validator(min: i32, max: i32) -> impl Fn(i32) -> bool {
    move |value| value >= min && value <= max
}

fn main() {
    let add_op = get_math_op('+');
    println!("{}", add_op(5, 3));  // 8

    let mut count = counter(0);
    println!("{}", count());  // 1
    println!("{}", count());  // 2
    println!("{}", count());  // 3

    let is_valid_age = create_validator(0, 120);
    println!("{}", is_valid_age(25));   // true
    println!("{}", is_valid_age(150));  // false
}
```

### 組合器模式

```rust
fn main() {
    // 函式組合
    fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
    {
        move |x| g(f(x))
    }

    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;

    let add_then_double = compose(add_one, double);
    println!("{}", add_then_double(5));  // (5 + 1) * 2 = 12

    // 管道模式
    fn pipe<T, F>(value: T, f: F) -> T
    where
        F: FnOnce(T) -> T,
    {
        f(value)
    }

    let result = pipe(5, |x| x + 1);
    let result = pipe(result, |x| x * 2);
    let result = pipe(result, |x| x - 3);
    println!("{}", result);  // ((5 + 1) * 2) - 3 = 9

    // 使用標準庫的迭代器達到類似效果
    let result = [5]
        .into_iter()
        .map(|x| x + 1)
        .map(|x| x * 2)
        .map(|x| x - 3)
        .next()
        .unwrap();
    println!("{}", result);  // 9
}
```

---

## 本章重點回顧

1. **函式定義**
   - 使用 `fn` 關鍵字
   - 參數需要型別標註
   - 最後一個表達式作為回傳值
   - 提前回傳使用 `return`

2. **表達式 vs 陳述式**
   - 表達式求值並回傳結果
   - 陳述式執行動作，不回傳值
   - 分號將表達式變成陳述式

3. **閉包**
   - 匿名函式，可捕獲環境
   - 三種捕獲方式：Fn、FnMut、FnOnce
   - `move` 強制取得所有權
   - 可作為參數和回傳值

4. **高階函式**
   - 函式指標型別 `fn`
   - 接受和回傳函式
   - 函式組合與管道模式

---

## 練習題

### 練習 1：基本函式

```rust
// 實作以下函式：

// 1. 計算圓面積
fn circle_area(radius: f64) -> f64 {
    // π * r²
}

// 2. 判斷是否為質數
fn is_prime(n: u32) -> bool {
    // 你的程式碼
}

// 3. 費氏數列第 n 項
fn fibonacci(n: u32) -> u64 {
    // 你的程式碼
}
```

### 練習 2：閉包練習

```rust
fn main() {
    // 1. 寫一個閉包，計算兩數的最大公因數
    let gcd = |a: u32, b: u32| -> u32 {
        // 你的程式碼
    };

    // 2. 寫一個閉包生成器，產生計數器
    fn make_counter() -> impl FnMut() -> i32 {
        // 你的程式碼
    }

    // 3. 使用閉包過濾和轉換向量
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // 找出偶數並平方
    let result: Vec<i32> = // 你的程式碼
    println!("{:?}", result);  // [4, 16, 36, 64, 100]
}
```

### 練習 3：高階函式

```rust
// 實作 compose 函式，組合兩個函式
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    // 你的程式碼
}

// 實作 curry 函式，將二元函式轉換為連續呼叫
fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> impl Fn(B) -> C
where
    A: Copy,
    F: Fn(A, B) -> C + Copy,
{
    // 你的程式碼
}
```

### 練習 4：實用工具函式

```rust
// 實作簡易的 Option 方法

fn map_option<T, U, F>(opt: Option<T>, f: F) -> Option<U>
where
    F: FnOnce(T) -> U,
{
    // 你的程式碼
}

fn filter_option<T, F>(opt: Option<T>, predicate: F) -> Option<T>
where
    F: FnOnce(&T) -> bool,
{
    // 你的程式碼
}

fn and_then_option<T, U, F>(opt: Option<T>, f: F) -> Option<U>
where
    F: FnOnce(T) -> Option<U>,
{
    // 你的程式碼
}
```

---

## 延伸閱讀

- [函式](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [閉包](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Fn traits](https://doc.rust-lang.org/std/ops/trait.Fn.html)
- [函數式程式設計模式](https://rust-unofficial.github.io/patterns/functional/index.html)

---

[← 上一章：控制流程](./04-control-flow.md) | [下一章：所有權系統 →](./06-ownership.md)

---

## 第一部分完結

恭喜你完成了 **第一部分：基礎入門**！

你已經學會了：

- Rust 的核心理念與優勢
- 開發環境建置
- 基本語法與型別系統
- 控制流程與模式匹配
- 函式與閉包

接下來的 **第二部分：核心概念** 將深入 Rust 最獨特的特性：

- 所有權系統
- 生命週期
- 結構體與列舉
- 錯誤處理

這些是真正掌握 Rust 的關鍵！
