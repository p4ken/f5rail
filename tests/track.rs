use std::{
    ffi::{OsStr, OsString},
    fs::OpenOptions,
    io::Write,
    path::Path,
};

use anyhow::{Context, Result};
use rstest::rstest;
use tempfile::{NamedTempFile, TempDir, TempPath};

#[rstest]
#[case("ascii.txt")]
#[case("日本語.txt")]
fn relative(#[case] map_name: &str) -> Result<()> {
    let jwc_temp_0 = TestFile::create()?;
    let jwc_temp_x = TestFile::create()?;
    let jwc_temp = TestFile::create()?;
    let project_dir = TestDir::create()?;
    jwc_temp.write_path(&project_dir.path().join("foo.jww"))?;

    let args = vec![
        Arg::new("/TRACK:X"),
        Arg::new("/TEMP_0:").push(jwc_temp_0.path()),
        Arg::new("/TEMP_X:").push(jwc_temp_x.path()),
        Arg::new("/TEMP:").push(jwc_temp.path()),
        Arg::new("/出力ファイル名:").push(map_name),
    ];
    f5rail::layout(args)?;

    assert!(project_dir.path().join(map_name).exists());

    jwc_temp_0.close()?;
    jwc_temp_x.close()?;
    project_dir.close()?;
    Ok(())
}

struct TestFile(TempPath);

impl TestFile {
    fn create() -> Result<Self> {
        let file = NamedTempFile::new_in("tests")?;
        Ok(Self(file.into_temp_path()))
    }
    fn path(&self) -> &Path {
        &self.0
    }
    fn write_path(&self, path: &impl AsRef<Path>) -> Result<()> {
        let path = path
            .as_ref()
            .to_str()
            .context("The directory contains non-UTF-8 strings.")?;
        let mut file = OpenOptions::new().write(true).open(&self.0)?;
        writeln!(file, "file={}", path)?;
        Ok(())
    }
    fn close(self) -> Result<()> {
        Ok(self.0.close()?)
    }
}

struct TestDir(TempDir);

impl TestDir {
    fn create() -> Result<Self> {
        let dir = TempDir::new_in("tests")?;
        Ok(Self(dir))
    }
    fn path(&self) -> &Path {
        self.0.path()
    }
    fn close(self) -> Result<()> {
        Ok(self.0.close()?)
    }
}

struct Arg(OsString);

impl Arg {
    fn new(s: &(impl AsRef<OsStr> + ?Sized)) -> Self {
        Self(s.as_ref().to_os_string())
    }
    fn push(mut self, s: &(impl AsRef<OsStr> + ?Sized)) -> Self {
        self.0.push(s);
        self
    }
}

impl AsRef<OsStr> for Arg {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}
