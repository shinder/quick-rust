# 第 30 章：設計模式

> Rust 的設計模式結合了傳統 OOP 模式和函數式程式設計特有的模式

---

## 30.1 創建型模式

### 建構器模式（Builder）

```rust
#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    max_connections: u32,
    timeout: Option<u64>,
}

#[derive(Default)]
struct ServerBuilder {
    host: String,
    port: u16,
    max_connections: u32,
    timeout: Option<u64>,
}

impl ServerBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    fn build(self) -> Result<Server, &'static str> {
        if self.host.is_empty() {
            return Err("Host is required");
        }
        if self.port == 0 {
            return Err("Port is required");
        }

        Ok(Server {
            host: self.host,
            port: self.port,
            max_connections: if self.max_connections == 0 { 100 } else { self.max_connections },
            timeout: self.timeout,
        })
    }
}

fn main() {
    let server = ServerBuilder::new()
        .host("localhost")
        .port(8080)
        .max_connections(50)
        .timeout(30)
        .build()
        .unwrap();

    println!("{:?}", server);
}
```

### 工廠模式

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

enum AnimalType {
    Dog,
    Cat,
}

fn create_animal(animal_type: AnimalType) -> Box<dyn Animal> {
    match animal_type {
        AnimalType::Dog => Box::new(Dog),
        AnimalType::Cat => Box::new(Cat),
    }
}

fn main() {
    let dog = create_animal(AnimalType::Dog);
    let cat = create_animal(AnimalType::Cat);

    dog.speak();
    cat.speak();
}
```

---

## 30.2 結構型模式

### 適配器模式

```rust
// 舊介面
trait OldPrinter {
    fn print_old(&self, text: &str);
}

// 新介面
trait NewPrinter {
    fn print(&self, text: &str);
}

struct LegacyPrinter;

impl OldPrinter for LegacyPrinter {
    fn print_old(&self, text: &str) {
        println!("[LEGACY] {}", text);
    }
}

// 適配器
struct PrinterAdapter {
    legacy: LegacyPrinter,
}

impl NewPrinter for PrinterAdapter {
    fn print(&self, text: &str) {
        self.legacy.print_old(text);
    }
}

fn main() {
    let adapter = PrinterAdapter { legacy: LegacyPrinter };
    adapter.print("Hello, World!");
}
```

### 裝飾器模式

```rust
trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 { 2.0 }
    fn description(&self) -> String { "Simple coffee".to_string() }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 { self.coffee.cost() + 0.5 }
    fn description(&self) -> String {
        format!("{} + milk", self.coffee.description())
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f64 { self.coffee.cost() + 0.2 }
    fn description(&self) -> String {
        format!("{} + sugar", self.coffee.description())
    }
}

fn main() {
    let coffee = SimpleCoffee;
    let with_milk = MilkDecorator { coffee: Box::new(coffee) };
    let with_sugar = SugarDecorator { coffee: Box::new(with_milk) };

    println!("{}: ${}", with_sugar.description(), with_sugar.cost());
}
```

---

## 30.3 行為型模式

### 策略模式

```rust
trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

struct CreditCard;
struct PayPal;
struct Bitcoin;

impl PaymentStrategy for CreditCard {
    fn pay(&self, amount: f64) {
        println!("Paid ${} via Credit Card", amount);
    }
}

impl PaymentStrategy for PayPal {
    fn pay(&self, amount: f64) {
        println!("Paid ${} via PayPal", amount);
    }
}

impl PaymentStrategy for Bitcoin {
    fn pay(&self, amount: f64) {
        println!("Paid ${} via Bitcoin", amount);
    }
}

struct ShoppingCart {
    items: Vec<f64>,
    strategy: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        Self { items: Vec::new(), strategy }
    }

    fn add(&mut self, price: f64) {
        self.items.push(price);
    }

    fn checkout(&self) {
        let total: f64 = self.items.iter().sum();
        self.strategy.pay(total);
    }
}

fn main() {
    let mut cart = ShoppingCart::new(Box::new(CreditCard));
    cart.add(100.0);
    cart.add(50.0);
    cart.checkout();
}
```

### 觀察者模式

```rust
trait Observer {
    fn update(&self, message: &str);
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    fn new() -> Self {
        Self { observers: Vec::new() }
    }

    fn subscribe(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

struct EmailSubscriber { email: String }
struct SMSSubscriber { phone: String }

impl Observer for EmailSubscriber {
    fn update(&self, message: &str) {
        println!("Email to {}: {}", self.email, message);
    }
}

impl Observer for SMSSubscriber {
    fn update(&self, message: &str) {
        println!("SMS to {}: {}", self.phone, message);
    }
}

fn main() {
    let mut subject = Subject::new();
    subject.subscribe(Box::new(EmailSubscriber { email: "user@example.com".into() }));
    subject.subscribe(Box::new(SMSSubscriber { phone: "123-456-7890".into() }));

    subject.notify("New update available!");
}
```

---

## 30.4 Rust 特有模式

### Newtype 模式

```rust
// 為型別增加語義
struct Kilometers(f64);
struct Miles(f64);

impl Kilometers {
    fn to_miles(&self) -> Miles {
        Miles(self.0 * 0.621371)
    }
}

impl Miles {
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 * 1.60934)
    }
}

// 防止混淆不同單位
fn travel(distance: Kilometers) {
    println!("Traveling {} km", distance.0);
}

fn main() {
    let km = Kilometers(100.0);
    travel(km);
    // travel(Miles(62.0)); // 編譯錯誤！
}
```

### Type State 模式

```rust
// 使用型別系統編碼狀態
struct Draft;
struct Published;

struct Article<State> {
    content: String,
    _state: std::marker::PhantomData<State>,
}

impl Article<Draft> {
    fn new(content: String) -> Self {
        Article { content, _state: std::marker::PhantomData }
    }

    fn edit(&mut self, new_content: String) {
        self.content = new_content;
    }

    fn publish(self) -> Article<Published> {
        Article { content: self.content, _state: std::marker::PhantomData }
    }
}

impl Article<Published> {
    fn view(&self) -> &str {
        &self.content
    }

    // 已發布的文章不能編輯
    // fn edit() 不存在於這個狀態
}

fn main() {
    let mut draft = Article::<Draft>::new("Hello".into());
    draft.edit("Hello, World!".into());

    let published = draft.publish();
    println!("{}", published.view());

    // published.edit("..."); // 編譯錯誤！
}
```

### RAII 模式

```rust
use std::fs::File;
use std::io::Write;

// 資源獲取即初始化
struct TempFile {
    path: String,
    file: File,
}

impl TempFile {
    fn new(path: &str) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(TempFile { path: path.to_string(), file })
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.file.write_all(data)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // 自動清理
        let _ = std::fs::remove_file(&self.path);
        println!("Cleaned up {}", self.path);
    }
}

fn main() -> std::io::Result<()> {
    {
        let mut temp = TempFile::new("temp.txt")?;
        temp.write(b"Hello, World!")?;
    } // 自動刪除檔案

    Ok(())
}
```

---

## 練習題

1. 實作一個使用建構器模式的 HTTP 請求建構器
2. 使用 Type State 模式設計一個訂單處理系統
3. 實作一個事件系統（觀察者模式）

---

## 本章小結

- **建構器模式**：靈活的物件建構
- **策略模式**：使用 trait 實作可替換的演算法
- **Newtype 模式**：增加型別安全性
- **Type State 模式**：在編譯時期驗證狀態轉換
- **RAII 模式**：自動資源管理

---

## 延伸閱讀

- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Idiomatic Rust](https://github.com/mre/idiomatic-rust)
