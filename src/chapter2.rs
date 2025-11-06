use rand::Rng;
use std::io;

pub fn main() {
    //! Crate 語言示範
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("無法讀取輸入");

    println!("你輸入: {}", guess);

    let secret = rand::rng().random_range(1..=100);
    println!("神秘數字: {}", secret);

    // ! 字串轉換為數字，需要有 expect 來處理錯誤
    let guess: u32 = guess.trim().parse().expect("請輸入數字");
    println!("你輸入: {}", guess);
}
