#[derive(Debug, Clone)]
struct Book {
    name: String,
}

impl Book {
    fn show_book_name(&self) {
        // ....
        println!("name is: {}", self.name);
    }
}

use std::ops::Deref;

#[derive(Debug, Clone)]
struct MyBook {
    p: Book,
    author: String,
}

impl MyBook {
    fn new() -> MyBook {
        MyBook {
            p: Book {
                name: "知乎大全".to_string(),
            },
            author: "我是谁".to_string(),
        }
    }
}

impl Deref for MyBook {
    type Target = Book;

    fn deref<'a>(&'a self) -> &'a Book {
        &self.p
    }
}
fn main() {
    let mut mybook = MyBook::new();
    println!("{:?}", mybook.name);
    mybook.show_book_name();
}
