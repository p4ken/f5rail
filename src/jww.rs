//! プライマリアダプタ

use anyhow::{bail, Result};
use std::ffi::{OsStr, OsString};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::{fmt::Display, fs::File};

use crate::bve::MapWriter;
use crate::cad::{Point, Polyline};
use crate::cg::{Anchor0, Track};
use crate::track;
use crate::unit::Meter;

// エントリーポイント
pub fn f5rail(args: impl IntoIterator<Item = std::ffi::OsString>) -> std::io::Result<()> {
    PlugIn::new(args.into_iter().collect()).run()
}

#[derive(derive_more::Constructor)]
pub struct PlugIn {
    args: Args,
}
impl PlugIn {
    pub fn run(&self) -> io::Result<()> {
        let result = self.routing();
        self.show_result(result)
    }
    fn routing(&self) -> Result<String> {
        if let Some(_diminish) = self.args.transition() {
            self.draw_transition()
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
    fn show_result(&self, result: Result<String>) -> io::Result<()> {
        let jwc = JwcWriter::new(File::create(self.args.jwc_path())?);
        match result {
            Ok(s) => jwc.write_ok(s),
            Err(e) => jwc.write_err(e),
        }
    }
    fn draw_transition(&self) -> Result<String> {
        // args -> JwcTemp, App
        // 〜〜を描画しました
        todo!()
    }
    fn check_track(&self, number: &str) -> Result<String> {
        Ok("todo".to_owned())
    }
    fn export_track(&self) -> Result<String> {
        // ひたすらDI
        // ファイル操作をここに集約する
        // args -> JwcTemp, App
        let jwc = JwcReader::new(File::open(self.args.jwc_path())?);
        let jwc_0 = JwcReader::new(File::open(self.args.jwc_0_path())?);
        let jwc_x = JwcReader::new(File::open(self.args.jwc_x_path())?);
        // temp0 -> BveFile

        // エラー表示文字列とかはappに集約したい気もする。
        // jwcファイルのtraitオブジェクトでもDIは可能。
        // 逆にappがエラーを返さないならこのままがいい。

        let point_0 = Point::new(Meter(0.0), Meter(0.0)); // tmp
                                                          // TODO: point_0がpolylinesの上にあることをチェック、point_0よりも手前をカット、point_0から近い順にソート、連続性チェック
        let polylines = (jwc_0.polyline()?, jwc_x.polyline()?);
        let anchor_0 = jwc_0.anchor_0()?;

        // ドメインにはargsの値またはクロージャまたはtraitのみを渡す
        let app = track::App::new(polylines, point_0, anchor_0);
        let track = app.calculate_track();

        let track_name = "track1";
        let map_path = "map.txt";
        let mut map = MapWriter::new(File::create(map_path)?);
        map.write_track(track_name, &track)?;

        jwc.close()?;
        jwc_0.close()?;
        jwc_x.close()?;
        map.close()?;

        Ok(format!("{}に出力しました", map_path))
    }
}

pub struct Args(Vec<OsString>);
impl Args {
    fn track(&self) -> Option<&str> {
        Some("export")
    }
    fn transition(&self) -> Option<&OsStr> {
        None
    }
    fn jwc_path(&self) -> &OsStr {
        OsStr::new("abc")
    }
    fn jwc_0_path(&self) -> &OsStr {
        OsStr::new("abc")
    }
    fn jwc_x_path(&self) -> &OsStr {
        OsStr::new("abc")
    }
}
impl FromIterator<OsString> for Args {
    fn from_iter<I: IntoIterator<Item = OsString>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

struct JwcReader<R: Read> {
    buf: BufReader<R>,
    // cache: JwcFormat
}
impl<R: Read> JwcReader<R> {
    pub fn new(inner: R) -> Self {
        let buf = BufReader::new(inner);
        Self { buf }
    }
    pub fn track_name(&self) -> () {
        // ここでCP932->UTF-8の変換が必要
    }
    fn polyline(&self) -> Result<Polyline> {
        // todo
        Ok(Polyline::new(vec![]))
    }
    /// 始点のBVE距離程
    fn anchor_0(&self) -> Result<Anchor0> {
        // todo
        Ok(Anchor0::new(Meter(0.0)))
    }
}
impl JwcReader<File> {
    fn close(self) -> std::io::Result<()> {
        self.buf.into_inner().sync_all()
    }
}
// impl From<&OsStr> for crate::transition_::Diminish {}
pub struct JwcWriter<W: Write> {
    buf: BufWriter<W>,
}

impl<W: Write> JwcWriter<W> {
    fn new(writer: W) -> Self {
        let writer = BufWriter::new(writer);
        Self { buf: writer }
    }
    pub fn write_err(&self, e: impl Display) -> io::Result<()> {
        Ok(())
    }
    pub fn write_ok(&self, info: impl Display) -> io::Result<()> {
        Ok(())
    }
    pub fn write_track(&self, name: &str, track: Track) -> io::Result<()> {
        Ok(())
    }
}
impl JwcWriter<File> {
    fn close(self) -> std::io::Result<()> {
        self.buf.into_inner()?.sync_all()
    }
}

struct JwcFormat;
