mod book;
mod error;
mod renderer;

use book::Book;

fn main() {
    let book = book::FileBook::new("./db");
    println!("{:?}", book.get_all_pages().unwrap());
    println!("{:?}", book.get_page("").unwrap());
    println!("{:?}", book.get_all_revisions("").unwrap());
    println!("{:?}", book.get_latest_revision("").unwrap());

    let pg = book.get_page("").unwrap();
    let rv = book.get_latest_revision(&pg.path).unwrap();

    let r = renderer::Renderer::new().unwrap();
    println!("{}", r.render(&pg, &rv).unwrap());
}
