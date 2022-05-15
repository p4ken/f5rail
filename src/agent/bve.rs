use std::{
    ffi::OsString,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Result};
use derive_more::{Deref, DerefMut};

/// BVEマップファイル
pub struct MapFile {
    file: File,
}

impl MapFile {
    /// ファイルシステムにマップファイルを作成する
    pub fn create(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let path = path.as_ref();
        ensure!(
            !path.exists(),
            "{} はすでに存在しています",
            path.to_string_lossy()
        );
        let file = File::create(&path)?;
        Ok(Self { file })
    }

    /// トラック名と相対座標をマップファイルに書き込む
    pub fn write_track(
        &mut self,
        name: &str,
        relative: &impl IntoIterator<Item = (i32, i32)>,
    ) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Deref, DerefMut)]
pub struct MapPath {
    buf: PathBuf,
}

// 「～～に作成しました」みたいにしたい。
// パスを決めてからファイルを作成するまでにタイムラグがある問題は残る。
impl MapPath {
    pub fn build<T>(
        given: &(impl AsRef<Path> + ?Sized),
        proj_dir: impl Fn() -> Result<T>,
    ) -> Result<Self>
    where
        PathBuf: From<T>,
    {
        let given = given.as_ref();
        let buf = match given.is_absolute() {
            true => PathBuf::new(),
            false => PathBuf::from(proj_dir()?),
        };
        let mut path = Self { buf };
        path.push(given);
        if given.file_name().is_none() || path.is_dir() {
            path.push("map");
        }
        if path.extension().is_none() {
            path.set_extension("txt");
        }
        if path.exists() {
            path.add_number();
        }
        Ok(path)
    }

    fn add_number(&mut self) {
        let stem = self.file_stem().unwrap().to_os_string();
        let ext = self.extension().unwrap().to_os_string();
        let mut i = 1;
        while self.exists() {
            let mut name = OsString::new();
            name.push(&stem);
            name.push(format!("-{i}."));
            name.push(&ext);

            self.pop();
            self.push(name);
            i += 1;
        }
    }
}

impl AsRef<Path> for MapPath {
    fn as_ref(&self) -> &Path {
        &self.buf
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use rstest::rstest;
    use tempfile::TempDir;

    use super::*;

    #[rstest]
    #[case::empty("", "map.txt")]
    #[case::extension("a", "a.txt")]
    #[case::numbering("b", "b-1.txt")]
    #[case::numbering2("c", "c-2.txt")]
    #[case::numbering3("c-1", "c-1-1.txt")]
    #[case::dir("d", r"d\map.txt")]
    #[case::dir2("e", r"e\map-1.txt")]
    #[case::absolute(r"C:\dir\file.txt", r"C:\dir\file.txt")]
    fn パス判断(#[case] given: &str, #[case] expected: &str) {
        let proj = TestDir::new().unwrap();
        let path = MapPath::build(given, || Ok(proj.path())).unwrap();
        assert!(
            path.ends_with(expected),
            "{} vs. {expected}",
            path.to_string_lossy()
        );
    }

    #[derive(Deref)]
    struct TestDir(TempDir);

    impl TestDir {
        fn new() -> Result<Self> {
            let dir = Self(TempDir::new_in(".")?);
            File::create(dir.path_with("a"))?;
            File::create(dir.path_with("b.txt"))?;
            File::create(dir.path_with("c.txt"))?;
            File::create(dir.path_with("c-1.txt"))?;
            fs::create_dir(dir.path_with("d"))?;
            fs::create_dir(dir.path_with("e"))?;
            File::create(dir.path_with(r"e\map.txt"))?;
            Ok(dir)
        }

        fn path_with(&self, s: &str) -> PathBuf {
            let mut path = self.0.path().to_path_buf();
            path.push(s);
            path
        }
    }
}
