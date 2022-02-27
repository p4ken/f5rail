mod format;
mod transition;

use anyhow::{Result, Ok};
use format::args::Args;
use std::ffi::OsString;

/// 配線する
pub fn layout(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let args = Args::parse(args)?;
    // match args.param {
    //     // Args::Func::Transition(param)
    // }

    // let mut jww_temp = JwwTemp::new(&args.file);

    // match args.func {
    //     // Func::Tc(param) => transition::draw(&param),
    // }

    // jww_temp.save()
    Ok(())
}
