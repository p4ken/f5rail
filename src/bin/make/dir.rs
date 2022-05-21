use std::{
    ffi::OsStr,
    fs::DirEntry,
    io,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub struct Dir {
    childlen: Vec<PathBuf>,
}

impl Dir {
    pub fn open(path: &str) -> Result<Self> {
        let dir1 = Path::new(path).read_dir()?;
        let dir2 = Path::new(path).join("sub").read_dir()?;
        Ok(dir2.chain(dir1).collect::<io::Result<Self>>()?)
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
