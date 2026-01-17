# 第 22 章：命令列工具

> Rust 是建立命令列工具的絕佳選擇，提供優秀的效能和跨平台支援

---

## 22.1 參數解析

### std::env::args

```rust
use std::env;

fn main() {
    // 取得所有命令列參數
    let args: Vec<String> = env::args().collect();

    println!("程式名稱: {}", args[0]);

    if args.len() > 1 {
        println!("第一個參數: {}", args[1]);
    }

    // 迭代所有參數
    for (i, arg) in args.iter().enumerate() {
        println!("args[{}]: {}", i, arg);
    }
}
```

### clap crate

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

**使用 derive 巨集**：

```rust
use clap::Parser;

/// 一個簡單的 CLI 工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 輸入檔案
    #[arg(short, long)]
    input: String,

    /// 輸出檔案
    #[arg(short, long)]
    output: Option<String>,

    /// 詳細模式
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// 重複次數
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

fn main() {
    let args = Args::parse();

    println!("輸入: {}", args.input);
    if let Some(output) = args.output {
        println!("輸出: {}", output);
    }
    if args.verbose {
        println!("詳細模式已啟用");
    }
    println!("重複 {} 次", args.count);
}
```

使用：
```bash
$ mycli --input file.txt --output out.txt -v --count 3
$ mycli -i file.txt -o out.txt -c 3
$ mycli --help
```

**子命令**：

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 新增項目
    Add {
        /// 項目名稱
        name: String,

        /// 標籤
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// 列出所有項目
    List {
        /// 只顯示未完成的
        #[arg(short, long)]
        pending: bool,
    },

    /// 刪除項目
    Remove {
        /// 項目 ID
        id: u32,

        /// 強制刪除
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, tags } => {
            println!("新增: {}", name);
            if !tags.is_empty() {
                println!("標籤: {:?}", tags);
            }
        }
        Commands::List { pending } => {
            if pending {
                println!("列出未完成項目");
            } else {
                println!("列出所有項目");
            }
        }
        Commands::Remove { id, force } => {
            if force {
                println!("強制刪除項目 {}", id);
            } else {
                println!("刪除項目 {}", id);
            }
        }
    }
}
```

**進階選項**：

```rust
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Parser, Debug)]
struct Args {
    /// 輸入檔案
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// 輸出格式
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,

    /// 環境變數
    #[arg(long, env = "MY_CONFIG")]
    config: Option<String>,

    /// 可以重複使用的選項
    #[arg(short = 'D', long = "define", value_parser = parse_key_val)]
    defines: Vec<(String, String)>,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("無效的 KEY=VALUE: '{}'", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
```

---

## 22.2 終端機 I/O

### 標準輸入輸出

```rust
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // 寫入 stdout
    println!("這是標準輸出");
    print!("不換行輸出");
    io::stdout().flush()?; // 確保立即輸出

    // 寫入 stderr
    eprintln!("這是錯誤輸出");

    // 讀取一行
    print!("請輸入名字: ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();

    println!("你好, {}!", name);

    // 逐行讀取
    println!("輸入多行文字 (Ctrl+D 結束):");
    for line in io::stdin().lock().lines() {
        let line = line?;
        println!("收到: {}", line);
    }

    Ok(())
}
```

### 彩色輸出

```toml
# Cargo.toml
[dependencies]
colored = "2"
```

```rust
use colored::*;

fn main() {
    // 基本顏色
    println!("{}", "紅色文字".red());
    println!("{}", "綠色文字".green());
    println!("{}", "藍色文字".blue());
    println!("{}", "黃色文字".yellow());

    // 背景色
    println!("{}", "紅色背景".on_red());
    println!("{}", "白字藍底".white().on_blue());

    // 樣式
    println!("{}", "粗體".bold());
    println!("{}", "斜體".italic());
    println!("{}", "底線".underline());
    println!("{}", "閃爍".blink());

    // 組合
    println!("{}", "紅色粗體".red().bold());
    println!("{}", "綠色底線".green().underline());

    // 條件著色
    let success = true;
    let message = if success {
        "成功".green()
    } else {
        "失敗".red()
    };
    println!("狀態: {}", message);

    // 狀態指示
    println!("{} 正在處理...", "INFO".blue().bold());
    println!("{} 發生錯誤", "ERROR".red().bold());
    println!("{} 注意事項", "WARN".yellow().bold());
}
```

### 進度條

```toml
# Cargo.toml
[dependencies]
indicatif = "0.17"
```

```rust
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::thread;
use std::time::Duration;

fn main() {
    // 簡單進度條
    let pb = ProgressBar::new(100);
    for _ in 0..100 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(20));
    }
    pb.finish_with_message("完成");

