mod agent;
mod transition;

use std::ffi::OsString;

use anyhow::Result;

use agent::{bat::Args, jww::JwcTemp, sjis};

/// 配線する
pub fn layout(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let args = Args::parse(args)?;

    match &args {
        Args::Transition(file, args) => plot(file, args),
        Args::Encode(path) => sjis::encode(&path),
        _ => todo!(),
    }
}

/// 緩和曲線を描画する
fn plot(file: &str, param: &Result<transition::Param>) -> Result<()> {
    match param {
        Ok(p) => {
            let spiral = transition::plot(&p);
            JwcTemp::export(file, &p.diminish, &spiral)
        }
        Err(e) => JwcTemp::export_err(file, &e),
    }
}
