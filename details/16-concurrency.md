# 第 16 章：並行程式設計

> Rust 的所有權系統讓並行程式設計更加安全，在編譯時期就能防止資料競爭

---

## 16.1 執行緒基礎

執行緒（Thread）是作業系統層級的並行執行單位。Rust 標準庫提供了 1:1 執行緒模型。

### 建立執行緒

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 建立新執行緒
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子執行緒: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主執行緒繼續執行
    for i in 1..5 {
        println!("主執行緒: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待子執行緒結束
    handle.join().unwrap();

    println!("所有執行緒已完成");
}
```

### JoinHandle

```rust
use std::thread;

fn main() {
    // spawn 回傳 JoinHandle
    let handle = thread::spawn(|| {
        // 執行緒可以回傳值
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        sum
    });

    // join() 會阻塞直到執行緒結束，並取得回傳值
    let result = handle.join().unwrap();
    println!("1 到 100 的總和: {}", result);

    // 多個執行緒
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("執行緒 {} 開始", i);
                thread::sleep(std::time::Duration::from_millis(100));
                println!("執行緒 {} 結束", i);
                i * 10
            })
        })
        .collect();

    // 等待所有執行緒並收集結果
    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    println!("結果: {:?}", results);
}
```

### move 閉包

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    // 錯誤：閉包借用了 data，但執行緒可能活得比 data 更久
    // let handle = thread::spawn(|| {
    //     println!("{:?}", data);
    // });

    // 正確：使用 move 將所有權轉移給執行緒
    let handle = thread::spawn(move || {
        println!("{:?}", data);
    });

    // data 已被移動，無法在這裡使用
    // println!("{:?}", data); // 錯誤！

    handle.join().unwrap();
}
```

### 執行緒配置

```rust
use std::thread;

fn main() {
    // 使用 Builder 自訂執行緒
    let builder = thread::Builder::new()
        .name("my-thread".to_string())
        .stack_size(32 * 1024); // 32KB 棧大小

    let handle = builder.spawn(|| {
        // 取得當前執行緒
        let current = thread::current();
        println!("執行緒名稱: {:?}", current.name());
        println!("執行緒 ID: {:?}", current.id());
    }).unwrap();

    handle.join().unwrap();

    // 取得 CPU 核心數
    let num_cpus = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("可用的並行度: {}", num_cpus);
}
```

---

## 16.2 訊息傳遞

Rust 提供了 channel（通道）來在執行緒間安全地傳遞資料。

### mpsc Channel

`mpsc` 代表 "multiple producer, single consumer"（多生產者，單消費者）：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 建立 channel，tx 是發送端，rx 是接收端
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("hello");
        tx.send(message).unwrap();
        // message 已被移動，無法再使用
    });

    // recv() 會阻塞直到收到訊息
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}
```

### 發送多個訊息

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("訊息 1"),
            String::from("訊息 2"),
            String::from("訊息 3"),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 將 rx 當作迭代器使用
    for received in rx {
        println!("收到: {}", received);
    }

    println!("通道已關閉");
}
```

### 多生產者

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 複製發送端給多個執行緒
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    drop(tx); // 丟棄原始的 tx

    thread::spawn(move || {
        tx1.send("來自執行緒 1".to_string()).unwrap();
    });

    thread::spawn(move || {
        tx2.send("來自執行緒 2".to_string()).unwrap();
    });

    // 接收所有訊息
    for received in rx {
        println!("收到: {}", received);
    }
}
```

### try_recv 與 recv_timeout

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("延遲訊息").unwrap();
    });

    // try_recv：非阻塞，立即回傳
    match rx.try_recv() {
        Ok(msg) => println!("收到: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("還沒有訊息"),
        Err(mpsc::TryRecvError::Disconnected) => println!("通道已關閉"),
    }

    // recv_timeout：帶超時的阻塞
    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(msg) => println!("收到: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("超時"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("通道已關閉"),
    }
}
```

### sync_channel：有界通道

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // 建立容量為 2 的有界通道
    let (tx, rx) = mpsc::sync_channel(2);

    thread::spawn(move || {
        for i in 0..5 {
            println!("發送 {}", i);
            tx.send(i).unwrap(); // 當緩衝區滿時會阻塞
            println!("已發送 {}", i);
        }
    });

    thread::sleep(std::time::Duration::from_secs(1));

    for received in rx {
        println!("收到: {}", received);
    }
}
```

---

## 16.3 共享狀態

有時候使用共享記憶體比訊息傳遞更合適。Rust 提供了 `Mutex` 和 `RwLock` 來安全地共享資料。

### Mutex\<T\>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // lock() 取得鎖，回傳 MutexGuard
        let mut num = m.lock().unwrap();
        *num = 6;
        // MutexGuard 離開作用域時自動解鎖
    }

    println!("m = {:?}", m);
}
```

