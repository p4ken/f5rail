use super::{
    curve::{Arc, Curvature, Radian, Radius, Subtension},
    unit::Meter,
};
use std::{
    f64::consts::{FRAC_PI_2, PI},
    ops::{Add, Index},
    slice::SliceIndex,
};

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

impl<I: SliceIndex<[Stroke]>> Index<I> for Spiral {
    type Output = I::Output;

    /// 添字アクセス
    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

/// 一画の線
#[derive(Debug, Clone, Copy)]
// pub enum Stroke {
/// 円弧
///
/// 始点の接線、弧長、半径で表現される。
// Curve(Tangent, Subtension, Radius),
//     /// 中心点、半径、始角、終角で表現される。
//     Arc(Point, f64, Radian, Radian),

/// 直線
///
/// 始点の接線、長さで表現される。
//     Straight(Tangent, Subtension),
//     /// 始点と終点で表現される。
//     Straight(Point, Point),
pub struct Stroke {
    /// 円弧
    arc: Arc,

    /// 始点の接線
    t0: Tangent,
}

impl Stroke {
    /// コンストラクタ
    pub fn new(arc: Arc, t0: Tangent) -> Self {
        Self { arc, t0 }
    }

    /// 直線なら `true`
    pub fn is_straight(&self) -> bool {
        self.arc.is_straight()
    }

    /// 曲線半径
    pub fn r(&self) -> Option<Radius> {
        self.arc.r()
    }

    /// 始点の中心角
    pub fn a0(&self) -> Radian {
        match self.arc.is_right() {
            true => self.t0.a + FRAC_PI_2.into(),
            false => self.t0.a - FRAC_PI_2.into(),
        }
    }

    /// 終点の中心角
    pub fn a1(&self) -> Radian {
        match self.arc.is_right() {
            true => self.a0() - self.arc.angle(),
            false => self.a0() + self.arc.angle(),
        }
    }

    /// 中心点
    ///
    /// 直線の場合は `None`
    pub fn center(&self) -> Option<Point> {
        self.r().map(|r| self.t0.p + (r, self.a0() + PI.into()))
    }

    /// 長さ
    pub fn len(&self) -> Subtension {
        self.arc.len()
    }

    /// 始点
    pub fn p0(&self) -> Point {
        self.t0.p
    }

    /// 終点
    pub fn p1(&self) -> Point {
        match self.center().zip(self.r()) {
            Some((c, r)) => c + (r, self.a1()),
            None => self.p0() + (self.arc.len(), self.t0.a),
        }
    }

    /// 終点の接線
    pub fn t1(&self) -> Tangent {
        Tangent {
            p: self.p1(),
            a: self.a1(),
        }
    }
}

/// 接線
#[derive(Debug, Clone, Copy)]
pub struct Tangent {
    /// 接点
    p: Point,

    /// 方向
    a: Radian,
}

impl Tangent {
    /// コンストラクタ
    pub fn new(p: Point, a: Radian) -> Self {
        Self { p, a }
    }
}

/// ベクトル
///
/// `Polar` を実装すれば、自動的に実装される。
pub trait Vector {
    fn x(self) -> f64;
    fn y(self) -> f64;
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

/// 極座標 (r, θ)
pub trait Polar {
    /// 半径
    fn r(self) -> f64;

    /// 中心角
    fn a(self) -> Radian;
}

impl<T: Polar + Copy> Vector for T {
    fn x(self) -> f64 {
        self.r() * self.a().cos()
    }
    fn y(self) -> f64 {
        self.r() * self.a().sin()
    }
}

impl<T: Meter> Polar for (T, Radian) {
    fn r(self) -> f64 {
        self.0.meter()
    }

    fn a(self) -> Radian {
        self.1
    }
}
