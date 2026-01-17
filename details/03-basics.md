# 第 3 章：基本語法

> 本章涵蓋 Rust 的基本語法元素，包括變數、型別、運算子等核心概念。

---

## 3.1 變數與可變性

### let：不可變綁定（預設）

在 Rust 中，變數預設是**不可變的**（immutable）：

```rust
fn main() {
    let x = 5;
    println!("x 的值是: {}", x);

    // x = 6;  // 錯誤！不能修改不可變變數
    // error[E0384]: cannot assign twice to immutable variable `x`
}
```

這與大多數語言不同（如 JavaScript 的 `let` 是可變的），是 Rust 安全性設計的一部分。

### let mut：可變綁定

如果需要修改變數，使用 `mut` 關鍵字：

```rust
fn main() {
    let mut x = 5;
    println!("x 的值是: {}", x);

    x = 6;  // OK！x 是可變的
    println!("x 的值是: {}", x);
}
```

### const：編譯時期常數

`const` 用於定義編譯時期就確定的常數：

```rust
// 必須標註型別
// 命名慣例：全大寫 + 底線分隔
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159265358979;
const APP_NAME: &str = "My App";

fn main() {
    println!("最大點數: {}", MAX_POINTS);
    println!("圓周率: {}", PI);
}
```

**const vs let 的差異：**

| 特性 | const | let |
|------|-------|-----|
| 必須標註型別 | 是 | 否（可推導） |
| 必須在編譯時期確定值 | 是 | 否 |
| 可以是可變的 | 否 | 是（mut） |
| 作用域 | 全域或區域 | 區域 |
| 可以遮蔽 | 否 | 是 |

### static：靜態變數

`static` 用於定義具有 `'static` 生命週期的變數：

```rust
// 不可變靜態變數
static GREETING: &str = "Hello";

// 可變靜態變數（需要 unsafe 存取）
static mut COUNTER: u32 = 0;

fn main() {
    println!("{}", GREETING);

    // 可變靜態變數需要 unsafe
    unsafe {
        COUNTER += 1;
        println!("計數: {}", COUNTER);
    }
}
```

**static vs const 的差異：**

| 特性 | static | const |
|------|--------|-------|
| 記憶體位置 | 固定的記憶體位置 | 內聯到使用處 |
| 可以是可變的 | 是（需 unsafe） | 否 |
| 生命週期 | 'static | N/A |
| 適用場景 | 需要固定位置、可變全域變數 | 編譯時期常數 |

### 變數遮蔽（Shadowing）

Rust 允許在同一作用域內重新宣告同名變數：

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);  // 5

    let x = x + 1;  // 遮蔽前一個 x
    println!("x = {}", x);  // 6

    {
        let x = x * 2;  // 內層作用域的遮蔽
        println!("內層 x = {}", x);  // 12
    }

    println!("外層 x = {}", x);  // 6（內層遮蔽結束）

    // 甚至可以改變型別！
    let x = "hello";  // 從 i32 變成 &str
    println!("x = {}", x);
}
```

**遮蔽 vs mut 的差異：**

```rust
// 使用遮蔽：可以改變型別
let spaces = "   ";
let spaces = spaces.len();  // OK：&str → usize

