# 第 14 章：迭代器

> 迭代器是 Rust 中處理序列資料的核心抽象，提供了函數式、惰性的資料處理方式

---

## 14.1 迭代器基礎

迭代器是實作了 `Iterator` trait 的型別，提供了一種統一的方式來遍歷集合中的元素。

### Iterator Trait

```rust
// Iterator trait 的簡化定義
pub trait Iterator {
    type Item;  // 關聯類型：迭代器產生的元素型別

    fn next(&mut self) -> Option<Self::Item>;

    // 其他方法都有預設實作...
}
```

### 建立迭代器

```rust
fn main() {
    let v = vec![1, 2, 3];

    // iter()：產生不可變引用的迭代器 (&T)
    let iter1 = v.iter();
    for item in iter1 {
        println!("{}", item); // item 是 &i32
    }

    // iter_mut()：產生可變引用的迭代器 (&mut T)
    let mut v2 = vec![1, 2, 3];
    for item in v2.iter_mut() {
        *item *= 2;
    }
    println!("{:?}", v2); // [2, 4, 6]

    // into_iter()：消耗集合，產生擁有所有權的迭代器 (T)
    let v3 = vec![String::from("a"), String::from("b")];
    for item in v3.into_iter() {
        println!("{}", item); // item 是 String
    }
    // v3 已被消耗，無法再使用
}
```

### iter、iter_mut、into_iter 比較

```rust
fn main() {
    let v = vec![1, 2, 3];

    // iter() - 借用集合
    for &item in v.iter() {
        println!("{}", item);
    }
    println!("{:?}", v); // v 仍然可用

    // iter_mut() - 可變借用集合
    let mut v2 = vec![1, 2, 3];
    for item in v2.iter_mut() {
        *item += 10;
    }
    println!("{:?}", v2); // [11, 12, 13]

    // into_iter() - 消耗集合
    let v3 = vec![1, 2, 3];
    for item in v3.into_iter() {
        println!("{}", item);
    }
    // println!("{:?}", v3); // 錯誤！v3 已被移動

    // for 迴圈會自動呼叫 into_iter()
    let v4 = vec![1, 2, 3];
    for item in v4 {  // 等同於 v4.into_iter()
        println!("{}", item);
    }

    // 對引用使用 for 迴圈
    let v5 = vec![1, 2, 3];
    for item in &v5 {  // 等同於 v5.iter()
        println!("{}", item);
    }

    for item in &mut vec![1, 2, 3] {  // 等同於 iter_mut()
        *item += 1;
    }
}
```

### next 方法

```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    // 手動呼叫 next
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None); // 迭代結束
    assert_eq!(iter.next(), None); // 之後都是 None
}
```

---

## 14.2 迭代器適配器

迭代器適配器（Iterator Adapters）是將一個迭代器轉換成另一個迭代器的方法。它們是惰性的，只有在消費者方法被呼叫時才會執行。

### map

將每個元素轉換成另一個值：

```rust
fn main() {
    let v = vec![1, 2, 3];

    // map 是惰性的，這行不會執行任何計算
    let mapped = v.iter().map(|x| x * 2);

    // 需要消費者來觸發計算
    let result: Vec<i32> = mapped.collect();
    println!("{:?}", result); // [2, 4, 6]

    // 鏈式呼叫
    let result2: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .map(|x| x + 1)
        .collect();
    println!("{:?}", result2); // [3, 5, 7]
}
```

### filter

