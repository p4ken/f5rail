mod agent;
mod app;
mod track;
mod transition;

use std::ffi::OsStr;

use anyhow::Result;

use agent::{jwc_temp, bat::Args};
use app::App;

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
    let mut jwc_temp = jwc_temp::create(file)?;
    match param {
        Ok(p) => {
            let spiral = transition::plot(p);
            jwc_temp.diminish(p.diminish)?.spiral(&spiral)
        }
        Err(e) => jwc_temp.error(&e),
    }
}
