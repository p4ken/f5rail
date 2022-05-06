

use anyhow::{Error, Result};


use crate::agent::{
    bat::{Args},
    bve::{MapFile, MapPath},
    jww::JwcTemp,
};

#[derive(Debug)]
pub struct Track<'a> {
    args: &'a Args,
}

impl<'a> Track<'a> {
    pub fn new(args: &'a Args) -> Self {
        Self { args }
    }

    /// 外部変形のエントリーポイント。
    ///
    /// JWC_TEMPファイルへの出力に失敗したらエラーを返す。
    /// それ以外のエラーはJWC_TEMPファイルに出力される。
    pub fn export(&self) -> Result<()> {
        self.make_map().or_else(|e| self.show_err(&e))
    }

    /// 他線座標をBVEマップに出力する。
    fn make_map(&self) -> Result<()> {
        let jwc_temp = JwcTemp::read(self.args.temp_path()?)?;
        let map_path = MapPath::new(self.args.map_name()?);
        let map_path = match map_path.absolute() {
            Some(map_path) => map_path.to_path_buf(),
            None => map_path.relative(&jwc_temp.project_dir()?),
        };
        let _map = MapFile::create(&map_path)?;
        Ok(())
    }

    /// 成功メッセージをJWC_TEMPファイルに出力する。
    fn show_path() {}

    /// エラーをJWC_TEMPファイルに出力する。
    fn show_err(&self, e: &Error) -> Result<()> {
        JwcTemp::create(self.args.temp_path()?)?.error(e)
    }

    /// 他線座標を計算する。
    fn plot(&self) {}
}

// pub struct Args<'a> {
//     // temp_path: &'a str,
//     args: &'a bat::Args,
// }

impl Args {
    // pub fn new(args: &'a bat::Args) -> Result<Self> {
    //     // let temp_path = args.get("TEMP")?.str();
    //     Ok(Self { args })
    // }

    fn temp_path(&self) -> Result<&str> {
        self.try_get("TEMP")
    }

    fn temp_0_path(&self) -> Result<&str> {
        self.try_get("TEMP_0")
    }

    fn temp_x_path(&self) -> Result<&str> {
        self.try_get("TEMP_Z")
    }

    fn map_name(&self) -> Result<&str> {
        self.try_get("出力ファイル名")
    }

    fn try_get(&self, s: &str) -> Result<&str> {
        let val = self.get(s)?;
        Ok(val.str())
    }
}
