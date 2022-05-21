use std::{
    ffi::OsStr,
    fs::DirEntry,
    io,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use derive_more::{Deref};

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

    pub fn bat_paths(&self) -> Result<Vec<String>> {
        let root = self.read_dir()?;
        let sub = self.sub().read_dir()?;
        root.chain(sub).collect::<io::Result<BatPaths>>()?.0
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

struct BatPaths(Result<Vec<String>>);

impl FromIterator<DirEntry> for BatPaths {
    fn from_iter<T: IntoIterator<Item = DirEntry>>(iter: T) -> Self {
        let paths = iter
            .into_iter()
            .filter_map(BatPath::new)
            .map(BatPath::try_to_string)
            .collect::<Result<Vec<_>>>();
        Self(paths)
    }
}

#[derive(Deref)]
struct BatPath(PathBuf);

impl BatPath {
    fn new(entry: DirEntry) -> Option<Self> {
        let path = entry.path();
        let is_bat = path.extension() == Some(OsStr::new("bat"));
        is_bat.then(|| Self(path))
    }

    fn try_to_string(self) -> Result<String> {
        let s = self.to_str().with_context(|| {
            format!(
                "batファイルのパスに非UTF-8の文字が含まれています {}",
                self.to_string_lossy()
            )
        })?;
        Ok(s.to_string())
    }
}
