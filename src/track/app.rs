use std::path::{Path, PathBuf};

use anyhow::{Error, Result};

use crate::agent::{
    bat::Args,
    bve::{MapFile, MapPath},
    jww::JwcTemp,
};

#[derive(Debug)]
/// 外部変形 "TRACK"
pub struct Track<'a> {
    args: &'a Args,
}

impl<'a> Track<'a> {
    pub fn new(args: &'a Args) -> Self {
        Self { args }
    }

    /// エントリーポイント。
    ///
    /// JWC_TEMPファイルへの出力に失敗したらエラーを返す。
    /// それ以外のエラーはJWC_TEMPファイルに出力される。
    pub fn export(&self) -> Result<()> {
        match self.make_map_file() {
            Ok(map_path) => self.show_map_path(&map_path),
            Err(e) => self.show_err(&e),
        }
    }

    /// 他線座標をBVEマップに出力する。
    fn make_map_file(&self) -> Result<PathBuf> {
        let map_path = self.map_path()?;
        let _map = MapFile::create(&map_path)?;
        Ok(map_path)
    }

    /// BVEマップのパスを決める。
    fn map_path(&self) -> Result<PathBuf> {
        let map_path = MapPath::new(self.args.map_name()?);
        if let Some(map_path) = map_path.absolute() {
            Ok(map_path.to_path_buf())
        } else {
            let project_dir = JwcTemp::read(self.args.temp_path()?)?.project_dir()?;
            Ok(map_path.relative(&project_dir))
        }
    }

    /// 成功メッセージをJWC_TEMPファイルに出力する。
    fn show_map_path(&self, path: &Path) -> Result<()> {
        self.create_temp_file()?
            .notice(format!("{} に出力しました", path.to_string_lossy()))
    }

    /// エラーをJWC_TEMPファイルに出力する。
    fn show_err(&self, e: &Error) -> Result<()> {
        self.create_temp_file()?.error(e)
    }

    /// JWC_TEMP.TXTを作成する。
    fn create_temp_file(&self) -> Result<JwcTemp> {
        JwcTemp::create(self.args.temp_path()?)
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
