use super::*;

pub fn template_def(ctx: &Context) -> Option<&'static str> {
    match ctx {
        Context::AttachmentList(_) => Some("attachment_list.html"),
        Context::Page(_) => Some("page.html"),
        Context::PageList(_) => Some("page_list.html"),
        Context::RevisionList(_) => Some("revision_list.html"),
        _ => None,
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum Context {
    Attachment(Attachment),
    AttachmentList(AttachmentList),
    Page(Page),
    PageList(PageList),
    RevisionList(RevisionList),
}

#[derive(Debug, serde::Serialize)]
pub struct Attachment {
    page: book::Page,
    // TODO
    common: Common,
}

#[derive(Debug, serde::Serialize)]
pub struct AttachmentList {
    page: book::Page,
    // TODO
    common: Common,
}

#[derive(Debug, serde::Serialize)]
pub struct Page {
    pub page: book::Page,
    pub content: String,
    pub revisions_count: u64,
    pub attachments_count: u64,

    pub common: Common,
}

#[derive(Debug, serde::Serialize)]
pub struct PageList {
    pages: Vec<book::Page>,

    common: Common,
}

#[derive(Debug, serde::Serialize)]
pub struct RevisionList {
    page: book::Page,
    revisions: Vec<book::Revision>,

    common: Common,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Common {
    pub title: String,
    pub menu_content: String,
}
