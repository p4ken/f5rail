use std::{fs, io::Write, path::Path};

use anyhow::Result;
use zip::{write::FileOptions, ZipWriter};

pub struct Package {
    dir: String,
    zip: ZipWriter<fs::File>,
    option: FileOptions,
}

impl Package {
    pub fn new_in(dir: &str) -> Result<Self> {
        fs::create_dir_all(dir)?;
        let zip_path = format!(
            "{}/f5rail-{}-{}-{}.zip",
            dir,
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
        );
        println!("Creating {}", zip_path);
        let zip = ZipWriter::new(fs::File::create(zip_path)?);
        let option = FileOptions::default();
        let dir = dir.to_owned();
        Ok(Self { zip, option, dir })
    }

    pub fn create_file(&mut self, path: &(impl AsRef<str> + ?Sized)) -> Result<FileWriter> {
        if let Some(parent) = Path::new(path.as_ref()).parent() {
            fs::create_dir_all(parent)?;
        }
        let path = format!("BVE座標計算/{}", path.as_ref());
        let zip = &mut self.zip;
        zip.start_file(path, self.option)?;
        let real = fs::File::create(format! {""})?;
        Ok(FileWriter { zip, real })
    }

    pub fn finish(&mut self) -> Result<()> {
        self.zip.finish()?;
        Ok(())
    }
}

pub trait WriteZip {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn flush(&mut self) -> std::io::Result<()>;
}

pub trait WriteReal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn flush(&mut self) -> std::io::Result<()>;
}

pub struct FileWriter<'a> {
    zip: &'a mut ZipWriter<fs::File>,
    real: fs::File,
}

// write_allはFileWriter自身に実装しても良さそう。

impl<'a> WriteZip for FileWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.zip.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.zip.flush()
    }
}

impl<'a> WriteReal for FileWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.real.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.real.flush()
    }
}

// let mut zip = Package::new()?;
// zip.write_file("abc")?;

// 各ファイル
// ファイルパス → 標準出力
//     ↓ ファイル読み込み, 変換
// u8配列
//     ↓ zip書き込み, ファイル書き出し
