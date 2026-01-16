# 第六章：集合與迭代器

## Vec：動態陣列

`Vec<T>` 是 Rust 最常用的集合，類似 JavaScript 的 `Array`。

### 建立 Vec

```javascript
// JavaScript
const arr = [];
const arr2 = [1, 2, 3];
const arr3 = new Array(5).fill(0);
```

```rust
// Rust
let v: Vec<i32> = Vec::new();        // 空的 Vec
let v = vec![1, 2, 3];               // vec! 巨集
let v = vec![0; 5];                  // [0, 0, 0, 0, 0]
let v: Vec<i32> = (0..5).collect();  // [0, 1, 2, 3, 4]
```

### 基本操作

```rust
let mut v = vec![1, 2, 3];

// 新增元素
v.push(4);                // [1, 2, 3, 4]
v.insert(0, 0);          // [0, 1, 2, 3, 4]

// 移除元素
let last = v.pop();       // Some(4), v = [0, 1, 2, 3]
let second = v.remove(1); // 1, v = [0, 2, 3]

// 存取元素
let first = v[0];         // 0（越界會 panic）
let first = v.get(0);     // Some(&0)（安全）
let none = v.get(100);    // None

// 長度
let len = v.len();
let is_empty = v.is_empty();
```

### JS Array vs Rust Vec 對照

| JavaScript | Rust | 說明 |
|------------|------|------|
| `arr.push(x)` | `v.push(x)` | 尾部新增 |
| `arr.pop()` | `v.pop()` | 尾部移除 |
| `arr.shift()` | `v.remove(0)` | 頭部移除 |
| `arr.unshift(x)` | `v.insert(0, x)` | 頭部新增 |
| `arr[i]` | `v[i]` 或 `v.get(i)` | 存取 |
| `arr.length` | `v.len()` | 長度 |
| `arr.slice(a, b)` | `&v[a..b]` | 切片 |
| `arr.concat(arr2)` | `v.extend(v2)` | 合併 |
| `arr.includes(x)` | `v.contains(&x)` | 包含 |
| `arr.indexOf(x)` | `v.iter().position(\|&i\| i == x)` | 索引 |

### 遍歷 Vec

```rust
let v = vec![1, 2, 3];

// 不可變遍歷
for item in &v {
    println!("{}", item);
}

// 可變遍歷
let mut v = vec![1, 2, 3];
for item in &mut v {
    *item += 10;
}

// 取得所有權（遍歷後 v 不能再用）
for item in v {
    println!("{}", item);
}
```

## HashMap：鍵值對集合

類似 JavaScript 的 `Object` 或 `Map`。

### 建立與操作

```javascript
// JavaScript
const map = new Map();
map.set("name", "Alice");
const obj = { name: "Alice", age: 30 };
```

```rust
use std::collections::HashMap;

// 建立
let mut map = HashMap::new();
map.insert("name", "Alice");
map.insert("city", "Taipei");

// 從陣列建立
let teams = vec![("Red", 10), ("Blue", 20)];
let scores: HashMap<_, _> = teams.into_iter().collect();
```

### 存取與修改

```rust
let mut scores = HashMap::new();
scores.insert("Alice", 100);

// 存取
let score = scores.get("Alice");  // Option<&i32>
let score = scores["Alice"];       // 直接取值（不存在會 panic）

// 檢查是否存在
if scores.contains_key("Alice") {
    println!("找到了！");
}

// 只在 key 不存在時插入
scores.entry("Bob").or_insert(50);

// 根據舊值更新
let count = scores.entry("Alice").or_insert(0);
*count += 1;
```

### 遍歷

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 只要 keys
for key in scores.keys() {
    println!("{}", key);
}

// 只要 values
for value in scores.values() {
    println!("{}", value);
}
```

## 迭代器（Iterator）

Rust 迭代器與 JS 的陣列方法非常相似！

### 建立迭代器

```rust
let v = vec![1, 2, 3];

let iter = v.iter();      // 借用：&T
let iter = v.iter_mut();  // 可變借用：&mut T
let iter = v.into_iter(); // 取得所有權：T
```

### 常用迭代器方法

#### map：轉換元素

```javascript
// JavaScript
[1, 2, 3].map(x => x * 2)  // [2, 4, 6]
```

```rust
// Rust
vec![1, 2, 3].iter().map(|x| x * 2).collect::<Vec<_>>()
// [2, 4, 6]
```

#### filter：過濾元素

```javascript
// JavaScript
[1, 2, 3, 4, 5].filter(x => x % 2 === 0)  // [2, 4]
```

```rust
// Rust
vec![1, 2, 3, 4, 5].iter().filter(|x| *x % 2 == 0).collect::<Vec<_>>()
// [2, 4]
```

#### fold/reduce：累積計算

```javascript
// JavaScript
[1, 2, 3, 4].reduce((acc, x) => acc + x, 0)  // 10
```

```rust
// Rust
vec![1, 2, 3, 4].iter().fold(0, |acc, x| acc + x)  // 10
// 或簡單用 sum
vec![1, 2, 3, 4].iter().sum::<i32>()  // 10
```

#### find：尋找元素

```javascript
// JavaScript
[1, 2, 3, 4].find(x => x > 2)  // 3
```

```rust
// Rust
vec![1, 2, 3, 4].iter().find(|&&x| x > 2)  // Some(&3)
```

### 迭代器鏈式呼叫

```javascript
// JavaScript
const result = numbers
    .filter(x => x % 2 === 0)
    .map(x => x * x)
    .reduce((a, b) => a + b, 0);
