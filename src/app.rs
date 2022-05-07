use anyhow::{bail, ensure, Result};

use crate::{
    agent::bat::Args,
    track::app::Track,
    transition::{self, app::Transition},
};

use transition::param::Param as TrParam;

#[derive(Debug)]
pub enum App<'a> {
    Transition(Transition),
    Track(Track<'a>),
}

impl<'a> App<'a> {
    /// 機能のファクトリ関数。
    pub fn new(args: &'a Args) -> Result<Self> {
        if let Ok(formula) = args.get("TRANSITION") {
            let file = args.get("FILE")?.into();
            let param = TrParam::parse(&formula, &args);
            Ok(Self::Transition(Transition::new(file, param)))
        } else if let Ok(track) = args.get("TRACK") {
            ensure!(track.str() == "X");
            Ok(Self::Track(Track::new(&args)))
        } else {
            bail!("機能を指定してください")
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn コマンドライン引数にファイル名がなければエラー() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/TRANSITION:1"),
        ];
        let args = Args::parse(args).unwrap();
        let e = App::new(&args).unwrap_err();
        assert_eq!(e.to_string(), "FILEを指定してください")
    }
}
