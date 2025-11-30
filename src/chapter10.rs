// 基本泛型函式
fn identity<T>(arg: T) -> T {
    arg
}

/* 泛型函式，但無法比較 T 的值
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {  // ❌
            largest = item;
        }
    }

    largest
}
*/

// 多個泛型參數
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn change(&mut self, x: T, y: U) {
        self.x = x;
        self.y = y;
    }
}

// !Rust 使用單態化(Monomorphization)來優化泛型函式
/*
fn add<T>(a: T, b: T) -> T { ... }

使用時，編譯器會生成對應的函式
add(1, 2);      // 編譯器生成 add_i32
add(1.0, 2.0);  // 編譯器生成 add_f64
*/

pub fn main() {
    println!("✅ identity");
    let output = identity(42);
    println!("output: {}", output);
    let output = identity("Hello, world!");
    println!("output: {}", output);

    println!("✅ Point");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer: {} {}", integer.x, integer.y);
    println!("float: {} {}", float.x, float.y);

    println!("✅ change");
    let mut integer = Point { x: 5, y: 10 };
    integer.change(100, 200);
    println!("integer: {} {}", integer.x, integer.y);
}
