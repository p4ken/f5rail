use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Not},
};

/// 逓減
#[derive(Debug)]
pub enum Diminish {
    /// サイン半波長逓減
    Sine,

    /// 直線逓減（クロソイド曲線）
    Linear,
}

impl Diminish {
    /// 曲率を計算する。
    pub fn k(&self, tcl: f64, s: f64, k0: Curvature, k1: Curvature) -> Curvature {
        Curvature(0.)
    }
}

/// 半径 (m)
///
/// `None` の場合は直線。
#[derive(Debug)]
pub struct Radius(pub Option<f64>);

impl From<Curvature> for Radius {
    /// 曲率から半径に変換する。
    fn from(k: Curvature) -> Self {
        Radius(k.is_straight().not().then(|| k.0.recip()))
    }
}

/// 曲率 (1/m)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Curvature(pub f64);

/// 直線
const STRAIGHT: Curvature = Curvature(0.);

impl Curvature {
    /// 直線を判定する。
    pub fn is_straight(&self) -> bool {
        self.0 == STRAIGHT.0
    }

    /// 弧長 `len` から中心角を計算する。
    pub fn angle(&self, len: f64) -> Radian {
        Radian(self.0 * len)
    }
}

impl From<Radius> for Curvature {
    /// 曲率に変換する。
    fn from(r: Radius) -> Self {
        r.0.map_or(STRAIGHT, |r| Curvature(r.recip()))
    }
}

/// 角度 (度)
#[derive(Debug, Copy, Clone)]
pub struct Degree(pub f64);

impl Add for Degree {
    type Output = Self;

    /// 加算演算子
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<Radian> for Degree {
    /// 度に変換する。
    fn from(rad: Radian) -> Self {
        Degree(rad.0.to_degrees())
    }
}

/// 角度 (ラジアン)
#[derive(Debug, Copy, Clone)]
pub struct Radian(f64);

/// XY座標上の点
#[derive(Debug, Copy, Clone)]
pub struct Point(pub f64, pub f64);

impl Add<(f64, Degree)> for &Point {
    type Output = Point;

    /// 長さと向きを指定して移動する
    fn add(self, rhs: (f64, Degree)) -> Self::Output {
        todo!()
    }
}