過濾元素：

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    // 只保留偶數
    let evens: Vec<i32> = v.iter()
        .filter(|&&x| x % 2 == 0)
        .copied()  // 將 &i32 轉換為 i32
        .collect();
    println!("{:?}", evens); // [2, 4, 6]

    // filter_map：結合 filter 和 map
    let strings = vec!["1", "two", "3", "four", "5"];
    let numbers: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("{:?}", numbers); // [1, 3, 5]
}
```

### take 與 skip

```rust
fn main() {
    let v: Vec<i32> = (1..=10).collect();

    // take：只取前 n 個
    let first_three: Vec<_> = v.iter().take(3).collect();
    println!("{:?}", first_three); // [1, 2, 3]

    // skip：跳過前 n 個
    let skip_three: Vec<_> = v.iter().skip(3).collect();
    println!("{:?}", skip_three); // [4, 5, 6, 7, 8, 9, 10]

    // take_while：取元素直到條件不滿足
    let take_while: Vec<_> = v.iter()
        .take_while(|&&x| x < 5)
        .collect();
    println!("{:?}", take_while); // [1, 2, 3, 4]

    // skip_while：跳過元素直到條件不滿足
    let skip_while: Vec<_> = v.iter()
        .skip_while(|&&x| x < 5)
        .collect();
    println!("{:?}", skip_while); // [5, 6, 7, 8, 9, 10]
}
```

### enumerate

加上索引：

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }
    // 0: a
    // 1: b
    // 2: c

    // 找出特定元素的索引
    let position = v.iter()
        .enumerate()
        .find(|(_, &c)| c == 'b')
        .map(|(i, _)| i);
    println!("'b' 的位置: {:?}", position); // Some(1)
}
```

### zip

合併兩個迭代器：

```rust
fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];

    let people: Vec<_> = names.iter()
        .zip(ages.iter())
        .collect();
    println!("{:?}", people);
    // [("Alice", 30), ("Bob", 25), ("Charlie", 35)]

    // 解壓縮
    let (names2, ages2): (Vec<_>, Vec<_>) = people.into_iter().unzip();
    println!("{:?}, {:?}", names2, ages2);

    // 長度不同時，以較短者為準
    let a = [1, 2, 3];
    let b = [4, 5];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("{:?}", zipped); // [(1, 4), (2, 5)]
}
```

### chain、flatten、flat_map

```rust
fn main() {
    // chain：連接兩個迭代器
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let chained: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("{:?}", chained); // [1, 2, 3, 4, 5, 6]

    // flatten：展平巢狀迭代器
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<_> = nested.into_iter().flatten().collect();
    println!("{:?}", flat); // [1, 2, 3, 4, 5]

    // flat_map：map + flatten
    let words = vec!["hello", "world"];
    let chars: Vec<_> = words.iter()
        .flat_map(|s| s.chars())
        .collect();
    println!("{:?}", chars); // ['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
}
```

### peekable 與 fuse

```rust
fn main() {
    // peekable：可以查看下一個元素而不消耗它
    let v = vec![1, 2, 3];
    let mut iter = v.iter().peekable();

    // 查看下一個元素
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.peek(), Some(&&1)); // 仍然是 1

    // 消耗元素
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.peek(), Some(&&2)); // 現在是 2

    // fuse：確保一旦回傳 None 後，永遠回傳 None
    // 某些迭代器在回傳 None 後可能再次回傳 Some
    // fuse() 保證這不會發生
    let iter = vec![1, 2].into_iter().fuse();
}
```

### 其他常用適配器

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // rev：反轉迭代器（需要 DoubleEndedIterator）
    let reversed: Vec<_> = v.iter().rev().collect();
    println!("{:?}", reversed); // [6, 2, 9, 5, 1, 4, 1, 3]

    // cloned：將 &T 轉換為 T（需要 T: Clone）
    let cloned: Vec<i32> = v.iter().cloned().collect();

    // copied：將 &T 轉換為 T（需要 T: Copy）
    let copied: Vec<i32> = v.iter().copied().collect();

    // cycle：無限循環
    let cycled: Vec<_> = [1, 2, 3].iter().cycle().take(7).collect();
    println!("{:?}", cycled); // [1, 2, 3, 1, 2, 3, 1]

    // step_by：間隔取值
    let stepped: Vec<_> = (0..10).step_by(2).collect();
    println!("{:?}", stepped); // [0, 2, 4, 6, 8]

    // inspect：除錯用，不改變元素
    let result: Vec<_> = [1, 2, 3].iter()
        .inspect(|x| println!("處理前: {}", x))
        .map(|x| x * 2)
        .inspect(|x| println!("處理後: {}", x))
        .collect();
}
```

---

## 14.3 消費者方法

消費者方法（Consuming Methods）會消耗迭代器並產生最終結果。

### collect

將迭代器轉換為集合：

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    // 收集到 Vec
    let v: Vec<i32> = (1..=5).collect();

    // 收集到 HashSet
    let set: HashSet<i32> = [1, 2, 2, 3].into_iter().collect();
    println!("{:?}", set); // {1, 2, 3}

    // 收集到 HashMap
    let pairs = [("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("{:?}", map);

    // 收集到 String
    let chars = ['H', 'e', 'l', 'l', 'o'];
    let s: String = chars.iter().collect();
    println!("{}", s); // Hello

    // 使用 turbofish 語法
    let v2 = (1..=5).collect::<Vec<_>>();
}
```

