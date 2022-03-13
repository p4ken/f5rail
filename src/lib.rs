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
    let param = match param {
        Ok(param) => param,
        Err(e) => return JwcTemp::export_err(file, e),
    };

    let segments = transition::plot(&param);

    match segments {
        Ok(s) => JwcTemp::export(file, &param.diminish, &s),
        Err(e) => JwcTemp::export_err(file, &e),
    }
}
