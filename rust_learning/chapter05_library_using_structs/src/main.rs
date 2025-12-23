#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    is_read: bool,
}

impl Book {
    fn add(title: String, author: String, pages: u32) -> Self {
        Self {
            title,
            author,
            pages,
            is_read: false,
        }
    }

    fn mark_as_read(&mut self) {
        self.is_read = true;
    }
}

fn main() {
    let mut book1 = Book::add(
        String::from("Programming Rust"),
        String::from("Jim Blandy"),
        222,
    );
    let mut book2 = Book::add(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        325,
    );

    book1.mark_as_read();
    let library = [book1, book2];

    println!("The library has the following books:");
    for book in library {
        dbg!(book);
    }
}
