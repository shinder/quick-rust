# 第 4 章：控制流程

> 控制流程決定程式的執行路徑。Rust 提供了條件判斷、迴圈和強大的模式匹配。

---

## 4.1 條件判斷

### if/else 表達式

基本的條件判斷：

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("數字小於 5");
    } else if number < 10 {
        println!("數字介於 5 到 9 之間");
    } else {
        println!("數字大於等於 10");
    }
}
```

**重要差異（與其他語言比較）：**

```rust
fn main() {
    let number = 3;

    // 錯誤！條件必須是 bool
    // if number {  // 錯誤：expected `bool`, found integer
    //     println!("數字不是零");
    // }

    // 正確：明確比較
    if number != 0 {
        println!("數字不是零");
    }

    // 條件不需要括號
    if number > 0 {
        println!("正數");
    }
}
```

### if 是表達式

在 Rust 中，`if` 是**表達式**，可以回傳值：

```rust
fn main() {
    let condition = true;

    // if 表達式作為值
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);  // 5

    // 類似三元運算子
    let max = if 10 > 5 { 10 } else { 5 };

    // 多行區塊
    let description = if number > 0 {
        let double = number * 2;
        format!("正數，兩倍是 {}", double)
    } else if number < 0 {
        String::from("負數")
    } else {
        String::from("零")
    };

    println!("{}", description);
}
```

**注意：分支的型別必須相同**

```rust
fn main() {
    let condition = true;

    // 錯誤！兩個分支型別不同
    // let number = if condition { 5 } else { "six" };
    // error: `if` and `else` have incompatible types

    // 正確
    let number = if condition { 5 } else { 6 };
}
```

### if let 語法糖

`if let` 是處理單一模式匹配的簡潔寫法：

```rust
fn main() {
    let some_value: Option<i32> = Some(42);

    // 使用 match
    match some_value {
        Some(x) => println!("值是: {}", x),
        None => (),  // 什麼都不做
    }

    // 使用 if let（更簡潔）
    if let Some(x) = some_value {
        println!("值是: {}", x);
    }

    // if let 搭配 else
    if let Some(x) = some_value {
        println!("有值: {}", x);
    } else {
        println!("沒有值");
    }

    // 解構列舉
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }

    let msg = Message::Move { x: 10, y: 20 };

    if let Message::Move { x, y } = msg {
        println!("移動到 ({}, {})", x, y);
    }
}
```

### let-else（Rust 1.65+）

```rust
fn main() {
    let some_value: Option<i32> = Some(42);

    // let-else：模式不匹配時必須發散（return、panic 等）
    let Some(x) = some_value else {
        println!("沒有值，提前返回");
        return;
    };

    println!("值是: {}", x);

    // 實用範例：解析或提前返回
    fn process_input(input: &str) -> Option<i32> {
        let Ok(number) = input.parse::<i32>() else {
            return None;
        };
        Some(number * 2)
    }
}
```

---

## 4.2 迴圈

### loop：無限迴圈

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("count = {}", count);

        if count >= 5 {
            break;  // 跳出迴圈
        }
    }

    println!("迴圈結束");
}
```

**loop 可以回傳值：**

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // 回傳 20
        }
    };

    println!("結果: {}", result);  // 20
}
```

### while 條件迴圈

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("發射！");
}
```

### while let 迴圈

```rust
fn main() {
    let mut stack = vec![1, 2, 3];

    // 當 pop() 回傳 Some 時繼續迴圈
    while let Some(top) = stack.pop() {
        println!("彈出: {}", top);
    }
    // 輸出：3, 2, 1

    // 等同於
    loop {
        match stack.pop() {
            Some(top) => println!("{}", top),
            None => break,
        }
    }
}
```

### for 迭代迴圈

`for` 迴圈是 Rust 中最常用的迴圈：

```rust
fn main() {
    // 遍歷陣列
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("值: {}", element);
    }

    // 遍歷範圍
    for number in 1..5 {
        print!("{} ", number);  // 1 2 3 4
    }
    println!();

    // 包含結尾的範圍
    for number in 1..=5 {
        print!("{} ", number);  // 1 2 3 4 5
    }
    println!();

    // 反向
    for number in (1..4).rev() {
        print!("{} ", number);  // 3 2 1
    }
    println!();

    // 帶索引
    for (index, value) in arr.iter().enumerate() {
        println!("[{}] = {}", index, value);
    }
}
```

