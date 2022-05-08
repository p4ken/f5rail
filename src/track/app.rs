use std::path::{Path, PathBuf};

use anyhow::{Error, Result};

use crate::agent::{
    bat::Args,
    bve::{MapFile, MapPath},
    jww::{self, JwcTemp},
};

use super::space::{Polyline, Relative};

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
    ///
    /// ファイル入出力を行なう。それ以外は下層へ移譲する。
    fn make_map_file(&self) -> Result<PathBuf> {
        let map_path = self.map_path()?;
        let _map = MapFile::create(&map_path)?;
        // TODO: トラック名を取得
        // TRACK_X.batの引数を保存しておく必要がある
        // let track_name = self.args.track_name()?;

        // let relative = self.plot_relative()?;

        // TODO: トラック名と座標リストをBVEマップに書き込む

        Ok(map_path)
    }

    /// BVEマップのパスを生成する。
    fn map_path(&self) -> Result<PathBuf> {
        let map_path = self.args.map_name().map(MapPath::new).unwrap_or_default();
        if let Some(map_path) = map_path.absolute() {
            Ok(PathBuf::from(map_path))
        } else {
            let project_dir = JwcTemp::read(self.args.temp_path()?)?.project_dir()?;
            Ok(map_path.relative(&project_dir)?)
        }
    }

    /// トラック名と座標リストをBVEマップに書き込む。
    ///
    /// JWC_TEMPファイルから読み込む。それ以外は下層へ移譲する。マップファイルへの書き込みは上層へ移譲する。
    fn plot_relative(&self) -> Result<Vec<Relative>> {
        // 図形から、自線・他線それぞれの連続線を見つける
        let track_0 = JwcTemp::read_polyline(self.args.temp_0_path()?)?;
        let track_x = JwcTemp::read_polyline(self.args.temp_x_path()?)?;

        Relative::between(&track_0, &track_x)
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
}

impl Args {
    fn temp_path(&self) -> Result<&str> {
        self.get_str("TEMP")
    }

    fn temp_0_path(&self) -> Result<&str> {
        self.get_str("TEMP_0")
    }

    fn temp_x_path(&self) -> Result<&str> {
        self.get_str("TEMP_Z")
    }

    fn map_name(&self) -> Result<&str> {
        self.get_str("出力ファイル名")
    }
}

impl JwcTemp {
    fn read_polyline(path: &(impl AsRef<Path> + ?Sized)) -> Result<Polyline> {
        let cache = JwcTemp::read(path)?;
        todo!()
    }
}

trait WriteRelative {
    fn write(&self, relative: &Relative) -> Result<()>;
}
