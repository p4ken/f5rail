use super::formula::{Curvature, Degree, Point, Radius};

/// 緩和曲線
///
/// 線分の集合で表現される。
pub struct Spiral(pub Vec<Line>);

impl FromIterator<Line> for Spiral {
    /// 線分を集める。
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

/// 線分
#[derive(Debug)]
pub enum Line {
    /// 円弧
    ///
    /// 中心点、半径、始角、終角で表現される。
    Curve(Point, f64, Degree, Degree),

    /// 直線
    ///
    /// 始点と終点で表現される。
    Straight(Point, Point),
}

impl Line {
    pub fn new(p0: Point, a0: Degree, len: f64, k: Curvature) -> Self {
        if k.is_straight() {
            // p0に対して左右どちらの方向か分からない・・・
            Self::Straight(p0, &p0 + (len, a0 + Degree(90.0)))
        } else {
            todo!()
            // Self::Curve {
            //     c: p0, // 三角関数？
            //     r: k.into(),
            //     a0,
            //     a1: a0 + k.angle(len).into(),
            // }
        }
    }
}
