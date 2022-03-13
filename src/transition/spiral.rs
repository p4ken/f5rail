use std::f64::consts::FRAC_2_PI;

use super::formula::{Curvature, Point, Radian};

/// 緩和曲線
///
/// 線分の集合で表現される。
pub struct Spiral(pub Vec<Line>);

impl FromIterator<Line> for Spiral {
    /// 線分の集合から緩和曲線を作成する。
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
    Curve(Point, f64, Radian, Radian),

    /// 直線
    ///
    /// 始点と終点で表現される。
    Straight(Point, Point),
}

impl Line {
    /// コンストラクタ
    pub fn new(p0: Point, t0: Radian, len: f64, k: Curvature) -> Self {
        if k.is_straight() {
            Self::Straight(p0, &p0 + (len, t0))
        } else {
            let r = k.0.recip();
            Self::Curve(
                &p0 + (r, t0 + Radian(FRAC_2_PI)),
                r.abs(),
                t0,
                t0 + k.angle(len),
            )
        }
    }
}
