// Traits

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Clone)]
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

#[derive(Clone)]
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// !預設實作

struct Tweet2 {
    username: String,
    content: String,
}

pub trait Summary2 {
    fn summarize_author(&self) -> String;

    // 可以呼叫預設實作的方法
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary2 for Tweet2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize 使用預設實作
}

// Trait 作為參數

//  trait bound 語法1
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//  trait bound 語法2
/*
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
*/

// !多個 trait bound
trait Display {
    fn display(&self) -> String;
}

impl Display for Article {
    fn display(&self) -> String {
        format!("Article: {}", self.title)
    }
}

fn notify_multiple_arguments(item1: &impl Summary, item2: &impl Display) {
    println!(
        "Breaking news! {} and {}",
        item1.summarize(),
        item2.display()
    );
}
/*
pub fn notify_multiple_arguments_2<T: Summary + Display>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {} and {}",
        item1.summarize(),
        item2.summarize()
    );
}
*/

//  !多個 Trait Bounds
fn notify_multiple_traits_2(item: &(impl Summary + Display)) {
    // item 必須同時實作 Summary 和 Display
    println!("Breaking news! {} and {}", item.summarize(), item.display());
}
/*  完整方法的好處，可以更清晰地指定 trait bound
pub fn notify<T: Summary + Display>(item: &T) {
}
*/

// !當 Trait Bound 變複雜時，使用 where 語句
fn some_function<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Summary,
{
    format!("t: {} and u: {}", t.display(), u.summarize())
}

// !返回 trait function
fn returns_summarize() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}

/* 不能根據條件不同返回不同型別，impl Trait 在編譯時必須確定是單一具體型別，不能是多個型別
fn returns_summarizable() -> impl Summary {
    match random_bool() {
        true => Article {
            title: String::from("Rust is fun"),
            content: String::from("Rust is a systems programming language"),
        },
        false => Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        },
    }
}
 */

pub fn main() {
    println!("✅ chapter10_2");
    let article = Article {
        title: String::from("Rust is fun"),
        content: String::from("Rust is a systems programming language"),
    };
    println!("Article: {}", article.summarize());
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };
    println!("Tweet: {}", tweet.summarize());

    print!("✅ 預設實作 \n");
    let tweet2 = Tweet2 {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };
    println!("Tweet2: {}", tweet2.summarize());

    print!("✅ Trait 作為參數 \n");
    notify(&article);
    notify(&tweet);

    print!("✅ 多個 arguments 的 trait bound \n");
    notify_multiple_arguments(&article, &article);

    print!("✅ 多個 Trait Bounds \n");
    notify_multiple_traits_2(&article);

    print!("✅ 當 Trait Bound 變複雜時，使用 where 語句 \n");
    println!("{}", some_function(&article, &tweet));

    print!("✅ 返回 trait function \n");
    println!("{}", returns_summarize().summarize());
}
