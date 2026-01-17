# 第 13 章：集合型別

> 集合是儲存多個值的資料結構，Rust 標準庫提供了豐富的集合型別

---

## 13.1 Vec\<T\>

`Vec<T>`（向量）是 Rust 中最常用的集合型別，可以儲存任意數量的同型別元素。

### 建立與基本操作

```rust
fn main() {
    // 建立空向量
    let mut v1: Vec<i32> = Vec::new();

    // 使用 vec! 巨集建立
    let v2 = vec![1, 2, 3];

    // 指定初始值和數量
    let v3 = vec![0; 5]; // [0, 0, 0, 0, 0]

    // 使用 with_capacity 預先配置容量
    let mut v4: Vec<i32> = Vec::with_capacity(100);

    // 新增元素
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // 移除並回傳最後一個元素
    let last = v1.pop(); // Some(3)

    // 插入到指定位置
    v1.insert(0, 0); // [0, 1, 2]

    // 移除指定位置的元素
    let removed = v1.remove(0); // 0, v1 = [1, 2]

    println!("{:?}", v1);
}
```

### 存取元素

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引（可能 panic）
    let third = v[2];
    println!("第三個元素: {}", third);

    // 使用 get（回傳 Option）
    match v.get(2) {
        Some(value) => println!("第三個元素: {}", value),
        None => println!("沒有第三個元素"),
    }

    // 安全地存取可能不存在的索引
    let tenth = v.get(10); // None，不會 panic

    // 第一個和最後一個
    if let Some(first) = v.first() {
        println!("第一個: {}", first);
    }

    if let Some(last) = v.last() {
        println!("最後一個: {}", last);
    }
}
```

### 迭代

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 不可變迭代
    for item in &v {
        println!("{}", item);
    }

    // 可變迭代
    let mut v2 = vec![1, 2, 3];
    for item in &mut v2 {
        *item *= 2;
    }
    println!("{:?}", v2); // [2, 4, 6]

    // 消耗迭代（取得所有權）
    let v3 = vec![String::from("a"), String::from("b")];
    for item in v3 {
        println!("{}", item);
    }
    // v3 已經被消耗，無法再使用

    // 帶索引迭代
    let v4 = vec!['a', 'b', 'c'];
    for (index, value) in v4.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}
```

### 容量與長度

```rust
fn main() {
    let mut v = Vec::with_capacity(10);

    println!("長度: {}", v.len());       // 0
    println!("容量: {}", v.capacity()); // 10

    // 新增元素不會重新配置（因為有足夠容量）
    for i in 0..10 {
        v.push(i);
    }

    println!("長度: {}", v.len());       // 10
    println!("容量: {}", v.capacity()); // 10

    // 超過容量時會自動擴展
    v.push(10);
    println!("長度: {}", v.len());       // 11
    println!("容量: {}", v.capacity()); // 20（通常是加倍）

    // 釋放多餘容量
    v.shrink_to_fit();
    println!("容量: {}", v.capacity()); // 11

    // 清空向量（保留容量）
    v.clear();
    println!("長度: {}", v.len());       // 0
    println!("容量: {}", v.capacity()); // 11
}
```

