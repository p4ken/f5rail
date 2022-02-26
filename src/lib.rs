mod transition;
mod jww;

use anyhow::Result;
use jww::{
    jww_temp::JwwTemp,
    param::{Func, Param},
};
use std::ffi::OsString;

/// 配線する
pub fn layout(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let param = Param::parse(args)?;

    let mut jww_temp = JwwTemp::new(&param.file);

    match param.func {
        Func::Tc(param) => transition::draw(&mut jww_temp, &param),
    }

    jww_temp.save()
}