// 使用 mut：不能改變型別
let mut spaces = "   ";
// spaces = spaces.len();  // 錯誤！型別不匹配
```

---

## 3.2 基本資料型別

### 純量型別（Scalar Types）

純量型別代表單一值。Rust 有四種主要的純量型別。

#### 整數

| 長度 | 有號 | 無號 | 範圍（有號） |
|------|------|------|-------------|
| 8-bit | i8 | u8 | -128 ~ 127 |
| 16-bit | i16 | u16 | -32,768 ~ 32,767 |
| 32-bit | i32 | u32 | -2³¹ ~ 2³¹-1 |
| 64-bit | i64 | u64 | -2⁶³ ~ 2⁶³-1 |
| 128-bit | i128 | u128 | -2¹²⁷ ~ 2¹²⁷-1 |
| arch | isize | usize | 取決於系統架構 |

```rust
fn main() {
    // 型別標註
    let a: i32 = 42;
    let b: u64 = 100;

    // 型別後綴
    let c = 42i32;
    let d = 100u64;

    // 字面值格式
    let decimal = 98_222;        // 十進位（底線分隔方便閱讀）
    let hex = 0xff;              // 十六進位
    let octal = 0o77;            // 八進位
    let binary = 0b1111_0000;    // 二進位
    let byte = b'A';             // 位元組（u8）

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("binary: {}", binary);
}
```

**整數溢位：**

```rust
fn main() {
    let x: u8 = 255;

    // Debug 模式：panic
    // Release 模式：溢位環繞（255 + 1 = 0）

    // 明確處理溢位
    let a = x.wrapping_add(1);   // 環繞：0
    let b = x.checked_add(1);    // 檢查：None
    let c = x.saturating_add(1); // 飽和：255
    let (d, overflow) = x.overflowing_add(1);  // (0, true)

    println!("wrapping: {}", a);
    println!("checked: {:?}", b);
    println!("saturating: {}", c);
    println!("overflowing: {} (overflow: {})", d, overflow);
}
```

#### 浮點數

```rust
fn main() {
    let x = 2.0;      // f64（預設）
    let y: f32 = 3.0; // f32

    // 運算
    let sum = 5.0 + 10.0;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 30.0;
    let quotient = 56.7 / 32.2;
    let remainder = 43.0 % 5.0;

    // 特殊值
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN;

    println!("NaN == NaN: {}", nan == nan);  // false！
    println!("is_nan: {}", nan.is_nan());    // true
}
```

#### 布林

```rust
fn main() {
    let t = true;
    let f: bool = false;

    // 布林運算
    let and = true && false;  // false
    let or = true || false;   // true
    let not = !true;          // false

    println!("and: {}, or: {}, not: {}", and, or, not);
}
```

#### 字元

Rust 的 `char` 是 Unicode 純量值，佔 4 位元組：

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart = '❤';
    let emoji = '😀';
    let chinese = '中';

    println!("字元: {}, {}, {}, {}, {}", c, z, heart, emoji, chinese);

    // char 是 4 位元組
    println!("char 大小: {} 位元組", std::mem::size_of::<char>());

    // Unicode 相關方法
    println!("是否為字母: {}", 'a'.is_alphabetic());
    println!("是否為數字: {}", '5'.is_numeric());
    println!("轉大寫: {}", 'a'.to_uppercase());
}
```

### 複合型別（Compound Types）

#### 元組（Tuple）

元組可以包含不同型別的多個值：

```rust
fn main() {
    // 建立元組
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解構
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 索引存取（從 0 開始）
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 單元元組
    let unit: () = ();  // 空元組，類似其他語言的 void

    // 元組作為函式回傳值
    fn swap(a: i32, b: i32) -> (i32, i32) {
        (b, a)
    }

    let (a, b) = swap(1, 2);
    println!("交換後: a = {}, b = {}", a, b);
}
```

#### 陣列（Array）

陣列是固定長度、相同型別的元素集合：

```rust
fn main() {
    // 建立陣列
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];  // 明確標註型別
    let c = [3; 5];  // [3, 3, 3, 3, 3]

    // 存取元素
    let first = a[0];
    let second = a[1];

    // 長度
    println!("長度: {}", a.len());

    // 邊界檢查
    // let invalid = a[10];  // panic: index out of bounds

    // 安全存取
    match a.get(10) {
        Some(value) => println!("值: {}", value),
        None => println!("索引超出範圍"),
    }

    // 遍歷
    for element in a {
        println!("{}", element);
    }

    for (index, element) in a.iter().enumerate() {
        println!("[{}] = {}", index, element);
    }
}
```

#### 切片（Slice）

