use std::ffi::OsStr;

use anyhow::{bail, ensure, Context, Result};

use crate::{agent::bat::Args, track::{self, app::Track}, transition};

pub enum App<'a> {
    Transition(String, Result<transition::Param>),
    Track(Track<'a>),
}

impl<'a> App<'a> {
    pub fn new(args: &'a Args) -> Result<Self> {
        if let Ok(formula) = args.get("TRANSITION") {
            let file = args.get("FILE")?.into();
            let param = transition::Param::parse(&formula, &args);
            Ok(Self::Transition(file, param))
        } else if let Ok(track) = args.get("TRACK") {
            ensure!(track.str() == "X");
            Ok(Self::Track(Track::new(&args)))
        } else {
            bail!("機能を指定してください")
        }
    }
}