```

```rust
// Rust
let result: i32 = numbers
    .iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * x)
    .sum();
```

### 迭代器方法大全

| JavaScript | Rust | 說明 |
|------------|------|------|
| `map(fn)` | `.map(fn)` | 轉換 |
| `filter(fn)` | `.filter(fn)` | 過濾 |
| `reduce(fn, init)` | `.fold(init, fn)` | 累積 |
| `find(fn)` | `.find(fn)` | 找第一個 |
| `some(fn)` | `.any(fn)` | 任一滿足 |
| `every(fn)` | `.all(fn)` | 全部滿足 |
| `forEach(fn)` | `.for_each(fn)` | 執行副作用 |
| `slice(0, n)` | `.take(n)` | 取前 n 個 |
| `slice(n)` | `.skip(n)` | 跳過前 n 個 |
| `flat()` | `.flatten()` | 攤平 |
| `flatMap(fn)` | `.flat_map(fn)` | map + 攤平 |
| `indexOf(x)` | `.position(fn)` | 找索引 |
| `reverse()` | `.rev()` | 反轉 |
| `sort()` | `.sorted()` (itertools) | 排序 |

## 閉包（Closure）

Rust 閉包類似 JavaScript 的箭頭函式。

```javascript
// JavaScript
const add = (a, b) => a + b;
const double = x => x * 2;
```

```rust
// Rust
let add = |a, b| a + b;
let double = |x| x * 2;

// 帶型別標註
let add: fn(i32, i32) -> i32 = |a, b| a + b;

// 多行
let complex = |x| {
    let y = x * 2;
    y + 1
};
```

### 閉包捕獲環境

```rust
let factor = 2;
let multiply = |x| x * factor;  // 捕獲 factor

println!("{}", multiply(5));  // 10
```

### 捕獲方式

```rust
let s = String::from("hello");

// 借用
let print = || println!("{}", s);
print();
println!("{}", s);  // s 還可以用

// 可變借用
let mut s = String::from("hello");
let mut append = || s.push_str(" world");
append();
// 不能在這裡用 s，直到 append 不再使用

// 取得所有權（move）
let s = String::from("hello");
let consume = move || println!("{}", s);
consume();
// println!("{}", s);  // 錯誤！s 已經被移動
```

## 惰性求值

Rust 迭代器是惰性的，直到呼叫終結方法（如 `collect`）才會執行。

```rust
// 這不會執行任何操作！
let iter = vec![1, 2, 3]
    .iter()
    .map(|x| {
        println!("處理 {}", x);
        x * 2
    });

// 呼叫 collect 才會執行
let result: Vec<_> = iter.collect();
// 印出：處理 1、處理 2、處理 3
```

### 終結方法

觸發迭代器執行的方法：

```rust
.collect()      // 收集成集合
.sum()          // 加總
.count()        // 計數
.for_each(fn)   // 執行副作用
.fold(init, fn) // 累積
.any(fn)        // 任一
.all(fn)        // 全部
.find(fn)       // 尋找
.last()         // 最後一個
```

## 實用範例

### 統計字詞出現次數

```rust
use std::collections::HashMap;

let text = "hello world hello rust world";
let mut word_count = HashMap::new();

for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
}

// {"hello": 2, "world": 2, "rust": 1}
```

### 分組

```javascript
// JavaScript
const grouped = items.reduce((acc, item) => {
    (acc[item.category] ||= []).push(item);
    return acc;
}, {});
```

```rust
// Rust
use std::collections::HashMap;

let items = vec![
    ("fruit", "apple"),
    ("fruit", "banana"),
    ("vegetable", "carrot"),
];

let mut grouped: HashMap<&str, Vec<&str>> = HashMap::new();
for (category, item) in items {
    grouped.entry(category).or_insert_with(Vec::new).push(item);
}
```

### 找出最大值

```rust
let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
let max = numbers.iter().max();  // Some(&9)
let min = numbers.iter().min();  // Some(&1)

// 自訂比較
let people = vec![("Alice", 30), ("Bob", 25), ("Carol", 35)];
let oldest = people.iter().max_by_key(|(_, age)| age);
// Some(&("Carol", 35))
```

## 練習題

### 練習 1：Vec 操作
```rust
// 實作函式：移除 Vec 中所有偶數
fn remove_evens(v: &mut Vec<i32>) {
    // 你的程式碼
}
```

### 練習 2：迭代器鏈
```rust
// 計算字串中所有數字的總和
// "a1b2c3" -> 6
fn sum_digits(s: &str) -> u32 {
    // 你的程式碼（使用迭代器）
}
```

### 練習 3：HashMap 練習
```rust
// 統計字串中每個字元出現的次數
fn char_count(s: &str) -> HashMap<char, usize> {
    // 你的程式碼
}
```

### 練習 4：綜合應用
給定一組學生成績，找出：
1. 平均分數
2. 最高分的學生
3. 及格（>= 60）的學生名單

```rust
let scores = vec![
    ("Alice", 85),
    ("Bob", 55),
    ("Carol", 92),
    ("David", 60),
];
```

---

[← 上一章：錯誤處理](./05-error-handling.md) | [下一章：模組與套件管理 →](./07-modules.md)
