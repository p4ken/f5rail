mod agent;
mod bve;
mod cad;
mod cg;
mod factory;
mod jww;
mod track;
mod track_;
mod transition;
mod unit;

use std::ffi::OsStr;

use anyhow::Result;

use agent::bat::Args;
use factory::App;

/// 配線する
pub fn layout(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let args = Args::parse(args)?;

    match App::new(&args)? {
        App::Transition(transition) => transition.draw(),
        App::Track(track) => track.export(),
    }
}
