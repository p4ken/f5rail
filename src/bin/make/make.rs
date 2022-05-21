use std::path::Path;

use anyhow::Result;

pub trait Make{
    fn make(&self, out: &impl AsRef<Path>) -> Result<()>;
}
