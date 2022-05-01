use std::{
    ffi::OsStr,
    fs::{self, DirEntry, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Error, Result};
use derive_more::Deref;
use encoding_rs::SHIFT_JIS;

use crate::make::Make;

#[derive(Deref)]
pub struct Bat {
    path: PathBuf,
}

impl TryFrom<DirEntry> for Bat {
    type Error = Error;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        let path = entry.path();
        ensure!(path.extension() == Some(OsStr::new("bat")));
        Ok(Self { path })
    }
}

impl Make for Bat {
    fn make(&self, out: &impl AsRef<Path>) -> Result<()> {
        // 読み込み
        let mut utf8 = String::new();
        File::open(&self.path)?.read_to_string(&mut utf8)?;

        // バージョン表示
        utf8 = utf8.replace("(VERSION)", env!("CARGO_PKG_VERSION"));

        // 文字コード変換
        let (sjis, _, _) = SHIFT_JIS.encode(&utf8);

        // 書き込み
        if let Some(out_dir) = out.as_ref().parent() {
            fs::create_dir_all(out_dir)?;
        }
        File::create(out)?.write_all(&sjis[..])?;

        Ok(())
    }
}
