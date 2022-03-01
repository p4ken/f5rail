mod agent;
mod transition;

use std::ffi::OsString;

use anyhow::{bail, Result};

use agent::{
    batch::{Args, Param, TcParam},
    jww::JwcTemp,
};

/// 配線する
pub fn layout(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let args = Args::parse(args)?;

    match args.param {
        Param::Transition(param) => layout_transition(&args.file, &param),
        Param::Encoding => agent::encode(&args.file),
        _ => bail!("未実装"),
    }
}

/// 緩和曲線を配線する
fn layout_transition(jwc_temp: &str, param: &Result<TcParam>) -> Result<()> {
    let param = match param {
        Ok(param) => param,
        Err(e) => return JwcTemp::export_err(jwc_temp, e),
    };

    let polyline = transition::draw(param);

    let polyline = match &polyline {
        Ok(polyline) => polyline,
        Err(e) => return JwcTemp::export_err(jwc_temp, e),
    };

    JwcTemp::export(jwc_temp, &param.func, polyline)
}
