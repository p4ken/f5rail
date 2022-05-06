use std::{ffi::OsStr};

use anyhow::{Error, Result};
use derive_more::Constructor;

use crate::agent::{
    bat, bat_0,
    bve::{MapFile, MapPath},
    jww::JwcTemp,
};

#[derive(Constructor)]
pub struct Track<'a> {
    args: &'a Args<'a>,
}

impl<'a> Track<'a> {
    /// 外部変形のエントリーポイント。
    ///
    /// JWC_TEMPファイルへの出力に失敗したらエラーを返す。
    /// それ以外のエラーはJWC_TEMPファイルに出力される。
    pub fn export(args: &bat_0::TrackArgs /* TODO: bat::Args */) -> Result<()> {
        let args = Args::new(&args);
        let app = Track::new(&args);
        app.make_map().or_else(|e| app.show_error(&e))
    }

    /// 他線座標をBVEマップに出力する。
    fn make_map(&self) -> Result<()> {
        let jwc_temp = JwcTemp::read(self.args.temp_path())?;
        let map_path = MapPath::new(self.args.map_name()?);
        let map_path = match map_path.absolute() {
            Some(map_path) => map_path.to_path_buf(),
            None => map_path.relative(&jwc_temp.project_dir()?),
        };
        let _map = MapFile::create(&map_path)?;
        Ok(())
    }

    /// 成功メッセージをJWC_TEMPファイルに出力する。
    fn show_success() {}

    /// エラーをJWC_TEMPファイルに出力する。
    fn show_error(&self, e: &Error) -> Result<()> {
        JwcTemp::create(self.args.temp_path())?.error(e)
    }

    /// 他線座標を計算する。
    fn plot(&self) {}
}

#[derive(Constructor)]
pub struct Args<'a> {
    args: &'a bat_0::TrackArgs, /* TODO: bat::Args */
}

impl<'a> Args<'a> {
    // これは必須
    fn temp_path(&self) -> &OsStr {
        self.args.temp.as_os_str() // 一時的な実装
    }

    fn parse(_buf: bat::Args) {}
    // ↑ 全部まとめてパースする案
    // ↓ 個別にオンザフライでパースする案
    fn temp_0_path(&self) -> Result<&OsStr> {
        Ok(OsStr::new(""))
    }

    fn temp_x_path(&self) -> Result<&OsStr> {
        Ok(OsStr::new(""))
    }

    fn map_name(&self) -> Result<&OsStr> {
        Ok(&OsStr::new(&self.args.map)) // 一時的
    }
}
