# 第 15 章：智慧指標

> 智慧指標是擁有額外能力的指標，可以管理記憶體配置、提供共享所有權或內部可變性

---

## 15.1 Box\<T\>

`Box<T>` 是最簡單的智慧指標，它在堆積（heap）上配置資料。

### 基本用法

```rust
fn main() {
    // 在堆積上配置 i32
    let b = Box::new(5);
    println!("b = {}", b);

    // 自動解引用
    let x = *b + 1;
    println!("x = {}", x);

    // 當 Box 離開作用域時，堆積上的資料會被釋放
}
```

### 使用場景

**1. 遞迴型別**

```rust
// 錯誤：編譯器無法確定大小
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// 正確：使用 Box 讓大小固定
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 遍歷鏈結串列
    fn print_list(list: &List) {
        match list {
            Cons(value, next) => {
                println!("{}", value);
                print_list(next);
            }
            Nil => println!("結束"),
        }
    }

    print_list(&list);
}
```

**2. 大型資料避免複製**

```rust
fn main() {
    // 大型陣列在棧上
    let large_array = [0u8; 1_000_000];

    // 移動（實際上是複製到新位置）
    let moved = large_array; // 複製 1MB 資料

    // 使用 Box：只複製指標
    let boxed = Box::new([0u8; 1_000_000]);
    let moved_box = boxed; // 只複製 8 bytes（指標）
}
```

**3. Trait 物件**

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("繪製圓形，半徑: {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Draw for Square {
    fn draw(&self) {
        println!("繪製正方形，邊長: {}", self.side);
    }
}

fn main() {
    // 使用 Box<dyn Trait> 儲存不同型別
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 10.0 }),
    ];

    for shape in shapes {
        shape.draw();
    }
}
```

### Box 與記憶體佈局

```rust
use std::mem;

fn main() {
    // Box<T> 本身只有一個指標大小
    println!("Box<i32> 大小: {} bytes", mem::size_of::<Box<i32>>()); // 8

    // 即使內部型別很大
    println!("Box<[u8; 1000]> 大小: {} bytes",
        mem::size_of::<Box<[u8; 1000]>>()); // 8

    // 解引用取得內部值
    let boxed = Box::new(42);
    let value = *boxed; // 移動出來
    // boxed 不再有效

    // 借用內部值
    let boxed2 = Box::new(42);
    let reference = &*boxed2; // 借用
    println!("{}", reference);
    println!("{}", boxed2); // boxed2 仍有效
}
```

---

## 15.2 Rc\<T\> 與 Arc\<T\>

`Rc<T>`（Reference Counted）和 `Arc<T>`（Atomic Reference Counted）提供共享所有權。

### Rc\<T\>：單執行緒共享所有權

```rust
use std::rc::Rc;

fn main() {
    // 建立 Rc
    let a = Rc::new(String::from("hello"));
    println!("引用計數: {}", Rc::strong_count(&a)); // 1

    // 複製 Rc（增加引用計數）
    let b = Rc::clone(&a);
    println!("引用計數: {}", Rc::strong_count(&a)); // 2

    let c = Rc::clone(&a);
    println!("引用計數: {}", Rc::strong_count(&a)); // 3

    // 所有 Rc 都指向同一份資料
    println!("{}, {}, {}", a, b, c);

    // 當 Rc 離開作用域時，引用計數減少
    drop(c);
    println!("drop c 後引用計數: {}", Rc::strong_count(&a)); // 2

    drop(b);
    println!("drop b 後引用計數: {}", Rc::strong_count(&a)); // 1

    // 當最後一個 Rc 被丟棄時，資料才會被釋放
}
```

### 使用 Rc 建立共享資料結構

```rust
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 建立共享的尾部
    let tail = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // 兩個串列共享相同的尾部
    let list_a = Cons(3, Rc::clone(&tail));
    let list_b = Cons(4, Rc::clone(&tail));

    println!("tail 引用計數: {}", Rc::strong_count(&tail)); // 3

    //     list_a: 3 -> \
    //                    5 -> 10 -> Nil
    //     list_b: 4 -> /
}
```

### Arc\<T\>：多執行緒共享所有權

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];

    for i in 0..3 {
        // 複製 Arc 給每個執行緒
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            println!("執行緒 {}: {:?}", i, data_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("主執行緒: {:?}", data);
}
```

