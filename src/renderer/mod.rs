use super::book;
use super::error::*;

#[derive(Debug, serde::Serialize)]
struct Context<'a> {
    page: &'a book::Page,
    content: &'a str,
}

pub struct Renderer {
    templates: tera::Tera,
    opts: pulldown_cmark::Options,
}

impl Renderer {
    pub fn new() -> Result<Self> {
        Ok(Self::new_with_options(
            tera::Tera::new("templates/*")?,
            pulldown_cmark::Options::all(),
        ))
    }
    fn new_with_options(templates: tera::Tera, opts: pulldown_cmark::Options) -> Self {
        Self {
            templates: templates,
            opts: opts,
        }
    }

    pub fn render(&self, page: &book::Page, revision: &book::Revision) -> Result<String> {
        let content = self.render_markdown(&revision.content)?;
        let data = self.render_page(Context {
            page: page,
            content: &content,
        })?;
        Ok(data)
    }
    fn render_markdown(&self, input: &str) -> Result<String> {
        let mut buf = String::new();
        pulldown_cmark::html::push_html(
            &mut buf,
            pulldown_cmark::Parser::new_ext(input, self.opts),
        );
        Ok(buf)
    }
    fn render_page(&self, ctx: Context) -> Result<String> {
        let ctx = tera::Context::from_serialize(ctx)?;
        Ok(self.templates.render("content.html", &ctx)?)
    }
}
