#[derive(Debug)]
enum Status {
    Pending,
    Success,
    Error,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // 攜帶四個 u8
    V6(String),         // 攜帶一個 String
}

#[derive(Debug)]
enum Message {
    Quit,                       // 無資料
    Move { x: i32, y: i32 },    // 具名欄位（像 struct）
    Write(String),              // 單一字串
    ChangeColor(i32, i32, i32), // 三個整數
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出訊息"),
            Message::Move { x, y } => println!("移動到 ({}, {})", x, y),
            Message::Write(text) => println!("寫入: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("變更顏色: RGB({}, {}, {})", r, g, b)
            }
        }
    }
}

pub fn main() {
    // Enum
    print!("✅Enum \n");
    let status = Status::Pending;
    println!("status: {:?}", status);

    let ip1 = IpAddr::V4(127, 0, 0, 1);
    let ip2 = IpAddr::V6(String::from("::1"));
    println!(
        "ip1: {:?}",
        match ip1 {
            IpAddr::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(ip) => ip.to_string(),
        }
    );
    println!(
        "ip2: {:?}",
        match ip2 {
            IpAddr::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(ip) => ip.to_string(),
        }
    );

    // enum function(enum 可以有方法)
    print!("✅enum function(enum 可以有方法) \n");
    let message = Message::Write(String::from("Hello, world!"));
    message.call();
    let message = Message::Move { x: 10, y: 20 };
    message.call();
    let message = Message::ChangeColor(255, 255, 255);
    message.call();
    let message = Message::Quit;
    message.call();

    // Option Enum
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    print!("✅Option Enum \n");
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);

    // Option Enum 方法
    print!("✅Option Enum 方法 \n");
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // unwrap_or：提供預設值
    println!("{}", some_number.unwrap_or(0)); // 5
    println!("{}", no_number.unwrap_or(0)); // 0

    // map：轉換 Some 中的值
    let doubled = some_number.map(|x| x * 2); // Some(10)
    println!("doubled: {:?}", doubled);

    // and_then：鏈式操作
    let result = some_number.and_then(|x| if x > 3 { Some(x * 2) } else { None });
    println!("result: {:?}", result);
    let result = no_number.and_then(|x| if x > 3 { Some(x * 2) } else { None });
    println!("result: {:?}", result);

    // Match
    print!("✅Match \n");
    let coin = Coin::Quarter(UsState::Alabama);
    println!("value_in_cents: {}", value_in_cents(coin));
    let coin = Coin::Penny;
    println!("value_in_cents: {}", value_in_cents(coin));
    let coin = Coin::Nickel;
    println!("value_in_cents: {}", value_in_cents(coin));
    let coin = Coin::Dime;
    println!("value_in_cents: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);

    println!("xxx1: {}", xxx(1));
    println!("xxx2: {}", xxx(2));
    println!("xxx3: {}", xxx(3));
    println!("xxx4: {}", xxx(4));

    // if let 簡化
    print!("✅if let 簡化 \n");
    // 使用 match（囉嗦）
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("最大值(囉嗦版): {}", max),
        _ => (), // 不關心 None
    }

    // 使用 if let（簡潔）
    if let Some(max) = config_max {
        println!("最大值(簡潔版): {}", max);
    }

    // if let else
    print!("✅if let else \n");
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);

    println!("coin: {:?}", coin);

    // match 版本
    match coin {
        Coin::Quarter(state) => println!("來自 {:?}！", state),
        _ => count += 1,
    }
    println!("count: {}", count);
    // println!("coin after match: {:?}", coin); // ❌ 錯誤，coin 已經被移動，不能再次使用

    // if let 版本（等價）
    // if let Coin::Quarter(state) = coin {
    //     println!("來自 {:?}！", state);
    // } else {
    //     count += 1;
    // }
    // println!("count: {}", count);

    // 範圍模式
    print!("✅範圍模式 \n");
    classify('a');
    classify('A');
    classify('0');
    classify('9');
    classify('@');

    // 解構結構體
    print!("✅解構結構體 \n");
    let origin = Point { x: 0, y: 0 };
    let bottom_right = Point { x: 5, y: -5 };
    let Point { x, y } = origin;
    println!("x: {}", x);
    println!("y: {}", y);
    let Point { x: x1, y: y1 } = bottom_right;
    println!("x1: {}", x1);
    println!("y1: {}", y1);

    // 使用 @ 綁定
    print!("✅使用 @ 綁定 \n");
    let msg = Message2::Hello { id: 5 };

    match msg {
        // id_variable 綁定了符合 3..=7 範圍的值
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("ID 在範圍內: {}", id_variable);
        }
        // 只測試範圍，但無法使用值
        Message2::Hello { id: 10..=12 } => {
            println!("ID 在另一個範圍");
        }
        Message2::Hello { id } => {
            println!("其他 ID: {}", id);
        }
    }
}

// Match
// Copy 和 Clone 是為了讓 UsState 可以被複製
// #[derive(Debug, Copy, Clone)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 攜帶州的資訊
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("來自 {:?} 州的 Quarter！", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// _ 通配符
fn xxx(x: i32) -> i32 {
    match x {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 0,
    }
}

// 範圍模式
fn classify(c: char) {
    match c {
        'a'..='z' => println!("小寫字母"),
        'A'..='Z' => println!("大寫字母"),
        '0'..='9' => println!("數字"),
        _ => println!("其他"),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Message2 {
    Hello { id: i32 },
}
