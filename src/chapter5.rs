// 需要 derive Debug trait
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username, // 變數名與欄位名相同可簡寫
        email,
        age: 25,
        active: true,
    }
}

// Method Syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated Functions(關聯函式)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    //1. 不可變借用-只能讀取
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 2. 可變借用 - 需要修改
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 3. 取得所有權 - 消耗 self（少用）
    fn into_square(self) -> Rectangle {
        let size = self.width.max(self.height);
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    // Structs
    print!("✅Structs \n");
    let user1 = User {
        username: String::from("John"),
        email: String::from("john@example.com"),
        age: 20,
        active: true,
    };
    println!("user1: {:?}", user1);
    println!("user1 username: {}", user1.username);
    println!("user1 email: {}", user1.email);
    println!("user1 age: {}", user1.age);
    println!("user1 active: {}", user1.active);

    // user1.age = 26; // ❌ 錯誤

    let mut user2 = User {
        username: String::from("Jane"),
        email: String::from("jane@example.com"),
        age: 25,
        active: false,
    };
    print!("user2 age: {}", user2.age);
    user2.age = 26;
    print!("user2 age after: {}", user2.age);

    let user3 = build_user(String::from("john@example.com"), String::from("John"));
    println!("user3: {:?}", user3);

    let user4 = User {
        email: String::from("newemail@example.com"),
        ..user1
    };
    println!("user4: {:?}", user4);
    // println!("{}", user1.username); // ❌ 錯誤，user1 已經被borrow

    // Method Syntax
    print!("✅Method Syntax \n");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1: {:?}", rect1);
    println!("rect1 area: {}", rect1.area());
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect2: {:?}", rect2);
    println!("rect2 area: {}", rect2.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect before scale: {:?}", rect);
    rect.scale(2);
    println!("rect after scale: {:?}", rect);
    let square = rect.into_square(); // 消耗 rect
    println!("square after into_square: {:?}", square);
    // println!("rect after into_square: {:?}", rect); // ❌ 錯誤，rect 已經被消耗

    // Associated Functions(關聯函式)
    print!("✅Associated Functions(關聯函式) \n");
    let rect3 = Rectangle::new(30, 50);
    println!("rect3: {:?}", rect3);
    println!("rect3 area: {}", rect3.area());
}