### 多執行緒共享 Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 使用 Arc 讓多個執行緒共享 Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("結果: {}", *counter.lock().unwrap());
}
```

### try_lock 與死鎖預防

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(0));

    // try_lock：非阻塞嘗試取得鎖
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        for _ in 0..10 {
            match data_clone.try_lock() {
                Ok(mut guard) => {
                    *guard += 1;
                    println!("成功取得鎖，值: {}", *guard);
                }
                Err(_) => {
                    println!("無法取得鎖，稍後重試");
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // 主執行緒也在使用同一個 Mutex
    for _ in 0..10 {
        let mut guard = data.lock().unwrap();
        *guard += 1;
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap();
    println!("最終結果: {}", *data.lock().unwrap());
}
```

### 死鎖範例與預防

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let resource_a = Arc::new(Mutex::new(0));
    let resource_b = Arc::new(Mutex::new(0));

    // 死鎖範例（不要這樣做！）
    // 執行緒 1：先鎖 A 再鎖 B
    // 執行緒 2：先鎖 B 再鎖 A
    // 可能導致死鎖

    // 預防死鎖：所有執行緒以相同順序取得鎖
    let a1 = Arc::clone(&resource_a);
    let b1 = Arc::clone(&resource_b);
    let handle1 = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        let _b = b1.lock().unwrap();
        println!("執行緒 1 完成");
    });

    let a2 = Arc::clone(&resource_a);
    let b2 = Arc::clone(&resource_b);
    let handle2 = thread::spawn(move || {
        let _a = a2.lock().unwrap(); // 相同順序：先 A 再 B
        let _b = b2.lock().unwrap();
        println!("執行緒 2 完成");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### RwLock\<T\>

讀寫鎖允許多個讀取者或單個寫入者：

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // 多個讀取者可以同時讀取
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let read_guard = data.read().unwrap();
            println!("讀取者 {}: {:?}", i, *read_guard);
        }));
    }

    // 寫入者需要獨佔訪問
    let data_writer = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        let mut write_guard = data_writer.write().unwrap();
        write_guard.push(4);
        println!("寫入者: {:?}", *write_guard);
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最終結果: {:?}", *data.read().unwrap());
}
```

---

## 16.4 原子操作

原子類型提供了不需要鎖的執行緒安全操作。

### 基本原子類型

```rust
use std::sync::atomic::{AtomicI32, AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // AtomicI32
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("計數器: {}", counter.load(Ordering::SeqCst)); // 10000

    // AtomicBool
    let flag = AtomicBool::new(false);
    flag.store(true, Ordering::SeqCst);
    println!("flag: {}", flag.load(Ordering::SeqCst));

    // compare_exchange（CAS 操作）
    let value = AtomicI32::new(5);
    let result = value.compare_exchange(5, 10, Ordering::SeqCst, Ordering::SeqCst);
    println!("CAS 結果: {:?}, 新值: {}", result, value.load(Ordering::SeqCst));
}
```

### Ordering（記憶體順序）

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let counter = AtomicUsize::new(0);

    // Relaxed：最弱的保證，只保證原子性
    counter.fetch_add(1, Ordering::Relaxed);

    // Release/Acquire：用於同步
    // Release：之前的寫入對其他執行緒可見
    // Acquire：之後的讀取能看到其他執行緒的寫入
    counter.store(10, Ordering::Release);
    let value = counter.load(Ordering::Acquire);

    // SeqCst：最強的保證，全局順序一致
    counter.fetch_add(1, Ordering::SeqCst);

    println!("最終值: {}", counter.load(Ordering::SeqCst));
}
```

### 原子類型的應用

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // 使用 AtomicBool 作為停止信號
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let worker = thread::spawn(move || {
        let mut count = 0;
        while running_clone.load(Ordering::Relaxed) {
            count += 1;
            thread::sleep(Duration::from_millis(10));
        }
        println!("工作執行緒結束，計數: {}", count);
    });

    // 讓工作執行緒執行一段時間
    thread::sleep(Duration::from_millis(100));

    // 發送停止信號
    running.store(false, Ordering::Relaxed);

    worker.join().unwrap();
    println!("程式結束");
}
```

### 自旋鎖

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        // 自旋直到成功取得鎖
        while self.locked.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ).is_err() {
            // 自旋等待
            std::hint::spin_loop();
        }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let lock = Arc::new(SpinLock::new());
    let counter = Arc::new(std::sync::atomic::AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let lock = Arc::clone(&lock);
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                lock.lock();
                counter.fetch_add(1, Ordering::SeqCst);
                lock.unlock();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("計數器: {}", counter.load(Ordering::SeqCst));
}
```

---

## 16.5 Send 與 Sync

`Send` 和 `Sync` 是兩個標記 trait，用於表示型別的執行緒安全性。

### Send Trait

`Send` 表示型別的所有權可以在執行緒間轉移：

