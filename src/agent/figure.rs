use crate::{track::polyline::{Edge, Point}, transition::unit::XY};

/// JWW_TEMPファイルの図形データ
#[derive(Debug, PartialEq)]
pub enum Figure {
    /// 単線
    Straight([f64; 4]),

    /// 円弧
    Arc([f64; 5]),

    /// 円・楕円・楕円弧
    Ellipse,

    /// ブロック図形
    Block,

    /// ソリッド（線形・円周）
    Solid,
}

impl Edge for Figure {
    fn start(&self) -> Option<Point> {
        let (x, y) = match self {
            Self::Straight([x, y, ..]) => (*x, *y),
            // Self::Arc([cx, cy, r, a0, _]) => (xy + ),
            _ => return None,
        };
        Some(Point(x, y))
    }

    fn end(&self) -> Option<Point> {
        todo!()
    }
}
