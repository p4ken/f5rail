use std::{
    fs::File,
    path::{self, Path, PathBuf},
};

use anyhow::{ensure, Result};
use derive_more::Deref;

/// BVEマップファイル
pub struct MapFile {
    _file: File,
}

impl MapFile {
    pub fn create(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let path = path.as_ref();
        ensure!(
            !path.exists(),
            "{} はすでに存在しています",
            path.to_string_lossy()
        );
        let file = File::create(&path)?;
        Ok(Self { _file: file })
    }
}

pub struct MapPath {
    path: PathBuf,
}

// 「～～に作成しました」みたいにしたい。
// パスを決めてからファイルを作成するまでにタイムラグがある問題は残る。
impl MapPath {
    // クロージャを受け取りたい
    pub fn new(
        given: &(impl AsRef<Path> + ?Sized),
        proj_dir: &(impl AsRef<Path> + ?Sized),
    ) -> Self {
        let builder = PathBuilder::new(given.as_ref());
        let path = builder.build();
        Self { path }
    }
}

impl AsRef<Path> for MapPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

struct PathBuilder {
    buf: PathBuf,
}

impl PathBuilder {
    fn new(given: &(impl AsRef<Path> + ?Sized)) -> Self {
        let buf = given.as_ref().to_path_buf();
        Self { buf }
    }
    fn build(mut self) -> PathBuf {
        self.buf
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use rstest::rstest;
    use tempfile::TempDir;

    use super::*;

    #[rstest]
    #[case("", "map.txt")]
    #[case("a", "a.txt")]
    fn パス判断(#[case] given: &str, #[case] exp: &str) {
        let test_dir = TestDir::new().unwrap();
        let path = MapPath::new(given, &test_dir.0);
        assert_eq!(path.as_ref(), Path::new(exp));
    }

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
