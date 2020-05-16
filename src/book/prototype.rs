use super::*;

pub trait Book {
    fn get_page(&self, path: &str) -> Result<Page> {
        for page in self.get_all_pages()? {
            if page.path == path {
                return Ok(page);
            }
        }
        Err(InternalError::new(format!("no such page: {}", path)))
    }
    fn get_latest_revision(&self, path: &str) -> Result<Revision> {
        self.get_all_revisions(path.clone())?
            .pop()
            .ok_or(InternalError::new(format!("no revisions on {}", path)))
    }

    fn get_all_pages(&self) -> Result<Vec<Page>>;
    fn get_all_revisions(&self, path: &str) -> Result<Vec<Revision>>;
    fn put_revision(&self, path: &str, content: &str) -> Result<(Page, Revision)>;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Page {
    pub path: String,

    pub created: u64,
    pub updated: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Revision {
    pub content: String,

    pub created: u64,
}
