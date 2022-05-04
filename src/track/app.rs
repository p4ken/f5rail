//! エラーをJWC_TEMPファイルに出力するような処理をここにまとめる

use std::{
    ffi::{OsStr, OsString},
};

use anyhow::Result;
use derive_more::Constructor;

use crate::agent::bat;

#[derive(Constructor)]
pub struct Track<'a> {
    args: &'a Args,
}

impl<'a> Track<'a> {
    pub fn export(&self) -> Result<()> {

        Ok(())
    }
}

pub struct Args {
    temp: OsString,
    buf: bat::Args,
}

impl Args {
    fn temp_path(&self) -> &OsStr {
        &self.temp
    }

    fn temp_0_path(&self) -> Result<&OsStr> {
        Ok(OsStr::new(""))
    }

    fn temp_x_path(&self) -> Result<&OsStr> {
        Ok(OsStr::new(""))
    }

    fn map_name(&self) -> Result<&OsStr> {
        Ok(OsStr::new(""))
    }
}
