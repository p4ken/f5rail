use std::{
    fs::File,
    path::{self, Path, PathBuf},
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

#[derive(Default)]
pub struct MapPath<'a> {
    given: Option<&'a str>, // 指定されたパス (Argsへの参照)
}
// parent: Option<PathBuf>, // 相対パス指定の親ディレクトリ (ファイルキャッシュの借用)
// 採番よりも中止や上書きのほうが良いかもしれない。
// stem: String,            // フォルダ指定の重複時のベース文字 (引数の借用の借用)
// i: i32,                  // フォルダ指定の重複時の採番 (所有)
// ext: String,             // フォルダ指定の重複時の拡張子 (引数の借用の借用)

// 「～～に作成しました」みたいにしたい。
// パスを決めてからファイルを作成するまでにタイムラグがある問題は残る。
impl<'a> MapPath<'a> {
    pub fn new(given: &'a (impl AsRef<str> + ?Sized)) -> Self {
        let given = given.as_ref();
        let given = Some(given);
        Self { given }
    }

    pub fn absolute(&self) -> Option<&str> {
        self.given
            .and_then(|given| Path::new(given).is_absolute().then(|| given))
    }

    pub fn relative(&self, dir: &(impl AsRef<Path> + ?Sized)) -> Result<PathBuf> {
        let dir = dir.as_ref();
        // givenがない、空、末尾コンポーネントが.の場合、default_under(given)
        // givenの末尾が/、末尾コンポーネントが..の場合、given_under(dir).is_dir()である必要がある。
        // given_under(dir).is_dir()の場合、default_under(given_under(dir))
        // そうでない場合、given_under(dir) に拡張子txtをつける。
        Ok(self.given_under(dir)
                    .unwrap_or_else(|| self.default_under(dir)))
    }

    fn given_under(&self, dir: &Path) -> Option<PathBuf> {
        self.given.map(|given| {
            let mut path = dir.to_path_buf();
            path.push(given);
            // if matches!(
            //     path.components().next_back(),
            //     Some(path::Component::Normal(_))
            // ) && path.extension().is_none()
            if path.extension().is_none() {
                path.set_extension("txt");
            }
            path
        })
    }

    fn default_under(&self, dir: &Path) -> PathBuf {
        let mut path = PathBuf::new();
        for i in 1..1000 {
            path = dir.to_path_buf();
            path.push(format!("他線座標-{i}.txt"));
            if !path.exists() {
                break;
            }
        }
        path
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::a(r"src\agent", r"c:\abc.bvemap", r"c:\abc.bvemap")]
    #[case::b(r"src\agent", r"abc.bvemap", r"src\agent\abc.bvemap")]
    #[case::c(r"src\agent", r".\abc.bvemap", r"src\agent\.\abc.bvemap")]
    #[case::d(r"src\agent", r"..\abc.bvemap", r"src\agent\..\abc.bvemap")]
    #[case::e(r"src\agent", r"abc", r"src\agent\abc.txt")]
    #[case::f(r"src\agent", r"abc.txt", r"src\agent\abc.txt")]
    #[case::g(r"src\agent", r"", r"src\agent\他線座標-1.txt")]
    #[case::h(r"src\agent", r".", r"src\agent\他線座標-1.txt")]
    #[case::i(r"src\agent", r"..", r"src\他線座標-1.txt")]
    #[case::j(r"src", r"agent", r"src\agent\他線座標-1.txt")]
    #[case::k(r"src", r"agent\", r"src\agent\他線座標-1.txt")]
    // #[should_panic(expected = "a")]
    // #[case::l(r"src", r"abc\", r"")]
    fn パス判断(#[case] dir: &str, #[case] name: &str, #[case] exp: &str) {
        let path = MapPath::new(name);
        if let Some(path) = path.absolute() {
            assert_eq!(path, exp);
        } else {
            let path = path.relative(dir);
            assert_eq!(path.unwrap().as_os_str(), exp)
        }
    }
}
