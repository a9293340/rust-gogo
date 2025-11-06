pub fn main() {
    // ❌ 預設不可變
    let x = 5;
    // x = 6; // 編譯錯誤！

    // ✅ 使用 mut 讓變數可變
    print!("✅ 使用 mut 讓變數可變 \n");
    let mut y = 5;
    println!("y-1: {}", y);
    y = 6; // 正確
    println!("y-2: {}", y);

    // 常數
    print!("✅ 常數 \n");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    // shadowing(遮蔽特性)
    print!("✅ shadowing(遮蔽特性) \n");
    let x = 5;
    println!("x: {}", x);
    let x = x + 1; // 遮蔽，創建新變數
    println!("x-2: {}", x);
    let x = x * 2; // 再次遮蔽
    println!("x-3: {}", x);

    // 優勢：可以改變型別
    let spaces = "   ";
    let spaces = spaces.len(); // ✅ 從 &str 變成 usize

    println!("spaces: {}", spaces);

    // 數值字面值
    print!("✅ 數值字面值 \n");
    let decimal = 98_222; // 十進位（可用 _ 分隔）
    let hex = 0xff; // 十六進位
    let octal = 0o77; // 八進位
    let binary = 0b1111_0000; // 二進位
    let byte = b'A'; // 位元組（僅 u8）

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);
    println!("byte: {}", byte);

    // Tuple (元組)
    print!("✅ Tuple (元組) \n");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解構
    let (x, y, z) = tup;

    // 索引存取
    let first = tup.0;
    let second = tup.1;

    // 空 tuple（unit）
    let unit = ();

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("first: {}", first);
    println!("second: {}", second);
    println!("unit: {:?}", unit);

    // 陣列(Array)
    print!("✅ 陣列(Array) \n");
    let months = ["January", "February"];

    // 指定型別和長度
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    // 初始化相同值
    let b = [3; 5]; // [3, 3, 3, 3, 3]

    // 存取
    let first = a[0];
    println!("first: {}", first);
    println!("months: {}", months[0]);
    println!("b: {:?}", b);

    // 函式 (Functions)
    print!("✅ 函式 (Functions) \n");
    fn print_sum(x: i32, y: i32) -> i32 {
        println!("{} + {} = {}", x, y, x + y);
        x + y + 1 // 隱式返回 不能有分號
    }

    print_sum(1, 2);
    println!("print_sum(1, 2): {}", print_sum(1, 2));

    let y = {
        let x = 3;
        x + 1 // 表達式，無分號
    };
    println!("y: {}", y);

    // 控制流 (Control Flow)
    print!("✅ 控制流 (Control Flow) \n");
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 表達式(表達式會回傳值)
    let number = if number < 5 {
        "less than 5"
    } else {
        "greater than or equal to 5"
    };
    println!("number: {}", number);

    // loop 無限迴圈
    print!("✅ loop 無限迴圈 \n");
    let mut count = 0;
    loop {
        count += 1;
        println!("count: {}", count);
        if count == 5 {
            break;
        }
    }

    // 多層迴圈和 break 標籤
    print!("✅ 多層迴圈和 break 標籤 \n");
    let mut count = 0;
    'outer: loop {
        println!("outer loop: {}", count);
        count += 1;
        let mut count2 = 0;
        'inner: loop {
            println!("inner loop: {}", count2);
            count2 += 1;
            if count2 == 3 {
                println!("break inner loop");
                break 'inner;
            }
        }
        if count == 5 {
            println!("break outer loop");
            break 'outer;
        }
    }
    println!("count: {}", count);

    // while 迴圈
    print!("✅ while 迴圈 \n");
    let mut number = 3;
    while number != 0 {
        println!("number: {}", number);
        number -= 1;
    }
    println!("number: {}", number);

    // for 迴圈
    print!("✅ for 迴圈 \n");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("element: {}", element);
    }
}
