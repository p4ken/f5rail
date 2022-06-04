use crate::track::polyline::Edge;

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
    fn start(&self) -> Option<(f64, f64)> {
        let pair = match self {
            Self::Straight([x, y, ..]) => (*x, *y),
            _ => return None,
        };
        Some(pair)
    }

    fn end(&self) -> Option<(f64, f64)> {
        todo!()
    }
}
