mod agent;
mod factory;
mod track;
mod transition;

use std::{ffi::OsStr};

use anyhow::Result;

use agent::{bat::Args};
use factory::App;


/// 配線する
pub fn layout(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let args = Args::parse(args)?;

    match App::new(&args)? {
        App::Transition(transition) => transition.draw(),
        App::Track(track) => track.export(),
    }
}
