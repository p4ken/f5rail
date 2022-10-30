//! セカンダリアダプタ

use std::{
    fs::File,
    io::{self, BufWriter, Write}, path::Path,
};

use crate::cg::Track;

pub struct MapWriter<W: Write> {
    buf: BufWriter<W>,
}
impl<W: Write> MapWriter<W> {
    pub fn new(write: W) -> Self {
        let buf = BufWriter::new(write);
        Self { buf }
    }
    pub fn write_track(&mut self, name: &str, track: &Track) -> io::Result<()> {
        // todo
        Ok(())
    }
}
impl MapWriter<File> {
    fn create(path: &(impl AsRef<Path> + ?Sized)) -> anyhow::Result<Self> {
        let file = File::create(path)?;
        Ok(Self::new(file))
    }
    pub fn close(self) -> io::Result<()> {
        self.buf.into_inner()?.sync_all()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_track() {
        // assert!(false)
    }
}
