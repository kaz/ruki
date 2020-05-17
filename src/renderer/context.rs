use super::*;

pub trait Renderable {
    fn distribute_path(&self) -> String;
    fn template_definition(&self) -> &'static str {
        unreachable!();
    }
    fn preproc(&mut self, renderer: &super::Renderer) -> Result<()>;
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum Context {
    Page(Page),
}
impl Renderable for Context {
    fn distribute_path(&self) -> String {
        match self {
            Self::Page(ctx) => ctx.page.path.clone(),
        }
    }
    fn template_definition(&self) -> &'static str {
        match self {
            Self::Page(_) => "page.html",
        }
    }
    fn preproc(&mut self, renderer: &super::Renderer) -> Result<()> {
        match self {
            Self::Page(ctx) => ctx.content = renderer.md_to_html(&ctx.content)?,
        }
        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Page {
    page: book::Page,
    content: String,
    revisions_count: u64,
    attachments_count: u64,
    common: RootContext,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RootContext {
    title: String,
    menu_content: String,
}
impl RootContext {
    pub fn new<S>(title: S, menu: book::Revision) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: title.into(),
            menu_content: menu.content,
        }
    }
    pub fn page(
        &self,
        page: book::Page,
        revision: book::Revision,
        revisions_count: u64,
        attachments_count: u64,
    ) -> Context {
        Context::Page(Page {
            page: page,
            content: revision.content,
            revisions_count: revisions_count,
            attachments_count: attachments_count,
            common: self.clone(),
        })
    }
}
