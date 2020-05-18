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
        let latest = book.get_latest_revision(&page.path).unwrap();
        let revisions = book.get_all_revisions(&page.path).unwrap();
        batch.enqueue(root_ctx.page(page.clone(), latest, 0, 0));
        batch.enqueue(root_ctx.revision_list(page, revisions));
    }

    batch.process_parallel().unwrap();
}