切片是對連續序列的引用：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    // 建立切片
    let slice = &a[1..4];  // [2, 3, 4]
    let slice_all = &a[..];  // 完整切片
    let slice_from = &a[2..];  // [3, 4, 5]
    let slice_to = &a[..3];  // [1, 2, 3]

    println!("切片: {:?}", slice);

    // 字串切片
    let s = String::from("hello world");
    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"

    println!("{} {}", hello, world);

    // 切片作為參數（比陣列更靈活）
    fn sum(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }

    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];

    println!("陣列總和: {}", sum(&arr));
    println!("Vec 總和: {}", sum(&vec));
}
```

---

## 3.3 型別推導與標註

### 自動型別推導

Rust 編譯器非常聰明，大多數時候可以自動推導型別：

```rust
fn main() {
    let x = 5;           // i32（整數預設）
    let y = 2.0;         // f64（浮點預設）
    let z = true;        // bool
    let s = "hello";     // &str
    let c = 'a';         // char

    // 根據使用方式推導
    let mut vec = Vec::new();
    vec.push(1);  // 編譯器推導 vec 是 Vec<i32>

    // 根據回傳型別推導
    let parsed: i32 = "42".parse().unwrap();
}
```

### 明確型別標註

有時需要明確指定型別：

```rust
fn main() {
    // 編譯器無法推導時
    let guess: u32 = "42".parse().expect("不是數字");

    // 避免歧義
    let x: i64 = 42;  // 明確使用 i64 而非預設的 i32

    // 集合型別
    let vec: Vec<i32> = Vec::new();
    let map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

    // Turbofish 語法 ::<>
    let parsed = "42".parse::<i32>().unwrap();
    let vec = Vec::<i32>::new();
    let numbers: Vec<_> = (0..10).collect();  // _ 讓編譯器推導元素型別
}
```

### 型別別名（Type Alias）

```rust
// 簡化複雜型別
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

fn main() {
    let distance: Kilometers = 100;
    println!("距離: {} 公里", distance);
}

// 在函式簽名中使用
fn takes_long_type(f: Thunk) {
    // ...
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}
```

---

## 3.4 運算子

### 算術運算子

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("加法: {} + {} = {}", a, b, a + b);    // 13
    println!("減法: {} - {} = {}", a, b, a - b);    // 7
    println!("乘法: {} * {} = {}", a, b, a * b);    // 30
    println!("除法: {} / {} = {}", a, b, a / b);    // 3（整數除法）
    println!("餘數: {} % {} = {}", a, b, a % b);    // 1

    // 浮點運算
    let x = 10.0;
    let y = 3.0;
    println!("浮點除法: {} / {} = {}", x, y, x / y);  // 3.333...

    // 負數
    let neg = -a;
    println!("負數: -{} = {}", a, neg);
}
```

### 比較運算子

```rust
fn main() {
    let a = 5;
    let b = 10;

    println!("{} == {}: {}", a, b, a == b);  // false
    println!("{} != {}: {}", a, b, a != b);  // true
    println!("{} < {}: {}", a, b, a < b);    // true
    println!("{} > {}: {}", a, b, a > b);    // false
    println!("{} <= {}: {}", a, b, a <= b);  // true
    println!("{} >= {}: {}", a, b, a >= b);  // false

    // 字串比較
    let s1 = "apple";
    let s2 = "banana";
    println!("{} < {}: {}", s1, s2, s1 < s2);  // true（字典序）
}
```

### 邏輯運算子

```rust
fn main() {
    let t = true;
    let f = false;

    // AND：兩者都為 true 才為 true
    println!("true && true: {}", true && true);    // true
    println!("true && false: {}", true && false);  // false

    // OR：任一為 true 即為 true
    println!("true || false: {}", true || false);  // true
    println!("false || false: {}", false || false); // false

    // NOT：反轉
    println!("!true: {}", !true);   // false
    println!("!false: {}", !false); // true

    // 短路求值
    let result = false && {
        println!("這不會被執行");
        true
    };

    let result = true || {
        println!("這也不會被執行");
        false
    };
}
```

### 位元運算子

```rust
fn main() {
    let a: u8 = 0b1010_1010;  // 170
    let b: u8 = 0b0101_0101;  // 85

    // 位元 AND
    println!("a & b = {:08b}", a & b);   // 00000000

    // 位元 OR
    println!("a | b = {:08b}", a | b);   // 11111111

    // 位元 XOR
    println!("a ^ b = {:08b}", a ^ b);   // 11111111

    // 位元 NOT
    println!("!a = {:08b}", !a);         // 01010101

    // 左移
    println!("a << 1 = {:08b}", a << 1); // 01010100

    // 右移
    println!("a >> 1 = {:08b}", a >> 1); // 01010101

    // 實用範例：檢查特定位元
    let flags: u8 = 0b0000_0101;
    let flag_1 = (flags & 0b0000_0001) != 0;  // true
    let flag_2 = (flags & 0b0000_0010) != 0;  // false
    let flag_3 = (flags & 0b0000_0100) != 0;  // true
}
```

