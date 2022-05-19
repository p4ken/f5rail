use std::io::Write;

use anyhow::Result;

pub trait Make {
    fn make(&self, writer: &mut impl Write) -> Result<()>;
}
