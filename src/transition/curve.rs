use std::{
    f64::consts::PI,
    fmt::{Debug, Display},
    ops::{Add, Mul, Not, Sub},
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
        // 緩和曲線長全体に対する弧長の比率
        let x = s / tcl;

        // 曲率変化量全体に対する曲率の比率
        let y = match self {
            Diminish::Sine => ((x - 0.5) * PI).sin() / 2.0 + 0.5,
            Diminish::Linear => x,
        };

        // 曲率
        k0 + (k1 - k0) * y
    }
}

/// 曲率 (1/m)
///
/// 左カーブなら負の数。
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Curvature(pub f64);

impl Curvature {
    /// 直線
    const STRAIGHT: Curvature = Curvature(0.);

    /// 直線を判定する。
    pub fn is_straight(&self) -> bool {
        self.0 == Self::STRAIGHT.0
    }

    /// 弧長 `len` から中心角を計算する。
    pub fn angle(&self, len: f64) -> Radian {
        Radian(self.0 * len)
    }

    /// 半径
    pub fn r(&self) -> Radius {
        Radius::from(*self)
    }
}

impl From<Radius> for Curvature {
    /// 半径から変換する。
    fn from(r: Radius) -> Self {
        r.0.map_or(Self::STRAIGHT, |r| Curvature(r.recip()))
    }
}

impl Display for Curvature {
    /// 表示する。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Curvature {
    type Output = Self;

    /// 足し算
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Curvature {
    type Output = Self;

    /// 引き算
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Curvature {
    type Output = Self;

    /// 掛け算
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

/// 半径 (m)
///
/// `None` の場合は直線。
#[derive(Debug)]
pub struct Radius(pub Option<f64>);

impl From<Curvature> for Radius {
    /// 曲率から変換する。
    fn from(k: Curvature) -> Self {
        Radius(k.is_straight().not().then(|| k.0.recip()))
    }
}

impl Display for Radius {
    /// 表示する。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_or(0.0))
    }
}

/// 角度 (ラジアン)
#[derive(Debug, Copy, Clone)]
pub struct Radian(pub f64);

impl Radian {
    /// サイン
    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    /// コサイン
    pub fn cos(self) -> f64 {
        self.0.cos()
    }
}

impl Add for Radian {
    type Output = Self;

    /// 足し算
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Radian {
    type Output = Self;

    /// 引き算
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<Degree> for Radian {
    /// 度から変換する。
    fn from(rad: Degree) -> Self {
        Radian(rad.0.to_radians())
    }
}

/// 角度 (度)
#[derive(Debug, Copy, Clone)]
pub struct Degree(pub f64);

impl From<Radian> for Degree {
    /// ラジアンから変換する。
    fn from(rad: Radian) -> Self {
        Degree(rad.0.to_degrees())
    }
}