### Weak\<T\>：弱引用

弱引用不會增加強引用計數，用於避免迴圈引用：

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,  // 弱引用指向父節點
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf 強引用: {}", Rc::strong_count(&leaf)); // 1
    println!("leaf 弱引用: {}", Rc::weak_count(&leaf));   // 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // 設定 leaf 的父節點（使用弱引用）
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch 強引用: {}", Rc::strong_count(&branch)); // 1
        println!("branch 弱引用: {}", Rc::weak_count(&branch));   // 1
        println!("leaf 強引用: {}", Rc::strong_count(&leaf));     // 2

        // 嘗試存取父節點
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("leaf 的父節點值: {}", parent.value);
        }
    }

    // branch 離開作用域，被丟棄
    // leaf 的父引用現在是無效的
    println!("leaf 的父節點: {:?}", leaf.parent.borrow().upgrade()); // None
    println!("leaf 強引用: {}", Rc::strong_count(&leaf)); // 1
}
```

### 迴圈引用問題

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("丟棄 Node({})", self.value);
    }
}

fn main() {
    let a = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let b = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&a))),
    });

    // 建立迴圈引用：a -> b -> a
    *a.next.borrow_mut() = Some(Rc::clone(&b));

    println!("a 強引用: {}", Rc::strong_count(&a)); // 2
    println!("b 強引用: {}", Rc::strong_count(&b)); // 2

    // 離開作用域時，a 和 b 都不會被丟棄
    // 因為它們互相引用，引用計數永遠不會變成 0
    // 這造成記憶體洩漏！

    // 解決方案：使用 Weak 打破迴圈
}
```

---

## 15.3 RefCell\<T\> 與 Cell\<T\>

`RefCell<T>` 和 `Cell<T>` 提供「內部可變性」——即使只有不可變引用，也能修改內部值。

### Cell\<T\>：適用於 Copy 型別

```rust
use std::cell::Cell;

fn main() {
    let cell = Cell::new(5);

    // 取得值（複製）
    let value = cell.get();
    println!("值: {}", value); // 5

    // 設定新值
    cell.set(10);
    println!("新值: {}", cell.get()); // 10

    // 即使是不可變的，也能修改內部值
    let immutable_cell = Cell::new(0);
    immutable_cell.set(42);
    println!("{}", immutable_cell.get()); // 42

    // 用於計數器
    struct Counter {
        count: Cell<usize>,
    }

    impl Counter {
        fn new() -> Self {
            Counter { count: Cell::new(0) }
        }

        fn increment(&self) {  // 注意：&self 而非 &mut self
            self.count.set(self.count.get() + 1);
        }

        fn get(&self) -> usize {
            self.count.get()
        }
    }

    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("計數: {}", counter.get()); // 2
}
```

### RefCell\<T\>：執行時借用檢查

```rust
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec![1, 2, 3]);

    // 不可變借用
    {
        let borrowed = cell.borrow();
        println!("借用: {:?}", *borrowed);
    }

    // 可變借用
    {
        let mut borrowed = cell.borrow_mut();
        borrowed.push(4);
    }

    println!("修改後: {:?}", cell.borrow());

    // 執行時期借用規則仍然適用
    // 違反規則會 panic，而非編譯錯誤

    // 同時有可變和不可變借用會 panic
    // let r1 = cell.borrow();
    // let r2 = cell.borrow_mut(); // panic!
}
```

### try_borrow 和 try_borrow_mut

```rust
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(5);

    let r1 = cell.borrow();

    // 嘗試借用，失敗時回傳 Err
    match cell.try_borrow_mut() {
        Ok(mut r) => *r += 1,
        Err(_) => println!("無法取得可變借用"),
    }

    drop(r1);

    // 現在可以借用了
    if let Ok(mut r) = cell.try_borrow_mut() {
        *r += 1;
    }

    println!("值: {}", cell.borrow()); // 6
}
```

