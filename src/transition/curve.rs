use std::{
    f64::consts::PI,
    ops::{Add, Mul, Not, Sub},
};

use super::distance::ArcLength;

/// 逓減関数
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Diminish {
    /// サイン半波長逓減
    Sine,

    /// 直線逓減（クロソイド曲線）
    Linear,
}

impl Diminish {
    /// 曲率を計算する。
    pub fn k(&self, tcl: ArcLength, s: ArcLength, k0: Curvature, k1: Curvature) -> Curvature {
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Curvature(f64);

impl Curvature {
    /// 直線なら `true`
    pub fn is_straight(&self) -> bool {
        self.0 == 0.0
    }

    /// 左カーブなら `true`
    pub fn is_left(&self) -> bool {
        self.0 < 0.0
    }

    /// 右カーブなら `true`
    pub fn is_right(&self) -> bool {
        self.0 > 0.0
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
        Curvature(r.0.map_or(0.0, |r| r.recip()))
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

    /// 比率
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

/// 半径 (m)
///
/// 左カーブなら負の数。
/// 
/// 直線の場合は `None`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radius(Option<f64>);

impl From<Option<f64>> for Radius {
    /// コンストラクタ
    fn from(r: Option<f64>) -> Self {
        Self(r)
    }
}

impl From<Curvature> for Radius {
    /// 曲率から変換する。
    fn from(k: Curvature) -> Self {
        Self(k.is_straight().not().then(|| k.0.recip()))
    }
}

impl Radius {
    /// 生の値
    #[deprecated]
    pub fn raw(&self) -> Option<f64> {
        self.0
    }
}

/// 角度 (ラジアン)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radian(f64);

impl From<f64> for Radian {
    /// コンストラクタ
    fn from(rad: f64) -> Self {
        Self(rad)
    }
}

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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degree(f64);

impl From<Radian> for Degree {
    /// ラジアンから変換する。
    fn from(rad: Radian) -> Self {
        Degree(rad.0.to_degrees())
    }
}
