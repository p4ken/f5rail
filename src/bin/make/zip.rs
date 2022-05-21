use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Result;
use zip::write::FileOptions;

type BufFile = BufWriter<fs::File>;
type BufZip = zip::ZipWriter<BufFile>;

pub struct Package {
    dir: String,
    zip: BufZip,
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
        let zip_file = BufWriter::new(fs::File::create(zip_path)?);
        let zip = BufZip::new(zip_file);
        let option = FileOptions::default();
        let dir = dir.to_owned();
        Ok(Self { zip, option, dir })
    }

    pub fn create_file(&mut self, path: &str) -> Result<File> {
        let zip = &mut self.zip;
        let zip_path = format!("BVE座標計算/{}", path);
        zip.start_file(zip_path, self.option)?;

        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        let real_path = format!("{}/{}", self.dir, path);
        let real = BufWriter::new(fs::File::create(real_path)?);

        Ok(File { zip, real })
    }

    pub fn finish(&mut self) -> Result<()> {
        self.zip.finish()?;
        Ok(())
    }
}

pub struct File<'a> {
    zip: &'a mut BufZip,
    real: BufFile,
}

impl<'a> Write for File<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.zip.write_all(buf)?;
        self.real.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.zip.flush()?;
        self.real.flush()
    }
}

// 各ファイル
// ファイルパス → 標準出力
//     ↓ ファイル読み込み, 変換
// u8配列
//     ↓ zip書き込み, ファイル書き出し