### 複合指派運算子

```rust
fn main() {
    let mut x = 10;

    x += 5;   // x = x + 5
    println!("x += 5: {}", x);  // 15

    x -= 3;   // x = x - 3
    println!("x -= 3: {}", x);  // 12

    x *= 2;   // x = x * 2
    println!("x *= 2: {}", x);  // 24

    x /= 4;   // x = x / 4
    println!("x /= 4: {}", x);  // 6

    x %= 4;   // x = x % 4
    println!("x %= 4: {}", x);  // 2

    // 位元複合指派
    let mut bits: u8 = 0b1010;
    bits &= 0b1100;   // AND
    bits |= 0b0001;   // OR
    bits ^= 0b0011;   // XOR
    bits <<= 1;       // 左移
    bits >>= 2;       // 右移
}
```

### 其他運算子

```rust
fn main() {
    // 範圍運算子
    let range = 1..5;      // 1, 2, 3, 4（不包含 5）
    let range_inclusive = 1..=5;  // 1, 2, 3, 4, 5（包含 5）

    for i in 1..5 {
        print!("{} ", i);  // 1 2 3 4
    }
    println!();

    // 解引用運算子
    let x = 5;
    let r = &x;
    println!("*r = {}", *r);  // 5

    // 問號運算子（錯誤傳遞）
    fn might_fail() -> Result<i32, String> {
        let value: i32 = "42".parse().map_err(|_| "解析失敗".to_string())?;
        Ok(value * 2)
    }

    // 型別轉換 as
    let x: i32 = 42;
    let y: i64 = x as i64;
    let z: f64 = x as f64;
    let c: char = 65u8 as char;  // 'A'
}
```

---

## 本章重點回顧

1. **變數與可變性**
   - `let`：不可變綁定（預設）
   - `let mut`：可變綁定
   - `const`：編譯時期常數
   - `static`：靜態變數
   - 遮蔽：同名變數可重新宣告

2. **基本資料型別**
   - 純量：整數、浮點數、布林、字元
   - 複合：元組、陣列、切片
   - Rust 的整數有明確的位元數

3. **型別系統**
   - 靜態型別，編譯時期確定
   - 強大的型別推導
   - 型別標註與型別別名

4. **運算子**
   - 算術、比較、邏輯、位元
   - 複合指派運算子
   - 範圍、解引用、型別轉換

---

## 練習題

### 練習 1：變數遮蔽

預測以下程式碼的輸出：

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("內層 x: {}", x);
    }
    println!("外層 x: {}", x);
}
```

### 練習 2：型別轉換

完成以下程式碼，進行各種型別轉換：

```rust
fn main() {
    let a: i32 = 42;

    // 轉換為 f64 並印出
    // 轉換為 u8 並印出
    // 轉換為 char 並印出（提示：先轉 u8）
}
```

### 練習 3：元組與陣列

```rust
fn main() {
    // 建立一個包含你名字、年齡、是否為學生的元組
    // 使用解構取出各欄位

    // 建立一個包含 5 個偶數的陣列
    // 計算並印出總和
}
```

### 練習 4：位元操作

實作一個簡單的權限系統：

```rust
const READ: u8 = 0b001;
const WRITE: u8 = 0b010;
const EXECUTE: u8 = 0b100;

fn main() {
    let mut permissions: u8 = 0;

    // 新增讀取權限
    // 新增執行權限
    // 檢查是否有寫入權限
    // 移除讀取權限
}
```

---

## 延伸閱讀

- [Rust 資料型別](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust 運算子](https://doc.rust-lang.org/book/appendix-02-operators.html)
- [原始型別文件](https://doc.rust-lang.org/std/primitive/index.html)

---

[← 上一章：環境建置](./02-environment.md) | [下一章：控制流程 →](./04-control-flow.md)