### fold 與 reduce

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // fold：帶初始值的累積操作
    let sum = v.iter().fold(0, |acc, &x| acc + x);
    println!("總和: {}", sum); // 15

    let product = v.iter().fold(1, |acc, &x| acc * x);
    println!("乘積: {}", product); // 120

    // 用 fold 建立字串
    let words = vec!["hello", "world", "rust"];
    let sentence = words.iter()
        .fold(String::new(), |mut acc, &word| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(word);
            acc
        });
    println!("{}", sentence); // hello world rust

    // reduce：無初始值，使用第一個元素作為初始值
    let max = v.iter().copied().reduce(|a, b| if a > b { a } else { b });
    println!("最大值: {:?}", max); // Some(5)

    // 空迭代器的 reduce 回傳 None
    let empty: Vec<i32> = vec![];
    let result = empty.iter().copied().reduce(|a, b| a + b);
    println!("{:?}", result); // None
}
```

### sum 與 product

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // sum
    let total: i32 = v.iter().sum();
    println!("總和: {}", total); // 15

    // product
    let product: i32 = v.iter().product();
    println!("乘積: {}", product); // 120

    // 浮點數
    let floats = vec![1.5, 2.5, 3.0];
    let sum: f64 = floats.iter().sum();
    println!("浮點總和: {}", sum); // 7.0
}
```

### any 與 all

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // any：是否存在滿足條件的元素
    let has_even = v.iter().any(|&x| x % 2 == 0);
    println!("有偶數: {}", has_even); // true

    // all：是否所有元素都滿足條件
    let all_positive = v.iter().all(|&x| x > 0);
    println!("全部正數: {}", all_positive); // true

    let all_even = v.iter().all(|&x| x % 2 == 0);
    println!("全部偶數: {}", all_even); // false
}
```

### find 與 position

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // find：找到第一個滿足條件的元素
    let first_even = v.iter().find(|&&x| x % 2 == 0);
    println!("第一個偶數: {:?}", first_even); // Some(&2)

    // find_map：find + map
    let strings = vec!["a", "1", "b", "2"];
    let first_number = strings.iter()
        .find_map(|s| s.parse::<i32>().ok());
    println!("第一個數字: {:?}", first_number); // Some(1)

    // position：找到第一個滿足條件的元素的索引
    let pos = v.iter().position(|&x| x == 3);
    println!("3 的位置: {:?}", pos); // Some(2)

    // rposition：從後面找
    let v2 = vec![1, 2, 3, 2, 1];
    let last_pos = v2.iter().rposition(|&x| x == 2);
    println!("最後一個 2 的位置: {:?}", last_pos); // Some(3)
}
```

