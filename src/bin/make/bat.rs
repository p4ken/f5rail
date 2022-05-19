use std::{
    ffi::OsStr,
    fs::{self, DirEntry, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Error, Result};
use encoding_rs::SHIFT_JIS;

use crate::make::Make;

pub struct BatFile {
    path: PathBuf,
}

impl BatFile {
    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

impl TryFrom<DirEntry> for BatFile {
    type Error = Error;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        let path = entry.path();
        ensure!(path.extension() == Some(OsStr::new("bat")));
        Ok(Self { path })
    }
}

impl Make for BatFile {
    fn make(&self, writer: &mut impl Write) -> Result<()> {
        // 読み込み
        let mut utf8 = String::new();
        File::open(&self.path)?.read_to_string(&mut utf8)?;

        // バージョン表示
        utf8 = utf8.replace("(VERSION)", env!("CARGO_PKG_VERSION"));

        // 文字コード変換
        let (sjis, _, _) = SHIFT_JIS.encode(&utf8);

        // 書き込み
        writer.write_all(&sjis[..])?;

        Ok(())
    }
}
