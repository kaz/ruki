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
    RevisionList(RevisionList),
}
impl Renderable for Context {
    fn distribute_path(&self) -> String {
        match self {
            Self::Page(ctx) => ctx.page.path.clone(),
            Self::RevisionList(ctx) => std::path::Path::new(&ctx.page.path)
                .join("_revisions")
                .to_str()
                .unwrap()
                .into(),
        }
    }
    fn template_definition(&self) -> &'static str {
        match self {
            Self::Page(_) => "page.html",
            Self::RevisionList(_) => "revision_list.html",
        }
    }
    fn preproc(&mut self, renderer: &super::Renderer) -> Result<()> {
        match self {
            Self::Page(ctx) => ctx.content = renderer.md_to_html(&ctx.content)?,
            _ => {}
        }
        Ok(())
    }
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

    pub fn revision_list(&self, page: book::Page, revisions: Vec<book::Revision>) -> Context {
        Context::RevisionList(RevisionList {
            page: page,
            revisions: revisions,
            common: self.clone(),
        })
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

#[derive(Debug, serde::Serialize)]
pub struct RevisionList {
    page: book::Page,
    revisions: Vec<book::Revision>,
    common: RootContext,
}
