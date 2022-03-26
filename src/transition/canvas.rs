use derive_more::{Deref, IntoIterator};

use super::{
    curve::{Central, Curvature, Radius, Subtension, Tangential},
    unit::{Rad, Vector},
};
use std::{f64::consts::PI, ops::Add};

/// 緩和曲線
///
/// 複数の線で表現される。
#[derive(Deref, IntoIterator)]
pub struct Spiral(Vec<Stroke>);

impl FromIterator<Stroke> for Spiral {
    /// 線から緩和曲線を作成する。
    fn from_iter<T: IntoIterator<Item = Stroke>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

/// 一画の線
#[derive(Debug, Clone, Copy)]
// pub enum Stroke {
/// 円弧
///
/// 始点の接線、弧長、半径で表現される。
// Curve(Tangent, Subtension, Radius),
// /// 中心点、半径、始角、終角で表現される。
// Arc(Point, f64, Radian, Radian),

/// 直線
///
/// 始点の接線、長さで表現される。
// Straight(Tangent, Subtension),
// /// 始点と終点で表現される。
// Straight(Point, Point),
pub struct Stroke {
    /// 曲率
    k: Curvature,

    /// 弧長
    len: Subtension,

    /// 始点の座標
    p0: Point,

    /// 始点の接線方向
    t0: Tangential,
}

impl Stroke {
    /// コンストラクタ
    pub fn new(k: Curvature, len: Subtension, p0: Point, t0: Tangential) -> Self {
        Self { k, len, p0, t0 }
    }

    /// 曲線半径
    pub fn r(&self) -> Option<Radius> {
        self.k.r()
    }

    /// 始点の中心角
    pub fn a0(&self) -> Central {
        self.t0.to_central(self.k)
    }

    /// 終点の中心角
    pub fn a1(&self) -> Central {
        self.a0() + self.k.a(self.len)
    }

    /// 中心点
    ///
    /// 直線の場合は `None`
    pub fn center(&self) -> Option<Point> {
        self.r().map(|r| self.p0 + (r, self.a0() + PI.into()))
    }

    /// 始点
    pub fn p0(&self) -> Point {
        self.p0
    }

    /// 終点
    pub fn p1(&self) -> Point {
        match self.center().zip(self.r()) {
            Some((c, r)) => c + (r, self.a1()),
            None => self.p0 + (self.len, self.t0),
        }
    }

    /// 終点の接線方向
    pub fn t1(&self) -> Tangential {
        self.t0 + self.k.a(self.len).rad().into()
    }
}

/// 座標 (x, y)
#[derive(Debug, Copy, Clone)]
pub struct Point(pub f64, pub f64);

impl Vector for Point {
    fn x(self) -> f64 {
        self.0
    }
    fn y(self) -> f64 {
        self.1
    }
}

impl<T: Vector + Copy> Add<T> for Point {
    type Output = Self;

    /// 足し算
    fn add(self, rhs: T) -> Self::Output {
        Self(self.x() + rhs.x(), self.y() + rhs.y())
    }
}
