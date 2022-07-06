mod agent;
mod factory;
mod track;
mod transition;

use std::{ffi::OsStr};

use anyhow::Result;

use agent::{bat::Args};
use factory::App;


/// 配線する
pub fn layout(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let args = Args::parse(args)?;

    match App::new(&args)? {
        App::Transition(transition) => transition.draw(),
        App::Track(track) => track.export(),
    }
}

pub fn f5rail(args: impl IntoIterator<Item = std::ffi::OsString>) -> std::io::Result<()> {
    jww::PlugIn::new(args.into_iter().collect()).run()
}

// プライマリアダプタ
mod jww {
    use anyhow::{bail, Result};
    use std::ffi::{OsStr, OsString};
    use std::io::{self, BufReader, BufWriter, Read, Write};
    use std::{fmt::Display, fs::File};

    use crate::bve::MapWriter;
    use crate::cad::{Point, Polyline};
    use crate::cg::{Anchor0, Track};
    use crate::track_;
    use crate::unit::Meter;

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
            let app = track_::App::new(polylines, point_0, anchor_0);
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
}

// セカンダリアダプタ
mod bve {
    use std::{
        fs::File,
        io::{self, BufWriter, Write},
    };

    use crate::cg::Track;

    pub struct MapWriter<W: Write> {
        buf: BufWriter<W>,
    }
    impl<W: Write> MapWriter<W> {
        pub fn new(write: W) -> Self {
            let buf = BufWriter::new(write);
            Self { buf }
        }
        pub fn write_track(&mut self, name: &str, track: &Track) -> io::Result<()> {
            // todo
            Ok(())
        }
    }
    impl MapWriter<File> {
        pub fn close(self) -> io::Result<()> {
            self.buf.into_inner()?.sync_all()
        }
    }

    #[cfg(test)]
    mod tests{
        #[test]
        fn write_track() {
            assert!(false)
        }
    }
}

// アプリケーション ひたすら移譲して薄くする
mod track_ {
    use derive_more::Constructor;

    use crate::{
        cad::{self, Line, Point, Polyline},
        cg::{self, Anchor, Anchor0},
        unit::Meter,
    };

    #[derive(Constructor)]
    pub struct App {
        polylines: (cad::Polyline, cad::Polyline),
        point_0: cad::Point,
        anchor_0: cg::Anchor0,
    }
    impl App {
        pub fn calculate_track(&self) -> cg::Track {
            // XY座標(cad) -> 相対座標(cg)
            // ここが肝である
            // point_0はpolylines.0の上にあって、anchor_0と同じ位置にある。

            // 自線と他線の線分始点ごとに
            // - anchor_0からの道のりを計算
            // - 自線から他線までの間隔を計算
            // - 相対半径を計算
            // 自線の線分始点：その弧長・半径、他線と交差する垂線の長さ、その他線の半径
            // - 垂線が他線と交差しなければ、スキップする。
            // 他線の線分始点：その半径、自線と交差する垂線の長さ、その自線の弧長・半径
            // - 垂線が自線と交差しなければ、スキップする。
            // 終点は半径なし
            // 交差する相手が直線：点と最も近い直線上の点
            // 垂線と曲線：

            cg::Track(vec![])
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn calculate_track_test() {
            let polyline_0 =
                Polyline::new(vec![Line::Straight(point(0.0, 0.0), point(0.0, 100.0))]);
            let polyline_x =
                Polyline::new(vec![Line::Straight(point(4.0, 0.0), point(4.0, 100.0))]);

            let point_0 =  point(0.0, 0.0);
            let anchor_0 = Anchor0::new(Meter(0.0));
            let sut = App::new((polyline_0, polyline_x), point_0, anchor_0);
            let track = sut.calculate_track();
            assert_eq!(track.0.len(), 3);
        }
        fn point(x: f64, y: f64) -> Point {
            Point::new(Meter(x), Meter(y))
        }
    }
}
mod transition_ {
    // ここは今は書き直したくない
}

// ドメイン
mod cg {
    use derive_more::Constructor;

    use super::unit::*;
    pub struct Track(pub Vec<Anchor>);
    impl Track {
        // 連続線はcadのXY座標系になっている...
        // pub fn calculate(/* polylines: Polyline */) -> Self {
        //     Self(vec![])
        // }
    }
    pub struct Anchor {
        z: Meter, // 距離程
        x: Meter, // 自線との間隔
        r: f64,   // 相対半径
    }
    // 自線の線分の端点または、他線の線分の端点に最も近い自線上の点。
    #[derive(Constructor)]
    pub struct Anchor0(Meter); // 距離程Z
}
mod cad {
    use derive_more::Constructor;

    use super::unit::*;
    #[derive(Constructor)]
    pub struct Polyline(Vec<Line>); // 連続線。内部で連続線判定
    pub enum Line {
        Straight(Point, Point),
        Curve(Point, Radius, Angle, Angle),
    }
    // 線分の端点(連続線上の点)もPointを使っているが、特化した型を作りたい
    #[derive(Constructor)]
    pub struct Point(Meter, Meter);
    // impl From<(Meter, Radian)> for Point {} // Derefでもいいかも
    pub struct Radius(Meter);
    pub struct Angle(Radian);
}
mod unit {
    pub struct Meter(pub f64);
    pub struct Radian(pub f64);
}