**遍歷集合的三種方式：**

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    // 1. iter()：借用（不可變引用）
    for x in v.iter() {
        println!("{}", x);  // x 是 &i32
    }
    // v 還可以使用

    // 2. iter_mut()：可變借用
    for x in v.iter_mut() {
        *x += 10;  // 修改元素
    }
    println!("{:?}", v);  // [11, 12, 13]

    // 3. into_iter()：取得所有權
    for x in v.into_iter() {
        println!("{}", x);  // x 是 i32
    }
    // v 不能再使用！

    // 語法糖
    let v = vec![1, 2, 3];
    for x in &v { }       // 等同於 v.iter()
    for x in &mut v { }   // 等同於 v.iter_mut()
    for x in v { }        // 等同於 v.into_iter()
}
```

### 迴圈標籤與巢狀迴圈

```rust
fn main() {
    // 迴圈標籤
    'outer: for i in 0..3 {
        println!("外層 i = {}", i);

        for j in 0..3 {
            println!("  內層 j = {}", j);

            if j == 1 {
                continue 'outer;  // 跳到外層迴圈的下一輪
            }

            if i == 2 {
                break 'outer;     // 跳出外層迴圈
            }
        }
    }

    // 實用範例：搜尋二維陣列
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    let target = 5;

    'search: for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == target {
                println!("找到 {} 在 [{}, {}]", target, i, j);
                break 'search;
            }
        }
    }
}
```

### loop 回傳值（進階）

```rust
fn main() {
    let mut count = 0;

    // 從巢狀迴圈回傳值
    let result = 'outer: loop {
        loop {
            count += 1;
            if count > 10 {
                break 'outer count * 2;
            }
        }
    };

    println!("結果: {}", result);  // 22
}
```

---

## 4.3 模式匹配

### match 表達式

`match` 是 Rust 最強大的控制流程結構：

```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"),  // _ 是萬用模式
    }

    // match 是表達式，可以回傳值
    let description = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "其他",
    };

    println!("{}", description);
}
```

**match 必須窮盡所有可能：**

```rust
fn main() {
    let number = 5;

    // 錯誤：沒有處理所有情況
    // match number {
    //     1 => println!("一"),
    //     2 => println!("二"),
    // }

    // 正確：使用 _ 處理其他情況
    match number {
        1 => println!("一"),
        2 => println!("二"),
        _ => println!("其他"),
    }
}
```

### 多值匹配與範圍

```rust
fn main() {
    let number = 5;

    match number {
        // 多值匹配
        1 | 2 | 3 => println!("一到三"),

        // 範圍匹配
        4..=6 => println!("四到六"),

        // 其他
        _ => println!("其他"),
    }

    // 字元範圍
    let c = 'c';
    match c {
        'a'..='z' => println!("小寫字母"),
        'A'..='Z' => println!("大寫字母"),
        _ => println!("其他"),
    }
}
```

### 解構模式

```rust
fn main() {
    // 解構元組
    let point = (3, 5);
    match point {
        (0, 0) => println!("原點"),
        (x, 0) => println!("在 x 軸上，x = {}", x),
        (0, y) => println!("在 y 軸上，y = {}", y),
        (x, y) => println!("點 ({}, {})", x, y),
    }

    // 解構結構體
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x: 0, y } => println!("在 y 軸上，y = {}", y),
        Point { x, y: 0 } => println!("在 x 軸上，x = {}", x),
        Point { x, y } => println!("點 ({}, {})", x, y),
    }

    // 解構列舉
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(255, 128, 0);
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移動到 ({}, {})", x, y),
        Message::Write(text) => println!("訊息: {}", text),
        Message::ChangeColor(r, g, b) => println!("顏色: RGB({}, {}, {})", r, g, b),
    }
}
```

### 匹配守衛（Match Guards）

```rust
fn main() {
    let number = Some(4);

    match number {
        Some(x) if x < 5 => println!("小於 5: {}", x),
        Some(x) if x >= 5 => println!("大於等於 5: {}", x),
        None => println!("沒有值"),
        _ => unreachable!(),
    }

    // 組合使用
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("x 是 4、5 或 6，且 y 為 true"),
        _ => println!("其他情況"),
    }
}
```

### @ 綁定

```rust
fn main() {
    let number = 5;

    match number {
        // 綁定變數同時檢查範圍
        n @ 1..=5 => println!("在 1-5 範圍內: {}", n),
        n @ 6..=10 => println!("在 6-10 範圍內: {}", n),
        n => println!("其他: {}", n),
    }

    // 結構體中使用
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 5, y: 10 };

    match p {
        Point { x: x_val @ 0..=5, y } => {
            println!("x 在 0-5 範圍內: {}, y: {}", x_val, y);
        }
        Point { x, y } => {
            println!("其他點: ({}, {})", x, y);
        }
    }
}
```

### 忽略模式

```rust
fn main() {
    // 忽略單一值
    let point = (3, 5, 7);
    match point {
        (x, _, z) => println!("x = {}, z = {}", x, z),
    }

    // 忽略多個值
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => println!("first = {}, last = {}", first, last),
    }

    // 忽略未使用的變數（避免警告）
    let _unused = 5;

    // 結構體中忽略欄位
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let p = Point { x: 1, y: 2, z: 3 };
    match p {
        Point { x, .. } => println!("x = {}", x),
    }
}
```

### 複雜模式範例

```rust
fn main() {
    // 巢狀解構
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Rgb(255, 128, 0));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: ({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: ({}, {}, {})", h, s, v);
        }
        _ => (),
    }

    // 引用模式
    let s = Some(String::from("hello"));

    match &s {
        Some(inner) => println!("長度: {}", inner.len()),
        None => println!("沒有值"),
    }
    // s 還可以使用

    // 可變引用模式
    let mut s = Some(String::from("hello"));

    match &mut s {
        Some(inner) => inner.push_str(" world"),
        None => (),
    }
    println!("{:?}", s);  // Some("hello world")
}
```

### matches! 巨集

```rust
fn main() {
    let x = Some(5);

    // 使用 matches! 簡化布林匹配
    let is_some_5 = matches!(x, Some(5));
    println!("是否為 Some(5): {}", is_some_5);

    // 等同於
    let is_some_5 = match x {
        Some(5) => true,
        _ => false,
    };

    // 範圍匹配
    let c = 'c';
    let is_letter = matches!(c, 'a'..='z' | 'A'..='Z');

    // 守衛條件
    let x = Some(10);
    let is_large = matches!(x, Some(n) if n > 5);
}
```

---

## 本章重點回顧

1. **條件判斷**
   - `if` 是表達式，可以回傳值
   - 條件必須是 `bool` 型別
   - `if let` 簡化單一模式匹配
   - `let-else` 處理不匹配時的發散

2. **迴圈**
   - `loop`：無限迴圈，可回傳值
   - `while`：條件迴圈
   - `for`：迭代迴圈（最常用）
   - 迴圈標籤處理巢狀迴圈

3. **模式匹配**
   - `match` 必須窮盡所有可能
   - 支援多值、範圍、解構
   - 匹配守衛增加條件
   - `@` 綁定同時匹配和命名
   - `matches!` 巨集簡化布林判斷

---

## 練習題

### 練習 1：FizzBuzz

實作經典的 FizzBuzz：

```rust
fn main() {
    for n in 1..=30 {
        // 15 的倍數印 "FizzBuzz"
        // 3 的倍數印 "Fizz"
        // 5 的倍數印 "Buzz"
        // 其他印數字
    }
}
```

### 練習 2：猜數字

完成以下猜數字遊戲：

```rust
use std::cmp::Ordering;

fn main() {
    let secret = 42;
    let guesses = [50, 25, 42];

    for guess in guesses {
        // 使用 match 和 Ordering 比較
        // 印出「太大」、「太小」或「正確！」
    }
}
```

### 練習 3：解構練習

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    // 使用 match 解構並計算面積
    // Circle: π * r²
    // Rectangle: w * h
    // Triangle: 0.5 * b * h
}
```

### 練習 4：模式匹配進階

```rust
fn describe_number(n: i32) -> &'static str {
    // 使用 match 回傳描述：
    // 0 => "零"
    // 1..=9 => "個位數"
    // 10..=99 => "兩位數"
    // 100..=999 => "三位數"
    // 負數 => "負數"
    // 其他 => "大數字"
}
```

---

## 延伸閱讀

- [控制流程](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [模式與匹配](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust 模式完整參考](https://doc.rust-lang.org/reference/patterns.html)

---

[← 上一章：基本語法](./03-basics.md) | [下一章：函式與閉包 →](./05-functions.md)
