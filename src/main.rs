mod book;
mod dist;
mod error;
mod renderer;

use book::Book;

fn main() {
    let book = book::FileBook::new("./db");
    let dist = dist::FileDistributor::new("./public", true).unwrap();
    let batch = renderer::BatchRenderer::new(dist).unwrap();
    let root_ctx =
        renderer::RootContext::new("RukiWiki", book.get_latest_revision("menu").unwrap());

    for page in book.get_all_pages().unwrap() {
        let revision = book.get_latest_revision(&page.path).unwrap();
        batch.enqueue(root_ctx.page(page, revision, 0, 0));
    }

    let result = batch.process_parallel().unwrap();
    println!("{:?}", result);
}
