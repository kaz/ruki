use super::book;
use super::dist::Distributor;
use super::error::*;

mod context;
pub use context::*;

pub struct Renderer {
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

pub struct BatchRenderer<D>
where
    D: Distributor,
{
    distributor: D,
    renderer: Renderer,
    jobs: std::sync::Mutex<Vec<Context>>,
}

impl<D> BatchRenderer<D>
where
    D: Distributor + Sync + Send + 'static,
{
    pub fn new(distributor: D) -> Result<Self> {
        Ok(Self {
            distributor: distributor,
            renderer: Renderer::new()?,
            jobs: std::sync::Mutex::new(vec![]),
        })
    }

    pub fn enqueue(&self, ctx: Context) {
        self.jobs.lock().unwrap().push(ctx);
    }

    pub fn process_parallel(self) -> Result<()> {
        let batch = std::sync::Arc::new(self);

        let (sender, receiver) = std::sync::mpsc::channel();
        let count = batch.jobs.lock().unwrap().len();

        for _ in 0..8 {
            let batch = batch.clone();
            let sender = sender.clone();

            std::thread::spawn(move || loop {
                let mut jobs = batch.jobs.lock().unwrap();
                let mut ctx = match jobs.pop() {
                    Some(ctx) => ctx,
                    None => break,
                };

                sender
                    .send(match ctx.preproc(&batch.renderer) {
                        Err(e) => Err(format!("{}", e)),
                        _ => match batch.renderer.render(&ctx) {
                            Err(e) => Err(format!("{}", e)),
                            Ok(content) => match batch.distributor.publish(ctx, content) {
                                Err(e) => Err(format!("{}", e)),
                                _ => Ok(()),
                            },
                        },
                    })
                    .unwrap();
            });
        }

        (0..count)
            .map(|_| receiver.recv())
            .collect::<std::result::Result<std::result::Result<Vec<_>, _>, _>>()??;
        Ok(())
    }
}