### Rc\<RefCell\<T\>\> 組合

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let root = Rc::new(Node {
        value: 0,
        children: RefCell::new(vec![]),
    });

    let child1 = Rc::new(Node {
        value: 1,
        children: RefCell::new(vec![]),
    });

    let child2 = Rc::new(Node {
        value: 2,
        children: RefCell::new(vec![]),
    });

    // 即使 root 是 Rc（不可變），也能修改 children
    root.children.borrow_mut().push(Rc::clone(&child1));
    root.children.borrow_mut().push(Rc::clone(&child2));

    println!("root: {:?}", root);
    println!("children 數量: {}", root.children.borrow().len());
}
```

### 實際應用：Mock 物件

```rust
use std::cell::RefCell;

trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> Self {
        MockMessenger {
            messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // send 接收 &self，但我們需要修改 messages
        // 使用 RefCell 實現內部可變性
        self.messages.borrow_mut().push(String::from(msg));
    }
}

fn main() {
    let mock = MockMessenger::new();

    mock.send("hello");
    mock.send("world");

    // 檢查訊息
    let messages = mock.messages.borrow();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], "hello");
}
```

---

## 15.4 Cow\<T\>

`Cow`（Clone-on-Write）是一個智慧指標，可以持有借用或擁有的資料。

### 基本用法

```rust
use std::borrow::Cow;

fn main() {
    // 持有借用
    let borrowed: Cow<str> = Cow::Borrowed("hello");

    // 持有擁有的資料
    let owned: Cow<str> = Cow::Owned(String::from("world"));

    println!("{}, {}", borrowed, owned);

    // 兩者都可以當作 &str 使用
    fn print_str(s: &str) {
        println!("{}", s);
    }

    print_str(&borrowed);
    print_str(&owned);
}
```

### Clone-on-Write 語義

```rust
use std::borrow::Cow;

fn main() {
    let mut cow: Cow<str> = Cow::Borrowed("hello");

    // 讀取不需要複製
    println!("長度: {}", cow.len());

    // 修改時才會複製（如果是借用的話）
    cow.to_mut().push_str(" world");

    // 現在 cow 持有 Owned 資料
    match &cow {
        Cow::Borrowed(_) => println!("借用"),
        Cow::Owned(_) => println!("擁有"), // 會印出這個
    }

    println!("{}", cow);
}
```

### 實際應用：避免不必要的配置

```rust
use std::borrow::Cow;

// 只在需要時才建立新字串
fn process_name(name: &str) -> Cow<str> {
    if name.contains(' ') {
        // 需要處理，回傳新字串
        Cow::Owned(name.replace(' ', "_"))
    } else {
        // 不需要處理，回傳借用
        Cow::Borrowed(name)
    }
}

fn main() {
    let name1 = "john_doe";
    let name2 = "jane doe";

    let result1 = process_name(name1);
    let result2 = process_name(name2);

    match &result1 {
        Cow::Borrowed(_) => println!("{} 未修改", result1),
        Cow::Owned(_) => println!("{} 已修改", result1),
    }

    match &result2 {
        Cow::Borrowed(_) => println!("{} 未修改", result2),
        Cow::Owned(_) => println!("{} 已修改", result2),
    }
}
```

### Cow 與函式參數

```rust
use std::borrow::Cow;

// 接受 Cow 的函式可以處理借用和擁有的資料
fn log_message(msg: Cow<str>) {
    println!("[LOG] {}", msg);
}

fn main() {
    // 傳入借用
    log_message(Cow::Borrowed("static message"));

    // 傳入擁有
    let owned = String::from("dynamic message");
    log_message(Cow::Owned(owned));

    // 使用 Into trait
    log_message("another static".into());
    log_message(String::from("another dynamic").into());
}
```

---

## 15.5 自訂智慧指標

### Deref Trait

`Deref` trait 允許自訂 `*` 運算子的行為：

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = MyBox::new(5);

    // 使用 * 解引用
    assert_eq!(5, *x);

    // Deref 強制轉型
    let s = MyBox::new(String::from("hello"));

    // &MyBox<String> -> &String -> &str
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    hello(&s);  // Deref 強制轉型讓這行可以運作
}
```

