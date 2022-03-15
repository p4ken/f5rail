use std::f64::consts::FRAC_PI_2;

use super::formula::{Curvature, Point, Radian, Radius};

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
        match Radius::from(k).0 {
            None => Self::Straight(p0, &p0 + (len, t0)),
            Some(r) => Self::Curve(
                &p0 + (r, t0 + Radian(FRAC_PI_2)),
                r.abs(),
                t0,
                t0 + k.angle(len),
            ),
        }
    }

    /// 終点座標
    pub fn p1(&self) -> Point {
        match self {
            Self::Straight(_, p1) => *p1,
            Self::Curve(c, r, _, a1) => c + (*r, *a1),
        }
    }

    /// 終点の進行方向（接線の向き）
    pub fn t1(&self, t0: Radian) -> Radian {
        match self {
            Self::Straight(..) => t0,
            Self::Curve(_, _, a0, a1) => {
                // 左カーブ→引く、右カーブ→足す
                t0 - (*a1 - *a0)
            }
        }
    }
}