    // 自訂樣式
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    for _ in 0..100 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(20));
    }
    pb.finish();

    // 不確定長度的進度
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    for i in 0..50 {
        pb.set_message(format!("處理中... {}", i));
        thread::sleep(Duration::from_millis(50));
    }
    pb.finish_with_message("完成");

    // 多進度條
    let m = MultiProgress::new();
    let pb1 = m.add(ProgressBar::new(100));
    let pb2 = m.add(ProgressBar::new(100));

    let h1 = thread::spawn(move || {
        for _ in 0..100 {
            pb1.inc(1);
            thread::sleep(Duration::from_millis(30));
        }
        pb1.finish_with_message("任務 1 完成");
    });

    let h2 = thread::spawn(move || {
        for _ in 0..100 {
            pb2.inc(1);
            thread::sleep(Duration::from_millis(50));
        }
        pb2.finish_with_message("任務 2 完成");
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
```

### 互動式介面

```toml
# Cargo.toml
[dependencies]
dialoguer = "0.11"
console = "0.15"
```

```rust
use dialoguer::{Confirm, Input, Select, MultiSelect, Password, theme::ColorfulTheme};
use console::Term;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stderr();

    // 文字輸入
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("你的名字")
        .default("Guest".into())
        .interact_text()?;
    println!("你好, {}!", name);

    // 確認
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("要繼續嗎?")
        .default(true)
        .interact()?
    {
        println!("繼續...");
    }

    // 選擇
    let options = vec!["選項 A", "選項 B", "選項 C"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("選擇一個選項")
        .items(&options)
        .default(0)
        .interact()?;
    println!("你選擇了: {}", options[selection]);

    // 多選
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("選擇多個選項")
        .items(&options)
        .interact()?;
    println!("你選擇了: {:?}", selections);

    // 密碼
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("輸入密碼")
        .with_confirmation("確認密碼", "密碼不匹配")
        .interact()?;
    println!("密碼長度: {}", password.len());

    // 清除畫面
    term.clear_screen()?;

    Ok(())
}
```

---

## 22.3 設定檔處理

### 環境變數

```rust
use std::env;

fn main() {
    // 讀取環境變數
    match env::var("HOME") {
        Ok(home) => println!("HOME: {}", home),
        Err(_) => println!("HOME 未設定"),
    }

    // 使用預設值
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    println!("PORT: {}", port);

    // 設定環境變數
    env::set_var("MY_VAR", "my_value");

    // 列出所有環境變數
    for (key, value) in env::vars() {
        if key.starts_with("RUST") {
            println!("{}: {}", key, value);
        }
    }
}
```

### dotenv

```toml
# Cargo.toml
[dependencies]
dotenvy = "0.15"
```

```rust
use std::env;

fn main() {
    // 載入 .env 檔案
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let api_key = env::var("API_KEY")
        .unwrap_or_else(|_| "default_key".to_string());

    println!("Database: {}", database_url);
    println!("API Key: {}", api_key);
}
```

**.env 檔案**：
```
DATABASE_URL=postgres://localhost/mydb
API_KEY=secret_key_123
DEBUG=true
```

### TOML 設定

```toml
# Cargo.toml
[dependencies]
toml = "0.8"
serde = { version = "1", features = ["derive"] }
```

```rust
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
    #[serde(default)]
    logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

#[derive(Debug, Deserialize, Default)]
struct LoggingConfig {
    #[serde(default = "default_level")]
    level: String,
    #[serde(default)]
    file: Option<String>,
}

fn default_level() -> String {
    "info".to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;

    println!("伺服器: {}:{}", config.server.host, config.server.port);
    println!("資料庫: {}", config.database.url);
    println!("日誌等級: {}", config.logging.level);

    Ok(())
}
```

**config.toml**：
```toml
[server]
host = "localhost"
port = 8080

[database]
url = "postgres://localhost/mydb"
max_connections = 10

[logging]
level = "debug"
file = "app.log"
```

### 設定檔路徑

```rust
use std::path::PathBuf;
use dirs;

fn get_config_path() -> PathBuf {
    // 使用 XDG 規範
    if let Some(config_dir) = dirs::config_dir() {
        return config_dir.join("myapp").join("config.toml");
    }

    // 備用：當前目錄
    PathBuf::from("config.toml")
}

fn get_data_path() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        return data_dir.join("myapp");
    }
    PathBuf::from("data")
}

