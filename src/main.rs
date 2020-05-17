mod book;
mod error;
mod renderer;

use book::Book;

fn main() {
    let book = book::FileBook::new("./db");

    let mut batch =
        renderer::BatchRenderer::new("RukiWiki", &book.get_latest_revision("menu").unwrap())
            .unwrap();

    for page in book.get_all_pages().unwrap() {
        let revision = book.get_latest_revision(&page.path).unwrap();
        batch.enqueue_page(page, revision, 0, 0);
    }

    let result = batch.process_serial().unwrap();
    println!("{:?}", result);
}
