use std::{
    fs,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use zip::{write::FileOptions, CompressionMethod};

type FileBuf = BufWriter<fs::File>;
type ZipBuf = zip::ZipWriter<FileBuf>;

pub struct Package {
    zip: ZipBuf,
    option: FileOptions,
    parent: PathBuf,
}

impl Package {
    pub fn new_in(dir: &str) -> Result<Self> {
        let zip = Self::create_zip(dir)?;
        let option = FileOptions::default().compression_method(CompressionMethod::Deflated);
        let parent = PathBuf::from(dir);
        Ok(Self {
            zip,
            option,
            parent,
        })
    }

    pub fn create_file(&mut self, path: impl AsRef<Path>) -> Result<File> {
        let path = path.as_ref();
        let path = path
            .to_str()
            .with_context(|| format!("非UTF-8のパス {}", path.display()))?;
        println!("{}", path);

        let real = self.create_real_file(path)?;
        let zip = self.create_zipped_file(path)?;
        Ok(File { zip, real })
    }

    pub fn finish(&mut self) -> Result<()> {
        self.zip.finish()?;
        Ok(())
    }

    fn create_zip(dir: &str) -> Result<ZipBuf> {
        if !Path::new(dir).exists() {
            println!("Creating {}", dir);
        }
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
        Ok(ZipBuf::new(zip_file))
    }

    fn create_zipped_file(&mut self, name: &str) -> Result<&mut ZipBuf> {
        let zip_path = format!("BVE座標計算/{}", name);
        self.zip.start_file(zip_path, self.option)?;
        Ok(&mut self.zip)
    }

    fn create_real_file(&self, name: &str) -> Result<FileBuf> {
        if let Some(parent) = Path::new(name).parent() {
            let parent = self.parent.join(parent);
            if !parent.exists() {
                println!("Creating {}", parent.display());
            }
            fs::create_dir_all(parent)?;
        }

        let real_path = self.parent.join(name);
        println!("Creating {}", real_path.display());
        let real_file = fs::File::create(real_path)?;
        Ok(BufWriter::new(real_file))
    }
}

pub struct File<'a> {
    zip: &'a mut ZipBuf,
    real: FileBuf,
}

impl<'a> Write for File<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.zip.write_all(buf)?;
        self.real.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.real.flush()
    }
}
