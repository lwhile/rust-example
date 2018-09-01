struct Book {
    // `&'static str` is a reference that only distribute to read only memory
    author: &'static str,
    title: &'static str,
    year: u32
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.author);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("i mutably borrowed {} -{} edition", book.title, book.author)
}

fn main() {
    let immutabook = Book {
        author: "D",
        title: "d",
        year: 1979,
    };

    let mut mutbook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutbook);

    new_edition(&mut mutbook);
    new_edition(&mut immutabook);
}