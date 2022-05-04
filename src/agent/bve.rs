use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};

/// BVEマップファイル
pub struct MapFile(File);

impl MapFile {
    pub fn create(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let path = path.as_ref();
        let dir = path
            .parent()
            .with_context(|| format!("{} の上位フォルダがありません", path.to_string_lossy()))?;
        ensure!(dir.exists(), "{} を開けません", dir.to_string_lossy());

        let stem = path.file_stem();
        let (mut path, ext) = match path.extension() {
            Some(ext) => (path.to_path_buf(), ext),
            None => {
                let ext = OsStr::new("txt");
                (path.with_extension(ext), ext)
            }
        };
        let mut i = 1;
        while path.exists() {
            let mut name = stem
                .with_context(|| format!("{} はすでに存在しています", path.to_string_lossy()))?
                .to_os_string();
            name.push(format!("-{}.", i));
            name.push(ext);
            path = dir.to_path_buf();
            path.push(name);
            i += 1;
        }
        Ok(Self(File::create(path)?))
    }
}

pub struct MapPath<'a>(&'a Path);

// 「～～に作成しました」みたいにしたい。
// パスを決めてからファイルを作成するまでにタイムラグがある問題は残る。
impl<'a> MapPath<'a> {
    pub fn new(map_name: &'a (impl AsRef<Path> + ?Sized)) -> Self {
        Self(map_name.as_ref())
    }

    pub fn absolute(&self) -> Option<&Path> {
        self.0.is_absolute().then(|| self.0)
    }

    pub fn relative(&self, project_dir: &(impl AsRef<Path> + ?Sized)) -> PathBuf {
        let mut map_path = project_dir.as_ref().to_path_buf();
        map_path.push(self.0);
        map_path
    }
}

// pub struct MapPath(PathBuf);

// impl MapPath {
//     fn new(path: &(impl AsRef<Path> + ?Sized)) {
//         let mut path = path.as_ref().to_path_buf();
//         match path.extension() {
//             Some(ext) => Self { path, ext },
//             None => path.set_extension("txt"),
//         }
//         let ext = path.extension().unwrap_or("txt");
//     }
// }
