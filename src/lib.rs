mod format;
mod transition;

use anyhow::{bail, Result};
use format::{
    args::{Args, Param},
    jwc_temp::JwcTemp,
};
use std::ffi::OsString;

/// 配線する
pub fn layout(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let args = Args::parse(args)?;

    match args.param {
        Param::Transition(param) => {
            let polyline = transition::plot(&param);
            JwcTemp::save(&args.file, &polyline)
        }
        _ => bail!("未実装"),
    }
}
