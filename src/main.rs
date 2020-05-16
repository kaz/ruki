mod book;
mod error;

use book::Book;

fn main() {
    let book = book::FileBook::new("./db");
    println!("{:?}", book.get_all_pages().unwrap());
    println!("{:?}", book.get_page("").unwrap());
    println!("{:?}", book.get_all_revisions("").unwrap());
    println!("{:?}", book.get_latest_revision("").unwrap());
}