### max、min 與相關方法

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    // max 和 min
    println!("最大值: {:?}", v.iter().max()); // Some(&9)
    println!("最小值: {:?}", v.iter().min()); // Some(&1)

    // max_by 和 min_by：自訂比較函式
    let words = vec!["hello", "hi", "goodbye"];
    let longest = words.iter().max_by(|a, b| a.len().cmp(&b.len()));
    println!("最長的單字: {:?}", longest); // Some(&"goodbye")

    // max_by_key 和 min_by_key：根據某個鍵比較
    let shortest = words.iter().min_by_key(|s| s.len());
    println!("最短的單字: {:?}", shortest); // Some(&"hi")

    // 結構體的比較
    #[derive(Debug)]
    struct Person { name: String, age: u32 }

    let people = vec![
        Person { name: "Alice".into(), age: 30 },
        Person { name: "Bob".into(), age: 25 },
        Person { name: "Charlie".into(), age: 35 },
    ];

    let oldest = people.iter().max_by_key(|p| p.age);
    println!("最年長: {:?}", oldest);
}
```

### count 與 last

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // count：計算元素數量
    let count = v.iter().count();
    println!("元素數量: {}", count); // 5

    // 配合 filter 使用
    let even_count = v.iter().filter(|&&x| x % 2 == 0).count();
    println!("偶數數量: {}", even_count); // 2

    // last：取得最後一個元素
    let last = v.iter().last();
    println!("最後一個: {:?}", last); // Some(&5)

    // nth：取得第 n 個元素（0-indexed）
    let third = v.iter().nth(2);
    println!("第三個: {:?}", third); // Some(&3)
}
```

### partition

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    // partition：將元素分成兩組
    let (evens, odds): (Vec<_>, Vec<_>) = v.iter()
        .partition(|&&x| x % 2 == 0);

    println!("偶數: {:?}", evens); // [2, 4, 6]
    println!("奇數: {:?}", odds);  // [1, 3, 5]
}
```

---

## 14.4 自訂迭代器

### 實作 Iterator Trait

```rust
struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);

    for n in counter {
        println!("{}", n);
    }
    // 1, 2, 3, 4, 5

    // 可以使用所有迭代器方法
    let sum: u32 = Counter::new(5).sum();
    println!("總和: {}", sum); // 15

    let evens: Vec<_> = Counter::new(10)
        .filter(|&x| x % 2 == 0)
        .collect();
    println!("偶數: {:?}", evens); // [2, 4, 6, 8, 10]
}
```

### IntoIterator Trait

```rust
struct MyCollection {
    items: Vec<i32>,
}

impl MyCollection {
    fn new() -> Self {
        MyCollection { items: Vec::new() }
    }

    fn add(&mut self, item: i32) {
        self.items.push(item);
    }

    // 提供 iter 方法
    fn iter(&self) -> impl Iterator<Item = &i32> {
        self.items.iter()
    }
}

// 實作 IntoIterator 讓 for 迴圈可以直接使用
impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// 為引用實作 IntoIterator
impl<'a> IntoIterator for &'a MyCollection {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

fn main() {
    let mut collection = MyCollection::new();
    collection.add(1);
    collection.add(2);
    collection.add(3);

    // 使用引用迭代
    for item in &collection {
        println!("{}", item);
    }
    println!("{:?}", collection.items); // collection 仍可用

    // 消耗集合迭代
    for item in collection {
        println!("{}", item);
    }
    // collection 已被消耗
}
```

### 雙向迭代器

```rust
struct TwoWayCounter {
    current: i32,
    front: i32,
    back: i32,
}

impl TwoWayCounter {
    fn new(start: i32, end: i32) -> Self {
        TwoWayCounter {
            current: start,
            front: start,
            back: end,
        }
    }
}

impl Iterator for TwoWayCounter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let result = self.front;
            self.front += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for TwoWayCounter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back >= self.front {
            let result = self.back;
            self.back -= 1;
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let counter = TwoWayCounter::new(1, 5);

    // 正向迭代
    let forward: Vec<_> = TwoWayCounter::new(1, 5).collect();
    println!("正向: {:?}", forward); // [1, 2, 3, 4, 5]

    // 反向迭代
    let backward: Vec<_> = TwoWayCounter::new(1, 5).rev().collect();
    println!("反向: {:?}", backward); // [5, 4, 3, 2, 1]
}
```

---

## 14.5 效能考量

### 惰性求值

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 這個鏈不會立即執行
    let lazy = v.iter()
        .map(|x| {
            println!("map: {}", x);
            x * 2
        })
        .filter(|x| {
            println!("filter: {}", x);
            x > &5
        });

    println!("迭代器已建立，但尚未執行");

    // 直到調用消費者方法才會執行
    let result: Vec<_> = lazy.collect();
    println!("結果: {:?}", result);

    // 輸出順序顯示是逐元素處理，而非逐步驟處理：
    // map: 1
    // filter: 2
    // map: 2
    // filter: 4
    // map: 3
    // filter: 6
    // ...
}
```

### 迭代器 vs 迴圈

```rust
fn main() {
    let numbers: Vec<i32> = (1..=1000).collect();

    // 迭代器風格
    let sum1: i32 = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();

    // 傳統迴圈風格
    let mut sum2: i32 = 0;
    for &x in &numbers {
        if x % 2 == 0 {
            sum2 += x * x;
        }
    }

    assert_eq!(sum1, sum2);

    // 兩種風格在 Release 模式下效能相當
    // 迭代器版本可能更容易被編譯器最佳化
}
```

### 編譯器最佳化

```rust
fn main() {
    // 迭代器會被編譯成與手寫迴圈相同的機器碼
    // 這是「零成本抽象」的體現

    let v: Vec<i32> = (1..=100).collect();

    // 這段程式碼...
    let sum: i32 = v.iter().sum();

    // 會被最佳化成類似這樣的程式碼：
    // let mut sum = 0;
    // for i in 0..v.len() {
    //     sum += v[i];
    // }

    // 甚至可能使用 SIMD 指令進一步加速
}
```

### 避免不必要的配置

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 不好：中間產生了 Vec
    let result1: Vec<i32> = v.iter()
        .map(|&x| x * 2)
        .collect::<Vec<_>>()  // 不必要的中間 Vec
        .into_iter()
        .filter(|&x| x > 5)
        .collect();

