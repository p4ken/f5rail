use std::{
    ffi::OsStr,
    fs::{DirEntry, ReadDir},
    io,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

pub struct Dir {
    childlen: Vec<PathBuf>,
}

impl Dir {
    pub fn open(path: &str) -> Result<Self> {
        let dir = Path::new(path)
            .read_dir()
            .with_context(|| "{path}を開けません")?;
        let sub_dir = Path::new(path).join("sub").read_dir();
        let io_result = match sub_dir {
            Ok(sub_dir) => sub_dir.chain(dir).collect::<io::Result<Self>>(),
            Err(e) => dir.collect::<io::Result<Self>>(),
        };
        Ok(io_result?)
    }

    pub fn bat_iter(&self) -> impl IntoIterator<Item = &Path> {
        self.childlen
            .iter()
            .filter(|path| path.extension() == Some(OsStr::new("bat")))
            .map(PathBuf::as_path)
    }
}

impl FromIterator<DirEntry> for Dir {
    fn from_iter<T: IntoIterator<Item = DirEntry>>(iter: T) -> Self {
        let childlen = iter
            .into_iter()
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        Self { childlen }
    }
}
