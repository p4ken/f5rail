mod agent;
mod transition;

use std::ffi::OsStr;

use anyhow::Result;

use agent::{bat::Args, jwc_temp};

/// 配線する
pub fn layout(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let args = Args::parse(args)?;

    match &args {
        Args::Transition(file, args) => plot(file, args),
        Args::Track => Ok(()),
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