### 切片操作

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 取得切片
    let slice = &v[1..4]; // [2, 3, 4]
    println!("{:?}", slice);

    // 全部切片
    let all: &[i32] = &v[..];

    // 從頭開始
    let from_start = &v[..3]; // [1, 2, 3]

    // 到結尾
    let to_end = &v[2..]; // [3, 4, 5]

    // 函式接受切片更靈活
    fn sum_slice(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }

    println!("總和: {}", sum_slice(&v));     // 傳入 Vec
    println!("部分總和: {}", sum_slice(&v[1..4])); // 傳入切片
}
```

### 記憶體佈局

```rust
fn main() {
    // Vec 在記憶體中的結構：
    // - 指向堆積上資料的指標
    // - 長度（目前元素數量）
    // - 容量（已配置空間）

    let v = vec![1, 2, 3];

    // Vec 本身在棧上（24 bytes：指標 + 長度 + 容量）
    // 實際資料在堆積上

    // 使用 std::mem 查看大小
    use std::mem;
    println!("Vec<i32> 大小: {} bytes", mem::size_of::<Vec<i32>>()); // 24
    println!("&[i32] 大小: {} bytes", mem::size_of::<&[i32]>());     // 16（胖指標）

    // 元素連續儲存，cache 友好
    let data = v.as_ptr();
    unsafe {
        println!("第一個元素位址: {:p}", data);
        println!("第二個元素位址: {:p}", data.add(1));
    }
}
```

---

## 13.2 String

`String` 是 Rust 中可增長、可修改、擁有所有權的 UTF-8 字串型別。

### String vs &str

```rust
fn main() {
    // String：擁有所有權的字串，可修改
    let mut s = String::from("hello");
    s.push_str(" world");

    // &str：字串切片，不可修改
    let slice: &str = "hello";
    let string_slice: &str = &s;

    // 轉換
    let s1: String = "hello".to_string();
    let s2: String = String::from("hello");
    let slice2: &str = &s1;
}
```

### 建立字串

```rust
fn main() {
    // 空字串
    let s1 = String::new();

    // 從字面值
    let s2 = String::from("hello");
    let s3 = "hello".to_string();

    // 指定容量
    let s4 = String::with_capacity(100);

    // 從字元迭代器
    let s5: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();

    // 從 bytes
    let s6 = String::from_utf8(vec![104, 101, 108, 108, 111]).unwrap();
    println!("{}", s6); // hello

    // 重複字串
    let s7 = "ab".repeat(3);
    println!("{}", s7); // ababab
}
```

### 字串操作

```rust
fn main() {
    let mut s = String::from("hello");

    // 附加字串
    s.push_str(" world");
    println!("{}", s); // hello world

    // 附加字元
    s.push('!');
    println!("{}", s); // hello world!

    // 字串連接
    let s1 = String::from("hello");
    let s2 = String::from(" world");

    // + 運算子（s1 被移動）
    let s3 = s1 + &s2;
    // s1 不再有效

    // format! 巨集（不移動任何值）
    let s4 = String::from("hello");
    let s5 = format!("{} {}", s4, s2);
    println!("{}", s4); // 仍然有效

    // 插入
    let mut s6 = String::from("hello");
    s6.insert(5, '!');
    println!("{}", s6); // hello!

    s6.insert_str(0, "oh ");
    println!("{}", s6); // oh hello!

    // 替換
    let s7 = String::from("hello world world");
    let s8 = s7.replace("world", "Rust");
    println!("{}", s8); // hello Rust Rust

    let s9 = s7.replacen("world", "Rust", 1);
    println!("{}", s9); // hello Rust world

    // 刪除
    let mut s10 = String::from("hello");
    s10.pop();           // 移除最後一個字元
    println!("{}", s10); // hell

    s10.remove(0);       // 移除指定位置
    println!("{}", s10); // ell

    s10.truncate(2);     // 截斷到指定長度
    println!("{}", s10); // el

    s10.clear();         // 清空
    println!("{}", s10); // ""
}
```

### UTF-8 編碼

```rust
fn main() {
    let s = String::from("你好世界");

    // 長度（位元組數）
    println!("位元組長度: {}", s.len()); // 12（每個中文字 3 bytes）

    // 字元數
    println!("字元數: {}", s.chars().count()); // 4

    // 遍歷字元
    for c in s.chars() {
        println!("{}", c);
    }

    // 遍歷位元組
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();

    // 字串索引
    // let first = s[0]; // 錯誤！Rust 不允許直接索引

    // 使用切片（必須在字元邊界）
    let first_char = &s[0..3]; // "你"
    println!("{}", first_char);

    // let bad = &s[0..1]; // panic！不在字元邊界

    // 安全地取得第 n 個字元
    let third = s.chars().nth(2);
    println!("{:?}", third); // Some('世')
}
```

### 字串切片

```rust
fn main() {
    let s = String::from("hello world");

    // 取得切片
    let hello = &s[0..5];
    let world = &s[6..11];

    // 常用方法
    println!("開頭是 hello: {}", s.starts_with("hello"));
    println!("結尾是 world: {}", s.ends_with("world"));
    println!("包含 lo: {}", s.contains("lo"));

    // 分割
    let parts: Vec<&str> = s.split(' ').collect();
    println!("{:?}", parts); // ["hello", "world"]

    // 去除空白
    let s2 = "  hello  ";
    println!("'{}'", s2.trim());       // 'hello'
    println!("'{}'", s2.trim_start()); // 'hello  '
    println!("'{}'", s2.trim_end());   // '  hello'

    // 大小寫
    let s3 = "Hello World";
    println!("{}", s3.to_lowercase()); // hello world
    println!("{}", s3.to_uppercase()); // HELLO WORLD

    // 分割成行
    let multiline = "line1\nline2\nline3";
    for line in multiline.lines() {
        println!("{}", line);
    }
}
```

---

## 13.3 HashMap\<K, V\>

`HashMap<K, V>` 儲存鍵值對，提供 O(1) 的平均查找時間。

### 建立與基本操作

```rust
use std::collections::HashMap;

