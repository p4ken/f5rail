use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use anyhow::{Result, ensure};
use derive_more::{From, IntoIterator, Deref};

use crate::bat::Bat;

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

    pub fn create(path: impl AsRef<OsStr>) -> Result<Self> {
        let dir = Self::new(path);
        if !dir.exists() {
            fs::create_dir(&dir.path)?;
        }
        Ok(dir)
    }

    pub fn bats(&self) -> Result<Bats> {
        // let sub = self.sub_dir()?;
        Ok(self.read_dir()?.collect::<io::Result<_>>()?)
    }

    fn new(path: impl AsRef<OsStr>) -> Self {
        let path = Path::new(&path).to_path_buf();
        Self { path }
    }

    // fn sub_dir(&self) -> io::Result<fs::ReadDir> {
    //     let sub_dir
    //     self.path.join("sub").read_dir()
    // }
}

impl AsRef<Path> for Dir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[derive(IntoIterator, From)]
pub struct Bats(Vec<Bat>);

impl FromIterator<DirEntry> for Bats {
    fn from_iter<T: IntoIterator<Item = DirEntry>>(iter: T) -> Self {
        iter.into_iter()
            .filter_map(|entry| Bat::try_from(entry).ok())
            .collect::<Vec<Bat>>()
            .into()
    }
}
