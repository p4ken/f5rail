use std::{
    ffi::OsStr,
    fs::{DirEntry},
    io,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Result};
use derive_more::{Deref, From, IntoIterator};

use crate::bat::BatFile;

#[derive(Deref)]
pub struct Dir {
    path: PathBuf,
}

impl Dir {
    pub fn open(path: impl AsRef<OsStr>) -> Result<Self> {
        let dir = Self::new(path);
        ensure!(dir.exists());
        Ok(dir)
    }

    pub fn all_bats(&self) -> Result<Bats> {
        let root = self.read_dir()?;
        let sub = self.sub().read_dir()?;
        Ok(root.chain(sub).collect::<io::Result<_>>()?)
    }

    fn sub(&self) -> Self {
        Self::new(self.join("sub"))
    }

    fn new(path: impl AsRef<OsStr>) -> Self {
        let path = Path::new(&path).to_path_buf();
        Self { path }
    }
}

impl AsRef<Path> for Dir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[derive(IntoIterator, From)]
pub struct Bats(Vec<BatFile>);

impl FromIterator<DirEntry> for Bats {
    fn from_iter<T: IntoIterator<Item = DirEntry>>(iter: T) -> Self {
        iter.into_iter()
            .filter_map(|entry| BatFile::try_from(entry).ok())
            .collect::<Vec<BatFile>>()
            .into()
    }
}
