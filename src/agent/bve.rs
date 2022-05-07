use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Result};

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

pub struct MapPath<'a> {
    given: &'a Path, // 指定されたパス (引数の借用)
}
// parent: Option<PathBuf>, // 相対パス指定の親ディレクトリ (ファイルキャッシュの借用)
// 採番よりも中止や上書きのほうが良いかもしれない。
// stem: String,            // フォルダ指定の重複時のベース文字 (引数の借用の借用)
// i: i32,                  // フォルダ指定の重複時の採番 (所有)
// ext: String,             // フォルダ指定の重複時の拡張子 (引数の借用の借用)

// 「～～に作成しました」みたいにしたい。
// パスを決めてからファイルを作成するまでにタイムラグがある問題は残る。
impl<'a> MapPath<'a> {
    pub fn new(given: &'a (impl AsRef<Path> + ?Sized)) -> Self {
        Self {
            given: given.as_ref(),
        }
    }

    pub fn absolute(&self) -> Option<&Path> {
        self.given.is_absolute().then(|| self.given)
    }

    pub fn relative(&self, dir: &(impl AsRef<Path> + ?Sized)) -> PathBuf {
        let mut path = dir.as_ref().to_path_buf();
        path.push(self.given);
        if path.extension().is_none() {
            path.set_extension("txt");
        }
        path
    }
}

impl Default for MapPath<'_> {
    fn default() -> Self {
        Self::new("map.txt")
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(r"abc.bvemap", r"dir\to\proj\abc.bvemap")]
    #[case(r"abc", r"dir\to\proj\abc.txt")]
    fn パス判断(#[case] name: &str, #[case] exp: &str) {
        let path = MapPath::new(name);
        if let Some(path) = path.absolute() {
            assert_eq!(path.as_os_str(), exp);
        } else {
            let path = path.relative(r"dir\to\proj");
            assert_eq!(path.as_os_str(), exp)
        }
    }
}