### DerefMut Trait

```rust
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut x = MyBox::new(5);

    *x = 10;  // 使用 DerefMut

    println!("{}", *x); // 10
}
```

### Drop Trait

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("丟棄 CustomSmartPointer，資料: {}", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer {
        data: String::from("a 的資料"),
    };

    let b = CustomSmartPointer {
        data: String::from("b 的資料"),
    };

    println!("CustomSmartPointer 已建立");

    // 提前丟棄
    drop(a);
    println!("a 已丟棄");

    println!("main 結束前");
}
// 輸出：
// CustomSmartPointer 已建立
// 丟棄 CustomSmartPointer，資料: a 的資料
// a 已丟棄
// main 結束前
// 丟棄 CustomSmartPointer，資料: b 的資料
```

### 完整的自訂智慧指標範例

```rust
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::alloc::{alloc, dealloc, Layout};

struct UniquePtr<T> {
    ptr: NonNull<T>,
}

impl<T> UniquePtr<T> {
    fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                panic!("配置失敗");
            }
            ptr.write(value);
            UniquePtr {
                ptr: NonNull::new_unchecked(ptr),
            }
        }
    }
}

impl<T> Deref for UniquePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for UniquePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        unsafe {
            // 先 drop 內部值
            std::ptr::drop_in_place(self.ptr.as_ptr());
            // 再釋放記憶體
            let layout = Layout::new::<T>();
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

fn main() {
    let ptr = UniquePtr::new(String::from("hello"));
    println!("{}", *ptr);

    let mut ptr2 = UniquePtr::new(vec![1, 2, 3]);
    ptr2.push(4);
    println!("{:?}", *ptr2);
}
```

---

## 智慧指標選擇指南

| 需求 | 推薦的智慧指標 |
|------|---------------|
| 堆積配置、單一所有權 | `Box<T>` |
| 單執行緒共享所有權 | `Rc<T>` |
| 多執行緒共享所有權 | `Arc<T>` |
| 內部可變性（Copy 型別） | `Cell<T>` |
| 內部可變性（非 Copy 型別） | `RefCell<T>` |
| 避免不必要的複製 | `Cow<T>` |
| 避免迴圈引用 | `Weak<T>` |
| 執行緒安全的內部可變性 | `Mutex<T>`、`RwLock<T>` |

---

## 練習題

### 練習 1：使用 Box 實作二元樹

```rust
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self { todo!() }
    fn insert(&mut self, value: i32) { todo!() }
    fn contains(&self, value: i32) -> bool { todo!() }
}
```

### 練習 2：使用 Rc 和 RefCell 實作圖

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

struct Node {
    value: i32,
    edges: Vec<Rc<RefCell<Node>>>,
}

// 實作新增節點和邊的方法
```

### 練習 3：實作 Cow 函式

```rust
use std::borrow::Cow;

// 將字串中的多個連續空格替換為單個空格
// 如果不需要修改，回傳借用
fn normalize_whitespace(s: &str) -> Cow<str> {
    todo!()
}
```

---

## 本章小結

- **Box\<T\>**：堆積配置，適用於遞迴型別和大型資料
- **Rc\<T\>/Arc\<T\>**：引用計數的共享所有權
- **Weak\<T\>**：弱引用，用於避免迴圈引用
- **Cell\<T\>/RefCell\<T\>**：內部可變性
- **Cow\<T\>**：Clone-on-Write，避免不必要的複製
- **Deref/DerefMut**：自訂解引用行為
- **Drop**：自訂清理邏輯

---

## 延伸閱讀

- [The Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust By Example - Box, stack and heap](https://doc.rust-lang.org/rust-by-example/std/box.html)
- [std::rc Documentation](https://doc.rust-lang.org/std/rc/)
- [std::cell Documentation](https://doc.rust-lang.org/std/cell/)
