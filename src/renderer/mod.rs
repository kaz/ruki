use super::book;
use super::error::*;

mod context;
pub use context::*;

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

    fn md_to_html(&self, input: &str) -> Result<String> {
        let mut buf = String::new();
        pulldown_cmark::html::push_html(
            &mut buf,
            pulldown_cmark::Parser::new_ext(input, self.opts),
        );
        Ok(buf)
    }

    fn render<C>(&self, ctx: &C) -> Result<String>
    where
        C: Renderable + serde::Serialize,
    {
        Ok(self.templates.render(
            ctx.template_definition(),
            &tera::Context::from_serialize(ctx)?,
        )?)
    }
}

pub struct BatchRenderer {
    renderer: std::sync::Arc<Renderer>,
    jobs: std::sync::Arc<std::sync::Mutex<Vec<Context>>>,
}

impl BatchRenderer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            renderer: std::sync::Arc::new(Renderer::new()?),
            jobs: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
        })
    }

    pub fn enqueue(&self, ctx: Context) {
        let jobs = self.jobs.clone();
        jobs.lock().unwrap().push(ctx);
    }

    pub fn process_parallel(&self) -> Result<Vec<String>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let jobs = self.jobs.clone();
        let count = jobs.lock().unwrap().len();

        for _ in 0..8 {
            let renderer = self.renderer.clone();
            let jobs = self.jobs.clone();
            let sender = sender.clone();

            std::thread::spawn(move || loop {
                let mut jobs = jobs.lock().unwrap();
                match jobs.pop() {
                    Some(mut ctx) => sender
                        .send(match ctx.preproc(renderer.as_ref()) {
                            Err(e) => Err(format!("{}", e)),
                            _ => match renderer.render(&ctx) {
                                Err(e) => Err(format!("{}", e)),
                                Ok(data) => Ok((ctx, data)),
                            },
                        })
                        .unwrap(),
                    None => break,
                }
            });
        }

        Ok((0..count)
            .map(|_| receiver.recv())
            .collect::<std::result::Result<std::result::Result<Vec<_>, _>, _>>()??
            .into_iter()
            .map(|x| x.1)
            .collect())
    }
}