```rust
use std::thread;

fn main() {
    // 大多數型別都是 Send
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", data);
    });
    handle.join().unwrap();

    // Rc 不是 Send（因為引用計數不是原子的）
    // use std::rc::Rc;
    // let rc = Rc::new(5);
    // thread::spawn(move || {
    //     println!("{}", rc); // 錯誤！Rc<i32> 不是 Send
    // });

    // Arc 是 Send（因為引用計數是原子的）
    use std::sync::Arc;
    let arc = Arc::new(5);
    let arc_clone = Arc::clone(&arc);
    thread::spawn(move || {
        println!("{}", arc_clone);
    }).join().unwrap();
}
```

### Sync Trait

`Sync` 表示型別可以安全地被多個執行緒引用：

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    // 如果 &T 是 Send，則 T 是 Sync

    // 不可變引用可以在執行緒間共享
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            println!("執行緒 {}: {:?}", i, data);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // RefCell 不是 Sync（因為借用檢查不是執行緒安全的）
    // 使用 Mutex 或 RwLock 代替
}
```

### 自動實作規則

```rust
// 如果所有成員都是 Send，則結構體自動是 Send
struct MyStruct {
    a: i32,     // Send
    b: String,  // Send
}
// MyStruct 是 Send

// 如果包含非 Send 成員，則結構體不是 Send
// struct NotSend {
//     rc: std::rc::Rc<i32>,  // 不是 Send
// }
// NotSend 不是 Send

// 手動實作（通常需要 unsafe）
struct MySendType {
    ptr: *mut i32,  // 裸指標不是 Send
}

// 如果我們確信這是安全的，可以手動實作
// unsafe impl Send for MySendType {}
// unsafe impl Sync for MySendType {}
```

### 實際範例

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// 這個結構體是 Send + Sync
struct SharedData {
    counter: Mutex<i32>,
    name: String,
}

fn main() {
    let data = Arc::new(SharedData {
        counter: Mutex::new(0),
        name: String::from("共享資料"),
    });

    let mut handles = vec![];

    for i in 0..5 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut counter = data.counter.lock().unwrap();
            *counter += 1;
            println!("執行緒 {}: {} 計數器 = {}", i, data.name, *counter);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最終計數: {}", *data.counter.lock().unwrap());
}
```

---

## 並行模式

### 執行緒池（使用 rayon）

```rust
// Cargo.toml: rayon = "1.8"

use rayon::prelude::*;

fn main() {
    let numbers: Vec<i64> = (1..=1_000_000).collect();

    // 並行迭代
    let sum: i64 = numbers.par_iter().sum();
    println!("總和: {}", sum);

    // 並行 map
    let squares: Vec<i64> = numbers.par_iter()
        .map(|&x| x * x)
        .collect();
    println!("平方數量: {}", squares.len());

    // 並行 filter
    let evens: Vec<_> = numbers.par_iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("偶數數量: {}", evens.len());
}
```

### Barrier（屏障）

```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("執行緒 {} 開始工作", i);
            thread::sleep(std::time::Duration::from_millis(100 * (i as u64 + 1)));
            println!("執行緒 {} 到達屏障", i);

            barrier.wait(); // 等待所有執行緒到達

            println!("執行緒 {} 繼續執行", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Condvar（條件變數）

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // 生產者
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(1));
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one(); // 通知等待的執行緒
        println!("生產者：資料已準備好");
    });

    // 消費者
    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        println!("消費者：等待資料...");
        ready = cvar.wait(ready).unwrap();
    }
    println!("消費者：收到資料");
}
```

---

## 練習題

### 練習 1：並行計數器

使用多個執行緒安全地增加計數器：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn parallel_counter(num_threads: usize, increments_per_thread: usize) -> usize {
    // 實作：使用 Arc<Mutex<usize>> 或 AtomicUsize
    todo!()
}
```

### 練習 2：生產者-消費者

實作一個生產者-消費者模式：

```rust
use std::sync::mpsc;
use std::thread;

fn producer_consumer() {
    // 建立多個生產者和一個消費者
    // 生產者產生 1-100 的數字
    // 消費者計算總和
    todo!()
}
```

### 練習 3：並行快速排序

使用多執行緒實作快速排序：

```rust
fn parallel_quicksort<T: Ord + Send>(arr: &mut [T]) {
    // 當子陣列足夠大時，使用新執行緒處理
    todo!()
}
```

---

## 本章小結

- **執行緒基礎**：使用 `thread::spawn` 建立執行緒，`JoinHandle` 等待結束
- **訊息傳遞**：使用 `mpsc::channel` 在執行緒間傳遞資料
- **共享狀態**：使用 `Mutex` 和 `RwLock` 安全地共享資料
- **原子操作**：使用原子類型進行無鎖同步
- **Send 與 Sync**：標記型別的執行緒安全性

---

## 延伸閱讀

- [The Rust Book - Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust By Example - Threads](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html)
- [std::sync Documentation](https://doc.rust-lang.org/std/sync/)
- [Rayon - Data parallelism library](https://docs.rs/rayon/)
