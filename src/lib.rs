mod agent;
mod app;
mod track;
mod transition;

use std::{ffi::OsStr, fmt::Display};

use anyhow::Result;

use agent::{bat::Args, jww::JwcTemp};
use app::App;
use transition::{unit::Meter, curve::Diminish};

/// 配線する
pub fn layout(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let args = Args::parse(args)?;
    let app = App::new(&args)?;

    match &app {
        App::Transition(file, args) => plot(file, args),
        App::Track(track) => track.export(),
    }
}

/// 緩和曲線を描画する
fn plot(file: &str, param: &Result<transition::Param>) -> Result<()> {
    let mut jwc_temp = JwcTemp::create(file)?;
    match param {
        Ok(p) => {
            let spiral = transition::plot(p);

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

impl Display for Diminish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diminish::Sine => f.write_str("サイン半波長逓減曲線"),
            Diminish::Linear => f.write_str("直線逓減（クロソイド）"),
        }
    }
}