fn main() {
    // 建立空 HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // 插入
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // 使用 collect 從元組建立
    let teams = vec!["Blue", "Red"];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    // 從陣列建立
    let scores3 = HashMap::from([
        ("Blue", 10),
        ("Red", 50),
    ]);

    // 查找
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue 隊分數: {}", s),
        None => println!("找不到 Blue 隊"),
    }

    // 使用 copied() 取得值而非引用
    let score: i32 = scores.get("Blue").copied().unwrap_or(0);

    // 遍歷
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 移除
    let removed = scores.remove("Blue");
    println!("移除的值: {:?}", removed);
}
```

### 更新值

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);

    // 直接覆寫
    scores.insert("Blue", 25);
    println!("{:?}", scores); // {"Blue": 25}

    // 只在鍵不存在時插入
    scores.entry("Yellow").or_insert(50);
    scores.entry("Blue").or_insert(50); // 不會更新，因為已存在
    println!("{:?}", scores); // {"Blue": 25, "Yellow": 50}

    // 根據舊值更新
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
    // {"hello": 1, "world": 2, "wonderful": 1}
}
```

### Entry API

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();

    // or_insert_with：只在需要時才計算預設值
    map.entry("numbers")
        .or_insert_with(Vec::new)
        .push(1);

    map.entry("numbers")
        .or_insert_with(Vec::new)
        .push(2);

    println!("{:?}", map); // {"numbers": [1, 2]}

    // and_modify：修改現有值
    let mut scores = HashMap::new();
    scores.insert("player1", 100);

    scores.entry("player1")
        .and_modify(|v| *v += 10)
        .or_insert(0);

    scores.entry("player2")
        .and_modify(|v| *v += 10)
        .or_insert(0);

    println!("{:?}", scores); // {"player1": 110, "player2": 0}

    // or_default：使用 Default trait
    let mut counts: HashMap<&str, i32> = HashMap::new();
    *counts.entry("a").or_default() += 1;
    *counts.entry("a").or_default() += 1;
    println!("{:?}", counts); // {"a": 2}
}
```

### 自訂鍵型別

```rust
use std::collections::HashMap;

// 要作為 HashMap 的鍵，必須實作 Eq + Hash
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut locations: HashMap<Point, &str> = HashMap::new();

    locations.insert(Point { x: 0, y: 0 }, "原點");
    locations.insert(Point { x: 1, y: 0 }, "東");
    locations.insert(Point { x: 0, y: 1 }, "北");

    let origin = Point { x: 0, y: 0 };
    if let Some(name) = locations.get(&origin) {
        println!("{:?} 是 {}", origin, name);
    }
}
```

### 自訂雜湊函式

```rust
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

// 使用第三方 crate 的更快雜湊函式
// use fnv::FnvHasher;
// type FnvHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

// 或使用標準庫的 RandomState（預設）
fn main() {
    // 預設使用 SipHash（安全但較慢）
    let map: HashMap<String, i32> = HashMap::new();

    // 對於不需要防禦 HashDoS 的場景，可以使用更快的雜湊
    // let map: FnvHashMap<String, i32> = FnvHashMap::default();
}
```

---

## 13.4 其他集合

### HashSet

```rust
use std::collections::HashSet;

fn main() {
    // 建立
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2); // 重複，不會加入

    println!("{:?}", set); // {1, 2, 3}

    // 從陣列建立
    let set2: HashSet<_> = [1, 2, 3, 4, 5].into_iter().collect();

    // 檢查是否包含
    println!("包含 2: {}", set.contains(&2));

    // 移除
    set.remove(&2);

    // 集合運算
    let a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let b: HashSet<_> = [2, 3, 4].into_iter().collect();

    // 聯集
    let union: HashSet<_> = a.union(&b).collect();
    println!("聯集: {:?}", union); // {1, 2, 3, 4}

    // 交集
    let intersection: HashSet<_> = a.intersection(&b).collect();
    println!("交集: {:?}", intersection); // {2, 3}

    // 差集
    let difference: HashSet<_> = a.difference(&b).collect();
    println!("a - b: {:?}", difference); // {1}

    // 對稱差集
    let symmetric: HashSet<_> = a.symmetric_difference(&b).collect();
    println!("對稱差集: {:?}", symmetric); // {1, 4}

    // 子集檢查
    let c: HashSet<_> = [2, 3].into_iter().collect();
    println!("c 是 a 的子集: {}", c.is_subset(&a)); // true
}
```

### BTreeMap 與 BTreeSet

有序的集合，基於 B-Tree 實作：

```rust
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    // BTreeMap：有序的 Map
    let mut map = BTreeMap::new();
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");

    // 遍歷時按鍵排序
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    // 1: one
    // 2: two
    // 3: three

    // 範圍查詢
    for (key, value) in map.range(1..3) {
        println!("{}: {}", key, value);
    }

    // BTreeSet：有序的 Set
    let mut set = BTreeSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);

    // 有序遍歷
    for item in &set {
        println!("{}", item);
    }
    // 1
    // 2
    // 3

    // 範圍
    let range: Vec<_> = set.range(1..3).collect();
    println!("{:?}", range); // [1, 2]
}
```

### VecDeque（雙端佇列）

```rust
use std::collections::VecDeque;

