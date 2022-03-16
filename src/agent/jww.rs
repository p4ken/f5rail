use std::{fmt::Display, fs::File, io::Write};

use anyhow::{Context, Result};

use super::sjis::to_sjis;
use crate::transition::{curve::Diminish, spiral::{Atom, Spiral}};

/// 入出力用の座標ファイル。
///
/// JWC_TEMP.TXTのフォーマット（参考）
/// http://mintleaf.sakura.ne.jp/cad/jwc_temp.html
pub struct JwcTemp {
    file: File,
}

impl JwcTemp {
    /// エラーをファイルに書き出す。
    pub fn export_err(path: &str, e: &impl Display) -> Result<()> {
        Self::create(path)?.alert(e)
    }

    /// 緩和曲線 `v` をファイルに書き出す。
    pub fn export(path: &str, diminish: &Diminish, v: &Spiral) -> Result<()> {
        let mut file = Self::create(path)?;
        file.notice(diminish.into())?;
        // file.curve(&segments)?;
        Ok(())
    }

    /// ファイルを作成する。
    fn create(path: &str) -> Result<Self> {
        let file = File::create(path).context("JWC_TEMP.TXTを作成できませんでした。")?;
        Ok(Self { file })
    }

    /// エラーを出力する。
    /// 
    /// - 最初のエラーのみが表示される。
    /// - エラーがあれば、エラー以外の座標などはすべて無視される。
    fn alert(&mut self, s: &impl Display) -> Result<()> {
        self.puts(&format!("he{}", s))
    }

    /// 注意を出力する。
    /// 
    /// - 最後の注意のみ表示される
    /// - 座標の間に出力すると、座標が途切れてしまう
    fn notice(&mut self, s: &str) -> Result<()> {
        self.puts(&format!("h#{}", s))
    }

    /// 曲線を出力する。
    fn curve(&mut self, vertex: &Vec<(f64, f64)>) -> Result<()> {
        self.puts("pl")?;
        for v in vertex {
            self.puts(&format!("{} {}", v.0, v.1))?;
        }
        self.puts("#")
    }

    /// 文字列と改行を出力する。
    fn puts(&mut self, s: &str) -> Result<()> {
        for bytes in [&to_sjis(s)[..], b"\r\n"] {
            self.file
                .write_all(bytes)
                .context("JWC_TEMP.TXTへの書き込みに失敗しました。")?;
        }
        Ok(())
    }
}

impl From<&Diminish> for &str {
    fn from(spiral: &Diminish) -> Self {
        match spiral {
            Diminish::Sine => "サイン半波長逓減",
            Diminish::Linear => "直線逓減（クロソイド曲線）",
        }
    }
}
