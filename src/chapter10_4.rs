pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {}x{}", self.width, self.height);
    }
}

pub struct TextField {
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing text field: {}", self.placeholder);
    }
}

/* 錯誤版本
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T: Draw> Screen<T> {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*/

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // ✅ 關鍵: dyn Draw
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/* 三種 trait object 寫法:
Box<dyn Trait>      // 堆積上的 trait object   擁有所有權/需要擁有值
&dyn Trait          // 借用的 trait object   不擁有所有權/唯獨訪問
&mut dyn Trait      // 可變借用的 trait object   不擁有所有權/可變訪問
*/

pub fn main() {
    // ❌ 問題: Vec<T> 中的 T 在編譯時必須是單一具體型別
    // let screen = Screen {
    //     components: vec![
    //         Button {
    //             width: 50,
    //             height: 10,
    //         },
    //         TextField {
    //             placeholder: String::from("Enter text"),
    //         }, // 錯誤!
    //     ],
    // };

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
            }),
            Box::new(TextField {
                placeholder: String::from("Enter text"),
            }),
        ],
    };

    screen.run();
}

/* 使用泛型 - 編譯時確定型別
fn draw_static<T: Draw>(item: &T) {
    item.draw();
}

fn main() {
    let button = Button { width: 10, height: 5 };
    draw_static(&button);  // 編譯器生成 draw_static_Button

    let textfield = TextField { placeholder: String::from("hi") };
    draw_static(&textfield);  // 編譯器生成 draw_static_TextField
}

編譯後
fn draw_static_Button(item: &Button) {
    item.draw();
}

fn draw_static_TextField(item: &TextField) {
    item.draw();
}
*/

/* 使用 trait object - 執行時決定
fn draw_dynamic(item: &dyn Draw) {
    item.draw();  // 執行時查表決定呼叫哪個方法
}

fn main() {
    let button = Button { width: 10, height: 5 };
    draw_dynamic(&button);  // 執行時決定

    let textfield = TextField { placeholder: String::from("hi") };
    draw_dynamic(&textfield);  // 執行時決定
}
*/
