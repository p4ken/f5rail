mod io;
mod transition;

use anyhow::{Result, Ok};
use io::args::Args;
use std::ffi::OsString;

/// 配線する
pub fn layout(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let args = Args::parse(args)?;

    // let mut jww_temp = JwwTemp::new(&args.file);

    // match args.func {
    //     // Func::Tc(param) => transition::draw(&param),
    // }

    // jww_temp.save()
    Ok(())
}
