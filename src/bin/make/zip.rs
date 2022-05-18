use std::{fs::File, path::Path};

use anyhow::Result;
use derive_more::{Deref, DerefMut};
use zip::{ZipWriter, write::FileOptions};

// const OPTIONS: FileOptions = FileOptions::default();

#[derive(Deref, DerefMut)]
pub struct Package(ZipWriter<File>);

impl Package {
    pub fn new(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let writer = ZipWriter::new(File::create(path)?);
        Ok(Self(writer))
    }

    pub fn write_file() {
        // zip.start_file("test/☃.txt", options)?;
        // zip.write_all(b"Hello, World!\n")?;
    }
}

// 参考
// https://github.com/zip-rs/zip/blob/master/examples/write_sample.rs
