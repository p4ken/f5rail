//! プライマリアダプタ

use anyhow::{bail, ensure, Context, Result};
use encoding_rs::SHIFT_JIS;
use std::ffi::{OsStr, OsString};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::{fmt::Display, fs::File};

use crate::bve;
use crate::cad::{Point, Polyline};
use crate::cg::{Anchor0, Track};
use crate::track;
use crate::transition::canvas::Spiral;
use crate::unit::Meter;

// エントリーポイント
pub fn f5rail(args: impl IntoIterator<Item = std::ffi::OsString>) -> Result<()> {
    PlugIn::new(args.into_iter().collect()).run()
}

#[derive(derive_more::Constructor)]
pub struct PlugIn {
    args: Args,
}
impl PlugIn {
    pub fn run(&self) -> Result<()> {
        let result = self.routing();
        self.show_result(result)
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
    fn show_result(&self, result: Result<String>) -> Result<()> {
        let jwc = TempWriter::create(self.args.jwc_path())?;
        match result {
            Ok(s) => jwc.write_ok(s),
            Err(e) => jwc.write_err(e),
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
        let param = crate::transition::param::Param {
            diminish,
            k0,
            k1,
            l0,
            tcl,
            p0,
            t0,
        };
        let spiral = crate::transition::plot(&param);

        // 緩和曲線 `spiral` を出力する
        let mut jwc_temp = TempWriter::create(self.args.jwc_path())?;
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
        let jwc = TempReader::open(self.args.jwc_path())?;
        let jwc_0 = TempReader::open(self.args.jwc_0_path())?;
        let jwc_x = TempReader::open(self.args.jwc_x_path())?;
        // temp0 -> BveFile

        // エラー表示文字列とかはappに集約したい気もする。
        // jwcファイルのtraitオブジェクトでもDIは可能。
        // 逆にappがエラーを返さないならこのままがいい。

        let polylines = (jwc_0.polyline()?, jwc_x.polyline()?);
        // TODO: point_0がpolylinesの上にあることをチェック、point_0よりも手前をカット、point_0から近い順にソート、連続性チェック
        let point_0 = Point::new(Meter(0.0), Meter(0.0)); // tmp
        let anchor_0 = jwc_0.anchor_0()?;

        // ドメインにはargsの値またはクロージャまたはtraitのみを渡す
        let app = track::App::new(polylines, point_0, anchor_0);
        let track = app.calculate_track();

        let track_name = "track1";
        let map_path = "map.txt";
        let mut map = bve::MapWriter::new(File::create(map_path)?);
        map.write_track(track_name, &track)?;

        jwc.close()?;
        jwc_0.close()?;
        jwc_x.close()?;
        map.close()?;

        Ok(format!("{}に出力しました", map_path))
    }
}

pub struct Args(Vec<OsString>);
impl FromIterator<OsString> for Args {
    fn from_iter<I: IntoIterator<Item = OsString>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl Args {
    fn track(&self) -> Option<&str> {
        Some("export")
    }
    fn transition(&self) -> Option<crate::transition::curve::Diminish> {
        let transition = self
            .0
            .iter()
            .filter_map(|x| x.to_str())
            .find(|x| x.starts_with("/TRANSITION"));
        match transition {
            Some(x) => match x {
                "/TRANSITION:1" => Some(crate::transition::curve::Diminish::Sine),
                "/TRANSITION:2" => Some(crate::transition::curve::Diminish::Linear),
                _ => None,
            },
            None => None,
        }
    }
    fn curvature(&self, i: i32) -> Result<crate::transition::curve::Curvature> {
        let prefix = format!("/R{}:", i);
        let radius = self
            .0
            .iter()
            .filter_map(|x| x.to_str())
            .find(|x| x.starts_with(&prefix));
        let radius = match radius {
            Some(radius) => radius.trim_start_matches(&prefix),
            None => return Ok(crate::transition::curve::STRAIGHT),
        };
        let r = match radius.parse::<f64>() {
            Err(_) | Ok(0.0) => bail!("R{}に{}を指定できません", i, radius),
            Ok(r) => crate::transition::curve::Radius(r),
        };
        Ok(crate::transition::curve::Curvature::from(r))
    }
    fn distance0(&self) -> crate::transition::distance::Distance<f64> {
        crate::transition::distance::Distance::from(0.0)
    }
    fn tcl(&self) -> Result<crate::transition::curve::Subtension> {
        const PREFIX: &str = "/TCL:";
        let tcl = self
            .0
            .iter()
            .filter_map(|x| x.to_str())
            .find(|x| x.starts_with(PREFIX));
        let tcl = match tcl {
            Some(tcl) => tcl.trim_start_matches(PREFIX),
            None => bail!("TCLを指定してください"),
        };
        match tcl.parse::<f64>() {
            Ok(tcl) => Ok(crate::transition::curve::Subtension::from(tcl)),
            Err(_) => bail!("TCLに{}を指定できません", tcl),
        }
    }
    fn p0(&self) -> crate::transition::canvas::Point {
        crate::transition::canvas::Point::from((0.0, 0.0))
    }
    fn t0(&self) -> crate::transition::curve::Tangential {
        crate::transition::curve::Tangential::from(0.0)
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

struct TempReader<R: Read> {
    buf: BufReader<R>,
    // cache: JwcFormat
}
impl<R: Read> TempReader<R> {
    fn new(inner: R) -> Self {
        let buf = BufReader::new(inner);
        Self { buf }
    }
    fn track_name(&self) -> () {
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
impl TempReader<File> {
    fn open(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self::new(file))
    }
    fn close(self) -> std::io::Result<()> {
        self.buf.into_inner().sync_all()
    }
}

// impl From<&OsStr> for crate::transition_::Diminish {}
pub struct TempWriter<W: Write> {
    buf: BufWriter<W>,
}
impl<W: Write> TempWriter<W> {
    fn new(writer: W) -> Self {
        let writer = BufWriter::new(writer);
        Self { buf: writer }
    }
    fn write_err(&self, e: impl Display) -> Result<()> {
        Ok(())
    }
    fn write_ok(&self, info: impl Display) -> Result<()> {
        Ok(())
    }
    fn write_spiral(&mut self, spiral: Spiral) -> Result<()> {
        for stroke in spiral.iter() {
            match stroke.center().zip(
                stroke
                    .r()
                    .filter(|r| crate::transition::unit::Meter::meter(r).abs() < 100_000.0),
            ) {
                Some((c, r)) => self.write_curve(&c, &r, &stroke.a0(), &stroke.a1())?,
                None => self.write_straight(&stroke.p0(), &stroke.p1())?,
            }
        }
        Ok(())
    }
    pub fn write_curve(
        &mut self,
        c: &impl crate::transition::unit::Vector,
        r: &impl crate::transition::unit::Meter,
        a0: &impl crate::transition::unit::Deg,
        a1: &impl crate::transition::unit::Deg,
    ) -> Result<()> {
        let (cx, cy) = (c.x(), c.y());
        let r = r.meter().abs();
        let (a0, a1) = (a0.deg(), a1.deg());
        let (a0, a1) = if a0 < a1 { (a0, a1) } else { (a1, a0) };
        self.puts(format!("ci {cx} {cy} {r} {a0} {a1}"))
    }
    pub fn write_straight(
        &mut self,
        p0: &impl crate::transition::unit::Vector,
        p1: &impl crate::transition::unit::Vector,
    ) -> Result<()> {
        self.puts(format!("{} {} {} {}", p0.x(), p0.y(), p1.x(), p1.y()))
    }
    fn write_track(&self, name: &str, track: Track) -> Result<()> {
        Ok(())
    }
    fn puts<T: AsRef<str>>(&mut self, s: T) -> Result<()> {
        // TODO:
        // SHIFT_JISではなくCP932にしたほうがいい。
        // - https://crates.io/crates/codepage
        // - https://crates.io/search?q=windows%20encoding&sort=downloads
        let (sjis, _, _) = SHIFT_JIS.encode(s.as_ref());
        for bytes in [&sjis[..], b"\r\n"] {
            io::Write::write_all(&mut self.buf, bytes)
                .context("JWC_TEMP.TXTへの書き込みに失敗しました。")?;
        }
        Ok(())
    }
}
impl TempWriter<File> {
    fn create(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let file = File::create(path)?;
        Ok(Self::new(file))
    }
    fn close(self) -> std::io::Result<()> {
        self.buf.into_inner()?.sync_all()
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use anyhow::Result;
    use rstest::rstest;

    use crate::transition::{
        self,
        curve::{Curvature, Diminish, Radius, STRAIGHT},
    };

    use super::*;

    #[test]
    fn コマンドライン引数をパースできる() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/TRANSITION:1"),
            OsString::from("/R0:1.1"),
            OsString::from("/R1:2"),
            OsString::from("/TCL:3"),
            OsString::from("/TEMP:./JWC_TEMP.TXT"),
        ];
        let args = Args(args);
        assert_eq!(args.transition(), Some(Diminish::Sine));
        // assert_eq!(args.jwc_path(), "./JWC_TEMP.TXT");
        assert_eq!(args.curvature(0).ok(), Some(Radius(1.1).into()));
        assert_eq!(args.curvature(1).ok(), Some(Radius(2.0).into()));
        assert_eq!(args.tcl().ok(), Some(3.0.into()));
    }

    // #[test]
    // fn 緩和曲線の長さ以外は省略可能() {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //         OsString::from("/TCL:3"),
    //         OsString::from("/TEMP:./JWC_TEMP.TXT"),
    //     ];
    //     let args = Args::parse(args);
    //     let args = args.unwrap();
    //     let transition = args.unwrap_transition();
    //     assert_eq!(transition.0, "./JWC_TEMP.TXT");
    //     let param = transition.1.as_ref().unwrap();
    //     assert!(matches!(param.diminish, Diminish::Sine));
    //     assert!(param.k0.is_straight());
    //     assert!(param.k1.is_straight());
    //     assert_eq!(param.tcl, 3.0.into());
    // }

    // #[rstest]
    // #[case("/TRANSITION:0",Err(anyhow::anyhow!("緩和曲線関数に正しい値を入力してください")))]
    // #[case("/TRANSITION:2", Ok(Diminish::Linear))]
    // fn 緩和曲線関数をパースする(#[case] arg: &str, #[case] expected: Result<Diminish>) {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from(arg),
    //         OsString::from("/TCL:3"),
    //         OsString::from("/TEMP:./JWC_TEMP.TXT"),
    //     ];
    //     let (_, param) = Args::parse(args).unwrap().unwrap_transition();
    //     match (param, expected) {
    //         (Ok(param), Ok(_expected)) => assert!(matches!(param.diminish, _expected)),
    //         (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
    //         _ => panic!(),
    //     }
    // }

    // #[rstest]
    // #[case("", "TCLを指定してください")]
    // #[case("/TCL:0", "TCLに0より大きい値を入力してください")]
    // #[case("/TCL:-1", "TCLに0より大きい値を入力してください")]
    // fn 緩和曲線の長さのエラーチェック(#[case] arg: &str, #[case] err: &str) {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //         OsString::from(arg),
    //         OsString::from("/TEMP:./JWC_TEMP.TXT"),
    //     ];
    //     let args = Args::parse(args);
    //     let (_, param) = args.unwrap().unwrap_transition();
    //     let e = param.as_ref().unwrap_err();
    //     assert_eq!(e.to_string(), err);
    // }

    // #[rstest]
    // #[case("/R0:abc", "R0を数値で入力してください")]
    // #[case("/R1:abc", "R1を数値で入力してください")]
    // #[case("/R0:0", "R0に0を指定できません")]
    // #[case("/R1:0", "R1に0を指定できません")]
    // fn 緩和曲線の半径のエラーチェック(#[case] arg: &str, #[case] err: &str) {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //         OsString::from(arg),
    //         OsString::from("/TCL:3"),
    //         OsString::from("/TEMP:./JWC_TEMP.TXT"),
    //     ];
    //     let args = Args::parse(args);
    //     let args = args.unwrap();
    //     let transition = args.unwrap_transition();
    //     assert_eq!(transition.0, "./JWC_TEMP.TXT");
    //     let e = transition.1.as_ref().unwrap_err();
    //     assert_eq!(e.to_string(), err);
    // }

    // impl Args {
    //     fn unwrap_transition(&self) -> (String, Result<transition::Param>) {
    //         if let Ok(formula) = self.transition() {
    //             let file = self.temp_path().unwrap().into();
    //             let param = Param::parse(&formula, &self);
    //             (file, param)
    //         } else {
    //             panic!("This is not a transition.")
    //         }
    //     }
    // }

    // impl Curvature {
    //     fn is_straight(&self) -> bool {
    //         *self == STRAIGHT
    //     }
    // }
}
