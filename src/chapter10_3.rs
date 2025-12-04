use std::fmt::Display;

/*
❌ 編譯錯誤
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {  // 錯誤: 不是所有 T 都能比較!
            largest = item;
        }
    }

    largest
}
*/

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            // ✅ 現在可以了!
            largest = item;
        }
    }

    largest
}

// fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
//     let mut largest = list[0].clone(); // 需要 Clone

//     for item in list {
//         if item > &largest {
//             largest = item.clone();
//         }
//     }

//     largest
// }

/* 常用的標準庫 Traits
PartialOrd: 部分可排序
pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
}

Ord: 完全可排序
pub trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;
}

Clone: 顯式複製
pub trait Clone {
    fn clone(&self) -> Self;
}

Copy: 隱式複製 (必須先實作 Clone)
pub trait Copy: Clone {}

Debug: 用於開發除錯
pub trait Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

Display: 用於使用者友善的輸出
pub trait Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
*/

/*
Blanket Implementations
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        實作細節...
    }
}
*/

// Condition Constraints
struct Pair<T> {
    x: T,
    y: T,
}

// 為所有 Pair<T> 實作 new
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只為 T: Display + PartialOrd 實作 cmp_display
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn main() {
    println!("✅ chapter10_3");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    print!("✅ PartialOrd and Ord \n");
    let x = 1.0;
    let y = 2.0;
    let nan = f64::NAN;

    println!("{}", x < y); // true
    println!("{}", nan < y); // false
    println!("{}", nan == nan); // false - 這就是為什麼用 PartialOrd!

    print!("✅ Blanket Implementations \n");
    let s = "Hello".to_string();
    println!("{}", s);

    print!("✅ Condition Constraints \n");

    let pair_int = Pair::new(3, 5);
    pair_int.cmp_display(); // ✅ i32 實作了 Display + PartialOrd

    let pair_string = Pair::new(String::from("hello"), String::from("world"));
    pair_string.cmp_display(); // ✅ String 也實作了這些 traits

    // 假設有個型別沒實作 Display
    struct NoDisplay;
    let pair_no = Pair::new(NoDisplay, NoDisplay);
    // pair_no.cmp_display(); // ❌ 編譯錯誤: NoDisplay 沒有 Display trait
}
