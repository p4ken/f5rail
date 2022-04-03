//! 入出力用の座標ファイル。
//!
//! JWC_TEMP.TXTのフォーマット（参考）
//! http://mintleaf.sakura.ne.jp/cad/jwc_temp.html

use std::{fmt::Display, fs::File, io};

use anyhow::{Context, Result};
use encoding_rs::SHIFT_JIS;

use crate::transition::{
    canvas::{Point, Spiral},
    curve::{Central, Diminish, Radius},
    unit::{Meter, Rad, Vector},
};

/// 出力用の座標ファイルを作成する。
///
/// すでに存在する場合は上書きする。
pub fn create(path: &str) -> Result<Write> {
    let file = File::create(path).context("JWC_TEMP.TXTを作成できませんでした。")?;
    Ok(Write { file })
}

pub struct Write {
    file: File,
}

impl Write {
    /// エラー `e` を出力する。
    ///
    /// - 最初のエラーのみが表示される。
    /// - エラーがあれば、エラー以外の座標などはすべて無視される。
    pub fn error(&mut self, e: &impl Display) -> Result<()> {
        self.puts(&format!("he{}", e))
    }

    /// 成功メッセージを出力する。
    pub fn diminish(&mut self, diminish: Diminish) -> Result<&mut Self> {
        self.notice(format!("{}を描画しました。", diminish))
            .and(Ok(self))
    }

    /// 緩和曲線 `spiral` を出力する。
    pub fn spiral(&mut self, spiral: &Spiral) -> Result<()> {
        for stroke in spiral.iter() {
            match stroke
                .center()
                .zip(stroke.r().filter(|r| r.meter().abs() < 100_000.0))
            {
                Some((c, r)) => self.curve(c, r, stroke.a0(), stroke.a1())?,
                None => self.straight(stroke.p0(), stroke.p1())?,
            }
        }
        Ok(())
    }

    /// 注意を出力する。
    ///
    /// 最後の注意のみ表示される。
    ///
    /// 座標の間に出力すると、座標が途切れてしまう。
    fn notice<T: AsRef<str>>(&mut self, s: T) -> Result<()> {
        self.puts(format!("h#{}", s.as_ref()))
    }

    /// 曲線を出力する。
    fn curve(&mut self, c: Point, r: Radius, a0: Central, a1: Central) -> Result<()> {
        let (a0, a1) = if a0 < a1 { (a0, a1) } else { (a1, a0) };
        self.puts(format!(
            "ci {} {} {} {} {}",
            c.x(),
            c.y(),
            r.meter().abs(),
            a0.deg(),
            a1.deg()
        ))
    }

    /// 直線を出力する。
    fn straight(&mut self, p0: Point, p1: Point) -> Result<()> {
        self.puts(format!("{} {} {} {}", p0.x(), p0.y(), p1.x(), p1.y()))
    }

    /// 文字列と改行を出力する。
    fn puts<T: AsRef<str>>(&mut self, s: T) -> Result<()> {
        // TODO:
        // SHIFT_JISではなくCP932にしたほうがいい。
        // 外部からの入力ファイルを読みだす場合は必須と思われる。
        // - https://crates.io/crates/codepage
        // - https://crates.io/search?q=windows%20encoding&sort=downloads
        let (sjis, _, _) = SHIFT_JIS.encode(s.as_ref());
        for bytes in [&sjis[..], b"\r\n"] {
            io::Write::write_all(&mut self.file, bytes)
                .context("JWC_TEMP.TXTへの書き込みに失敗しました。")?;
        }
        Ok(())
    }
}

impl Display for Diminish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diminish::Sine => f.write_str("サイン半波長逓減曲線"),
            Diminish::Linear => f.write_str("直線逓減（クロソイド）"),
        }
    }
}
