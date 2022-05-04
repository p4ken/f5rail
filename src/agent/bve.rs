use std::{
    fs::File,
    path::{Path},
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
        let ext = path.extension();
        let mut path = path.to_path_buf();
        let mut i = 1;
        while path.exists() {
            let mut name = stem
                .with_context(|| format!("{} はすでに存在しています", path.to_string_lossy()))?
                .to_os_string();
            name.push(format!("-{}.", i));
            if let Some(ext) = ext { name.push(ext) }
            path = dir.to_path_buf();
            path.push(name);
            i += 1;
        }
        Ok(Self(File::create(path)?))
    }
}

// struct MapPath(PathBuf);

// // 「～～に作成しました」みたいにしやすい。
// // パスを決めてからファイルを作成するまでにタイムラグがある問題は残る。
// impl MapPath {
//     pub fn new(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
//     }
// }
