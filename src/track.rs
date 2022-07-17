//! アプリケーションサービス

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
    // なるべく移譲して薄くする
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
        let polyline_0 = Polyline::new(vec![Line::Straight(point(0.0, 0.0), point(0.0, 100.0))]);
        let polyline_x = Polyline::new(vec![Line::Straight(point(4.0, 0.0), point(4.0, 100.0))]);

        let point_0 = point(0.0, 0.0);
        let anchor_0 = Anchor0::new(Meter(0.0));
        let sut = App::new((polyline_0, polyline_x), point_0, anchor_0);
        let track = sut.calculate_track();
        // assert_eq!(track.0.len(), 3);
    }

    fn point(x: f64, y: f64) -> Point {
        Point::new(Meter(x), Meter(y))
    }
}
