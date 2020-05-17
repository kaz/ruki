use super::error::*;
use super::renderer;

mod file;
pub use file::*;

pub trait Distributor {
    fn publish(&self, ctx: renderer::Context, content: String) -> Result<()>;
}
