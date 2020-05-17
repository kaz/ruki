mod context;

use super::book;
use super::error::*;

struct Renderer {
    templates: tera::Tera,
    opts: pulldown_cmark::Options,
}

impl Renderer {
    fn new() -> Result<Self> {
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

    fn to_html(&self, input: &str) -> Result<String> {
        let mut buf = String::new();
        pulldown_cmark::html::push_html(
            &mut buf,
            pulldown_cmark::Parser::new_ext(input, self.opts),
        );
        Ok(buf)
    }

    fn render(&self, ctx: &context::Context) -> Result<String> {
        Ok(self.templates.render(
            context::template_def(ctx).unwrap(),
            &tera::Context::from_serialize(ctx)?,
        )?)
    }
}

pub struct BatchRenderer {
    renderer: Renderer,
    common_context: context::Common,
    jobs: std::sync::Arc<std::sync::Mutex<Vec<context::Context>>>,
}

impl BatchRenderer {
    pub fn new<S>(title: S, menu: &book::Revision) -> Result<Self>
    where
        S: Into<String>,
    {
        let renderer = Renderer::new()?;
        let ctx = context::Common {
            title: title.into(),
            menu_content: renderer.to_html(&menu.content)?,
        };
        Ok(Self {
            renderer: renderer,
            common_context: ctx,
            jobs: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
        })
    }

    pub fn enqueue_page(
        &mut self,
        page: book::Page,
        revision: book::Revision,
        revisions_count: u64,
        attachments_count: u64,
    ) {
        let ctx = context::Context::Page(context::Page {
            page: page,
            content: revision.content.to_string(),
            revisions_count: revisions_count,
            attachments_count: attachments_count,
            common: self.common_context.clone(),
        });
        let jobs = self.jobs.clone();
        jobs.lock().unwrap().push(ctx);
    }

    pub fn process_serial(&mut self) -> Result<Vec<String>> {
        let jobs = self.jobs.clone();
        let mut jobs = jobs.lock().unwrap();
        jobs.iter_mut().map(|x| self.process(x)).collect()
    }

    fn process(&self, ctx: &mut context::Context) -> Result<String> {
        match ctx {
            context::Context::Page(p) => {
                p.content = self.renderer.to_html(&p.content)?;
            }
            _ => unimplemented!(),
        }
        self.renderer.render(ctx)
    }
}
