mod geo;
mod jww;

use anyhow::{Context, Result};
use geo::transition;
use jww::{
    jww_temp::JwwTemp,
    param::{Func, Param},
};
use std::env;

fn main() -> Result<()> {
    let param = Param::parse(env::args_os()).context("引数のパースに失敗しました。")?;

    let mut jww_temp = JwwTemp::new(&param.file);

    match param.func {
        Func::Tc(p) => transition::draw(&mut jww_temp, &p),
    }
}
