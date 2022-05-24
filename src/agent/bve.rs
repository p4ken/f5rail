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
        ensure!(!path.exists(), "{} はすでに存在しています", path.display());
        let file = File::create(&path)?;
        Ok(Self { file })
    }

    /// トラック名と相対座標をマップファイルに書き込む
    pub fn write_track(
        &mut self,
        _name: &str,
        _relative: &impl IntoIterator<Item = (i32, i32)>,
    ) -> Result<()> {
        // TODO
        Ok(())
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
        mut proj_dir: impl FnMut() -> Result<T>,
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
mod tests {
    use std::fs;

    use rstest::rstest;
    use tempfile::TempDir;

    use super::*;

    #[rstest]
    #[case::empty("", vec!["map.txt"])]
    #[case::extension("a", vec!["a.txt"])]
    #[case::numbering("b", vec!["b-1.txt"])]
    #[case::numbering2("c", vec!["c-2.txt"])]
    #[case::numbering3("c-1", vec!["c-1-1.txt"])]
    #[case::dir("d", vec!["d", "map.txt"])]
    #[case::dir2("e", vec!["e", "map-1.txt"])]
    fn 相対パス判断(#[case] given: &str, #[case] relative: Vec<&str>) {
        let proj = TestDir::new().unwrap();
        let mut expected = proj.path().to_path_buf();
        expected.push(relative.iter().collect::<PathBuf>());
        let actual = MapPath::build(given, || Ok(proj.path())).unwrap();
        assert_eq!(actual.as_ref(), expected);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn 絶対パス判断() {
        let given = r"C:\dir\file.txt";
        let path = MapPath::build(given, || Ok("dir")).unwrap();
        assert_eq!(path.as_ref(), Path::new(given));
    }

    #[derive(Deref)]
    struct TestDir(TempDir);

    impl TestDir {
        fn new() -> Result<Self> {
            let dir = Self(TempDir::new_in(".")?);
            File::create(dir.path_with(vec!["a"]))?;
            File::create(dir.path_with(vec!["b.txt"]))?;
            File::create(dir.path_with(vec!["c.txt"]))?;
            File::create(dir.path_with(vec!["c-1.txt"]))?;
            fs::create_dir(dir.path_with(vec!["d"]))?;
            fs::create_dir(dir.path_with(vec!["e"]))?;
            File::create(dir.path_with(vec!["e", "map.txt"]))?;
            Ok(dir)
        }

        fn path_with(&self, s: Vec<&str>) -> PathBuf {
            let mut path = self.0.path().to_path_buf();
            path.push(s.iter().collect::<PathBuf>());
            path
        }
    }
}
