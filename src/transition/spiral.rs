use std::{f64::consts::FRAC_PI_2, ops::Add};

use super::curve::{Curvature, Radian, Radius};

/// 緩和曲線
///
/// 複数の線で表現される。
pub struct Spiral(pub Vec<Stroke>);

impl FromIterator<Stroke> for Spiral {
    /// 線から緩和曲線を作成する。
    fn from_iter<T: IntoIterator<Item = Stroke>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

/// 一画の線
#[derive(Debug)]
pub enum Stroke {
    /// 円弧
    ///
    /// 中心点、半径、始角、終角で表現される。
    Arc(Point, f64, Radian, Radian),

    /// 直線
    ///
    /// 始点と終点で表現される。
    Straight(Point, Point),
}

impl Stroke {
    /// コンストラクタ
    pub fn new(p0: &Point, t0: Radian, len: f64, k: Curvature) -> Self {
        match Radius::from(k).0 {
            None => Self::Straight(*p0, p0 + &Polar(len, t0)),
            Some(r) => Self::Arc(
                p0 + &Polar(r, t0 + Radian(FRAC_PI_2)),
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
            Self::Arc(c, r, _, a1) => c + &Polar(*r, *a1),
        }
    }

    /// 終点の進行方向（接線の向き）
    pub fn t1(&self, t0: Radian) -> Radian {
        match self {
            Self::Straight(..) => t0,
            Self::Arc(_, _, a0, a1) => {
                // 左カーブ→引く、右カーブ→足す
                t0 - (*a1 - *a0)
            }
        }
    }
}

/// 直交座標 (x, y)
#[derive(Debug, Copy, Clone)]
pub struct Point(pub f64, pub f64);

impl Add for &Point {
    type Output = Point;

    /// 足し算
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<&Polar> for &Point {
    type Output = Point;

    /// 足し算
    fn add(self, rhs: &Polar) -> Self::Output {
        self + &Point::from(rhs)
    }
}

impl From<&Polar> for Point {
    /// 極座標から変換する。
    fn from(polar: &Polar) -> Self {
        let x = polar.0 * polar.1.cos();
        let y = polar.0 * polar.1.sin();
        Self(x, y)
    }
}

/// 極座標 (r, θ)
#[derive(Debug, Copy, Clone)]
pub struct Polar(pub f64, pub Radian);
