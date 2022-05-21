use std::path::Path;

use anyhow::{Error, Result};

use crate::agent::{
    bat::Args,
    bve::{MapFile, MapPath},
    jww::{self, JwcTemp},
};

use super::relative::{Polyline, Relative_};

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
    fn make_map_file(&self) -> Result<MapPath> {
        // トラック名と図形を読み取る
        let mut temp_file = JwcTemp::open(self.args.temp_path()?)?;
        let _temp_0_file = JwcTemp::open(self.args.temp_0_path()?)?;
        let mut temp_x_file = JwcTemp::open(self.args.temp_x_path()?)?;
        let _track_name = temp_x_file.track_name();
        // let track_0 = temp_0_file.read_polyline()?;
        // let track_x = temp_x_file.read_polyline()?;

        // 始点を読み込む

        // 相対座標を計算する
        // let _ = Relative_::between(&track_0, &track_x)?;

        // マップファイルに書き込む
        let map_path = MapPath::build(self.args.map_name(), || temp_file.project_dir())?;
        let _map_file = MapFile::create(&map_path)?;
        // map_file.write_track(track_name, &relative)?;

        Ok(map_path)
    }

    /// 成功メッセージをJWC_TEMPファイルに出力する。
    fn show_map_path(&self, path: &(impl AsRef<Path> + ?Sized)) -> Result<()> {
        self.create_temp_file()?.notice(format!(
            "{} を作成しました",
            path.as_ref().to_string_lossy()
        ))
    }

    /// エラーをJWC_TEMPファイルに出力する。
    fn show_err(&self, e: &Error) -> Result<()> {
        self.create_temp_file()?.error(e)
    }

    /// JWC_TEMP.TXTを作成する。
    fn create_temp_file(&self) -> Result<jww::Write> {
        JwcTemp::create(self.args.temp_path()?)
    }
}

impl jww::Cache {
    fn read_polyline(&self) -> Result<Polyline> {
        todo!()
    }
}

trait WriteRelative {
    fn write(&self, relative: &Relative_) -> Result<()>;
}
