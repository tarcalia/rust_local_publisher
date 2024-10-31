use crate::models::book::Book;

pub fn handle_input(input: &str) {
    let splitted_word: Vec<&str> = input.split(' ').collect();

    let command = String::from(splitted_word[0].trim());
    let title = String::from(splitted_word[1].trim());
    let author = String::from(splitted_word[2].trim());
    let isbn = String::from(splitted_word[3].trim());

    let book = Book{
        title,
        author,
        isbn
    };

    match command.as_str() {
        "add" => add_book(book),
        "remove" => remove_book(book),
        "list" => list_books(),
        "get" => get_book(book),
        _ => {}
    }
}

fn add_book(book: Book) {
    println!("Adding book {}", book.isbn);
}

fn remove_book(book: Book) {
    println!("Adding book {}", book.title);

}

fn list_books() {
    println!("Adding book");

}

fn get_book(book: Book) {
    println!("Adding book {}", book.author);

}