    // 好：鏈式操作，沒有中間配置
    let result2: Vec<i32> = v.iter()
        .map(|&x| x * 2)
        .filter(|&x| x > 5)
        .collect();

    assert_eq!(result1, result2);
}
```

### 選擇正確的迭代器方法

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 用 find 而非 filter + next
    // 好
    let first_even = v.iter().find(|&&x| x % 2 == 0);

    // 不好（效果相同但意圖不清楚）
    let first_even2 = v.iter().filter(|&&x| x % 2 == 0).next();

    // 用 any/all 而非 filter + count/len
    // 好
    let has_even = v.iter().any(|&x| x % 2 == 0);

    // 不好
    let has_even2 = v.iter().filter(|&&x| x % 2 == 0).count() > 0;

    // 用 sum/product 而非 fold
    // 好
    let total: i32 = v.iter().sum();

    // 不必要的複雜
    let total2: i32 = v.iter().fold(0, |acc, &x| acc + x);
}
```

---

## 練習題

### 練習 1：實作 Fibonacci 迭代器

```rust
struct Fibonacci {
    // 定義欄位
}

impl Fibonacci {
    fn new() -> Self {
        todo!()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    // 預期輸出前 10 個 Fibonacci 數
    let fibs: Vec<_> = Fibonacci::new().take(10).collect();
    // [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
}
```

### 練習 2：使用迭代器處理資料

```rust
fn process_scores(scores: Vec<(String, i32)>) -> Vec<String> {
    // 1. 過濾分數 >= 60 的學生
    // 2. 按分數降序排列
    // 3. 只取前 3 名
    // 4. 轉換為 "姓名: 分數" 的格式
    todo!()
}
```

### 練習 3：實作 Chunks 迭代器

```rust
struct Chunks<'a, T> {
    // 將切片分成固定大小的塊
}

impl<'a, T> Iterator for Chunks<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
```

---

## 本章小結

- **Iterator Trait**：統一的迭代介面，核心是 `next()` 方法
- **iter、iter_mut、into_iter**：三種迭代方式
- **迭代器適配器**：惰性轉換迭代器（map、filter、take 等）
- **消費者方法**：觸發計算並產生結果（collect、fold、sum 等）
- **自訂迭代器**：實作 `Iterator` trait
- **效能**：零成本抽象，與手寫迴圈效能相當

---

## 延伸閱讀

- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust By Example - Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Rust Performance Book - Iterators](https://nnethercote.github.io/perf-book/iterators.html)
