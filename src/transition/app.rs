use std::fmt::Display;

use anyhow::Result;

use crate::agent::jww::JwcTemp;

use super::{curve::Diminish, unit::Meter, Param};

/// 外部変形 "TRANSITION"
#[derive(Debug)]
pub struct Transition {
    file: String,
    param: Result<Param>,
}

impl Transition {
    pub fn new(file: &str, param: Result<Param>) -> Self {
        let file = file.to_string();
        Self { file, param }
    }

    /// エントリーポイント
    pub fn draw(&self) -> Result<()> {
        let mut jwc_temp = JwcTemp::create(&self.file)?;
        match &self.param {
            Ok(p) => {
                let spiral = super::plot(&p);

                // 緩和曲線 `spiral` を出力する。
                for stroke in spiral.iter() {
                    match stroke
                        .center()
                        .zip(stroke.r().filter(|r| r.meter().abs() < 100_000.0))
                    {
                        Some((c, r)) => jwc_temp.curve(&c, &r, &stroke.a0(), &stroke.a1())?,
                        None => jwc_temp.straight(&stroke.p0(), &stroke.p1())?,
                    }
                }

                // 成功メッセージを出力する。
                jwc_temp.notice(format!("{}を描画しました。", p.diminish))?;

                Ok(())
            }
            Err(e) => jwc_temp.error(&e),
        }
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