fn main() {
    let mut deque: VecDeque<i32> = VecDeque::new();

    // 從後面加入
    deque.push_back(1);
    deque.push_back(2);

    // 從前面加入
    deque.push_front(0);

    println!("{:?}", deque); // [0, 1, 2]

    // 從前面移除
    let front = deque.pop_front();
    println!("{:?}", front); // Some(0)

    // 從後面移除
    let back = deque.pop_back();
    println!("{:?}", back); // Some(2)

    // 存取
    println!("{:?}", deque.front()); // Some(&1)
    println!("{:?}", deque.back());  // Some(&1)

    // 使用場景：實作佇列（FIFO）
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("first");
    queue.push_back("second");
    queue.push_back("third");

    while let Some(item) = queue.pop_front() {
        println!("處理: {}", item);
    }
}
```

### LinkedList（雙向鏈結串列）

```rust
use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_front(0);

    println!("{:?}", list); // [0, 1, 2]

    // 注意：LinkedList 在大多數情況下效能不如 Vec 或 VecDeque
    // 只有在需要頻繁在中間插入/刪除時才考慮使用
}
```

### BinaryHeap（二元堆積）

```rust
use std::collections::BinaryHeap;

fn main() {
    // 最大堆積（Max-Heap）
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    // 最大值在頂端
    println!("最大值: {:?}", heap.peek()); // Some(&5)

    // pop 會按順序取出最大值
    while let Some(max) = heap.pop() {
        print!("{} ", max);
    }
    println!(); // 5 4 3 1 1

    // 使用場景：優先佇列
    #[derive(Debug, Eq, PartialEq)]
    struct Task {
        priority: i32,
        name: String,
    }

    // 實作 Ord 讓 BinaryHeap 可以比較
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut task_queue = BinaryHeap::new();
    task_queue.push(Task { priority: 1, name: "低優先".into() });
    task_queue.push(Task { priority: 3, name: "高優先".into() });
    task_queue.push(Task { priority: 2, name: "中優先".into() });

    while let Some(task) = task_queue.pop() {
        println!("執行任務: {} (優先級 {})", task.name, task.priority);
    }
    // 執行任務: 高優先 (優先級 3)
    // 執行任務: 中優先 (優先級 2)
    // 執行任務: 低優先 (優先級 1)
}
```

### 集合選擇指南

| 需求 | 推薦的集合 |
|------|-----------|
| 按索引存取 | `Vec<T>` |
| 需要有序 | `BTreeMap`、`BTreeSet` |
| 需要快速查找 | `HashMap`、`HashSet` |
| FIFO 佇列 | `VecDeque<T>` |
| 優先佇列 | `BinaryHeap<T>` |
| 頻繁在中間插入/刪除 | `LinkedList<T>`（謹慎使用） |

---

## 練習題

### 練習 1：單字計數器

```rust
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    // 統計每個單字出現的次數
    // 忽略大小寫
    todo!()
}

fn main() {
    let text = "Hello world hello Rust hello";
    let counts = word_count(text);
    // 預期: {"hello": 3, "world": 1, "rust": 1}
}
```

### 練習 2：移除重複值

```rust
fn remove_duplicates(nums: Vec<i32>) -> Vec<i32> {
    // 移除重複值，保持原始順序
    todo!()
}

fn main() {
    let nums = vec![1, 2, 2, 3, 1, 4, 3];
    let unique = remove_duplicates(nums);
    // 預期: [1, 2, 3, 4]
}
```

### 練習 3：兩個陣列的交集

```rust
use std::collections::HashSet;

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 找出兩個陣列的交集
    todo!()
}
```

---

## 本章小結

- **Vec\<T\>**：最常用的動態陣列，連續記憶體儲存
- **String**：擁有所有權的 UTF-8 字串
- **HashMap\<K, V\>**：雜湊表，O(1) 平均查找
- **HashSet\<T\>**：不重複值的集合
- **BTreeMap/BTreeSet**：有序的 Map 和 Set
- **VecDeque**：雙端佇列，適合 FIFO 操作
- **BinaryHeap**：優先佇列

---

## 延伸閱讀

- [The Rust Book - Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust By Example - Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)
- [std::collections Documentation](https://doc.rust-lang.org/std/collections/)
