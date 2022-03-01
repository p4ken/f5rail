use std::{fmt::Display, fs::File, io::Write};

use anyhow::{bail, Context, Result};

use super::sjis::to_sjis;
use crate::transition::{self, polyline::Polyline};

type TcFunc = transition::param::Func;

/// 入出力用の座標ファイル。
///
/// JWC_TEMP.TXTのフォーマット（参考）
/// http://mintleaf.sakura.ne.jp/cad/jwc_temp.html
pub struct JwcTemp {
    file: File,
}

impl JwcTemp {
    /// エラーをファイルに書き出す
    pub fn export_err(path: &str, e: &impl Display) -> Result<()> {
        Self::create(path)?.err(e)
    }

    /// ポリラインをファイルに書き出す
    pub fn export(path: &str, func: &TcFunc, polyline: &Polyline) -> Result<()> {
        let mut jwc_temp = Self::create(path)?;

        jwc_temp.notice(to_string(func))?;

        bail!("未実装")
    }

    /// ファイルを作成する
    fn create(path: &str) -> Result<Self> {
        let file = File::create(path).context("JWC_TEMP.TXTを作成できませんでした。")?;
        Ok(Self { file })
    }

    /// エラー文を出力する
    /// - 最初のエラーのみが表示される
    /// - エラーがあれば、エラー以外の座標などはすべて無視される
    fn err(&mut self, s: &impl Display) -> Result<()> {
        self.puts(&format!("he{}", s))
    }

    /// 注意文を表示する
    /// - 最後の注意文のみ表示される
    /// - 座標の間に出力すると、座標が途切れてしまう
    fn notice(&mut self, s: &str) -> Result<()> {
        self.puts(&format!("h#{}", s))
    }

    /// 文字列と改行を書き込む
    fn puts(&mut self, s: &str) -> Result<()> {
        for bytes in [&to_sjis(s)[..], b"\r\n"] {
            self.file
                .write_all(bytes)
                .context("JWC_TEMP.TXTへの書き込みに失敗しました。")?;
        }
        Ok(())
    }

    fn _a(&self) -> Result<()> {
        println!("pl");
        println!("0 0");
        println!("100 -100");
        println!("200 -400");
        println!("300 -900");
        println!("#");
        Ok(())
    }
}

fn to_string(tc_func: &TcFunc) -> &str {
    match tc_func {
        TcFunc::Sin => "サイン半波長逓減",
        TcFunc::Linear => "直線逓減",
    }
}
