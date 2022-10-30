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

use std::{
    ffi::{OsStr, OsString},
    fs::File,
};

use anyhow::{bail, Result};

/// 配線する
pub fn layout(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let args = agent::bat::Args::parse(args)?;

    match factory::App::new(&args)? {
        factory::App::Transition(transition) => transition.draw(),
        factory::App::Track(track) => track.export(),
    }
}

/// 外部変形
#[derive(derive_more::Constructor)]
pub struct Plugin {
    // TODO: 個別の設定項目の値を保持するか、Argsをインタフェースにしないと、CLIと密接になってしまう
    args: jww::Args,
}

impl Plugin {
    /// コマンドラインインタフェース
    pub fn cli(args: impl IntoIterator<Item = impl Into<OsString>>) -> Result<()> {
        let args = args.into_iter().collect::<jww::Args>();
        let plugin = Self::new(args);
        plugin.show_result(plugin.routing())
    }
    fn routing(&self) -> Result<String> {
        if let Some(diminish) = self.args.transition() {
            self.draw_transition(&diminish)
        } else if let Some(number) = self.args.track() {
            match number {
                "0" | "x" => self.check_track(number),
                "export" => self.export_track(),
                _ => bail!("trackの値が無効"),
            }
        } else {
            bail!("機能の指定なし")
        }
    }
    fn draw_transition(&self, &diminish: &crate::transition::curve::Diminish) -> Result<String> {
        // args -> JwcTemp, App
        let k0 = self.args.curvature(0)?;
        let k1 = self.args.curvature(1)?;
        let l0 = self.args.distance0();
        let tcl = self.args.tcl()?;
        let p0 = self.args.p0();
        let t0 = self.args.t0();
        let param = crate::transition::param::Param::new(diminish, k0, k1, l0, tcl, p0, t0);
        let spiral = crate::transition::plot(&param);

        // 緩和曲線 `spiral` を出力する
        let mut jwc_temp = jww::TempWriter::create(self.args.jwc_path()?)?;
        jwc_temp.write_spiral(spiral)?;
        let diminish = match diminish {
            crate::transition::curve::Diminish::Sine => "サイン半波長逓減曲線",
            crate::transition::curve::Diminish::Linear => "直線逓減（クロソイド）",
        };
        Ok(format!("{}を描画しました", diminish))
    }
    fn check_track(&self, number: &str) -> Result<String> {
        Ok("todo".to_owned())
    }
    fn export_track(&self) -> Result<String> {
        // ひたすらDI
        // ファイル操作をここに集約する
        // args -> JwcTemp, App
        let jwc_path = self.args.jwc_path()?;
        let jwc = jww::TempReader::open(jwc_path)?;
        let jwc_0_path = self.args.jwc_0_path()?;
        let jwc_0 = jww::TempReader::open(jwc_0_path)?;
        let jwc_x_path = self.args.jwc_x_path()?;
        let jwc_x = jww::TempReader::open(jwc_x_path)?;
        // temp0 -> BveFile

        // エラー表示文字列とかはappに集約したい気もする。
        // jwcファイルのtraitオブジェクトでもDIは可能。
        // 逆にappがエラーを返さないならこのままがいい。

        let polylines = (jwc_0.polyline()?, jwc_x.polyline()?);
        // TODO: point_0がpolylinesの上にあることをチェック、point_0よりも手前をカット、point_0から近い順にソート、連続性チェック
        let point_0 = cad::Point::new(unit::Meter(0.0), unit::Meter(0.0)); // tmp
        let anchor_0 = jwc_0.anchor_0()?;

        // ドメインにはargsの値またはクロージャまたはtraitのみを渡す
        let app = track::App::new(polylines, point_0, anchor_0);
        let track = app.calculate_track();

        let track_name = "track1";
        let map_path = agent::bve::MapPath::build(self.args.map_name(), || jwc_0.project_dir())?;
        let mut map = bve::MapWriter::new(File::create(&map_path)?);
        map.write_track(track_name, &track)?;

        jwc.close()?;
        jwc_0.close()?;
        jwc_x.close()?;
        map.close()?;

        Ok(format!("{}に出力しました", map_path.to_string_lossy()))
    }
    fn show_result(&self, result: Result<String>) -> Result<()> {
        let jwc = jww::TempWriter::create(self.args.jwc_path()?)?;
        match result {
            Ok(s) => jwc.write_ok(s),
            Err(e) => jwc.write_err(e),
        }
    }
}
