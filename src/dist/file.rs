use super::*;

use super::renderer::Renderable;
use std::io::Write;

pub struct FileDistributor {
    root: String,
}

impl FileDistributor {
    pub fn new<S>(root: S, overwrite: bool) -> Result<Self>
    where
        S: Into<String>,
    {
        let root = root.into();
        let path = std::path::Path::new(&root);
        if path.exists() {
            if overwrite {
                std::fs::remove_dir_all(path)?;
            } else {
                return Err(InternalError::new(format!(
                    "`{}` already exists. aborted",
                    root
                )));
            }
        }
        Ok(Self { root: root })
    }
}

impl Distributor for FileDistributor {
    fn publish(&self, ctx: renderer::Context, content: String) -> Result<()> {
        let path = std::path::Path::new(&self.root).join(ctx.distribute_path());
        std::fs::create_dir_all(&path)?;

        let mut file =
            std::io::BufWriter::new(std::fs::File::create(path.as_path().join("index.html"))?);
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