fn main() {
    println!("設定檔: {:?}", get_config_path());
    println!("資料目錄: {:?}", get_data_path());

    // 其他常用路徑
    println!("Home: {:?}", dirs::home_dir());
    println!("Cache: {:?}", dirs::cache_dir());
    println!("Config: {:?}", dirs::config_dir());
    println!("Data: {:?}", dirs::data_dir());
}
```

---

## 完整 CLI 範例

```rust
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "todo")]
#[command(author, version, about = "一個簡單的待辦事項 CLI")]
struct Cli {
    /// 設定檔路徑
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 新增待辦事項
    Add {
        /// 待辦事項描述
        description: String,
    },
    /// 列出所有待辦事項
    List,
    /// 標記完成
    Done {
        /// 待辦事項 ID
        id: usize,
    },
    /// 刪除待辦事項
    Remove {
        /// 待辦事項 ID
        id: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TodoList {
    items: Vec<TodoItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    id: usize,
    description: String,
    completed: bool,
}

impl TodoList {
    fn load(path: &PathBuf) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save(&self, path: &PathBuf) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }

    fn add(&mut self, description: String) -> usize {
        let id = self.items.len() + 1;
        self.items.push(TodoItem {
            id,
            description,
            completed: false,
        });
        id
    }

    fn complete(&mut self, id: usize) -> bool {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            true
        } else {
            false
        }
    }

    fn remove(&mut self, id: usize) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.id == id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let data_path = cli.config.unwrap_or_else(|| {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("todo.json")
    });

    let mut todos = TodoList::load(&data_path);

    match cli.command {
        Commands::Add { description } => {
            let id = todos.add(description.clone());
            todos.save(&data_path)?;
            println!("{} 新增待辦事項 #{}: {}", "✓".green(), id, description);
        }
        Commands::List => {
            if todos.items.is_empty() {
                println!("{}", "沒有待辦事項".yellow());
            } else {
                for item in &todos.items {
                    let status = if item.completed {
                        "✓".green()
                    } else {
                        "○".white()
                    };
                    let desc = if item.completed {
                        item.description.strikethrough().to_string()
                    } else {
                        item.description.clone()
                    };
                    println!("{} #{}: {}", status, item.id, desc);
                }
            }
        }
        Commands::Done { id } => {
            if todos.complete(id) {
                todos.save(&data_path)?;
                println!("{} 標記 #{} 為完成", "✓".green(), id);
            } else {
                println!("{} 找不到 #{}", "✗".red(), id);
            }
        }
        Commands::Remove { id } => {
            if todos.remove(id) {
                todos.save(&data_path)?;
                println!("{} 刪除 #{}", "✓".green(), id);
            } else {
                println!("{} 找不到 #{}", "✗".red(), id);
            }
        }
    }

    Ok(())
}
```

---

## 練習題

### 練習 1：檔案搜尋工具

建立一個類似 `grep` 的工具：
- 搜尋檔案中的文字
- 支援正規表達式
- 彩色輸出匹配結果

### 練習 2：系統監控工具

建立一個顯示系統資訊的工具：
- CPU 使用率
- 記憶體使用量
- 磁碟空間

### 練習 3：JSON 處理工具

建立一個處理 JSON 的工具：
- 格式化 JSON
- 查詢特定欄位
- 轉換格式

---

## 本章小結

- **參數解析**：使用 clap 處理命令列參數
- **終端機 I/O**：彩色輸出、進度條、互動式介面
- **設定檔**：環境變數、TOML、JSON 設定

---

## 延伸閱讀

- [Clap Documentation](https://docs.rs/clap/)
- [Command Line Applications in Rust](https://rust-cli.github.io/book/)
- [Indicatif Documentation](https://docs.rs/indicatif/)
