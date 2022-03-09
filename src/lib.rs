mod agent;
mod transition;

use std::ffi::OsString;

use anyhow::{bail, Result};

use agent::{
    bat::{self, Args},
    jww::JwcTemp,
    sjis,
};

/// 配線する
pub fn layout(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let args = Args::parse(args)?;

    match args {
        Args::Transition(args) => plot(&args),
        Args::Encode(path) => sjis::encode(&path),
        _ => bail!("未実装"),
    }
}

/// 緩和曲線を描画する
fn plot(args: &bat::Transition) -> Result<()> {
    let param = match &args.param {
        Ok(param) => param,
        Err(e) => return JwcTemp::export_err(&args.file, &e),
    };

    let segments = transition::plot(&param);

    match segments {
        Ok(s) => JwcTemp::export(&args.file, &param.spiral, &s),
        Err(e) => JwcTemp::export_err(&args.file, &e),
    }
}
