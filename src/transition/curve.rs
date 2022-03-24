use std::{
    f64::consts::PI,
    ops::{Div, Mul},
};

use derive_more::{Add, Sub};

use super::unit::Meter;

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
    pub fn k(&self, tcl: Subtension, s: Subtension, k0: Curvature, k1: Curvature) -> Curvature {
        // 緩和曲線長に対する弧長の比率 (0 <= x <= 1)
        let x = s / tcl;
        debug_assert!(x <= 1.0);

        // 曲率の配分 (0 <= y <= 1)
        let y = match self {
            Diminish::Sine => ((x - 0.5) * PI).sin() / 2.0 + 0.5,
            Diminish::Linear => x,
        };

        // 曲率
        k0 + (k1 - k0) * y
    }
}

/// 弧長 (m)
#[derive(Debug, Clone, Copy, PartialEq, Add, Sub)]
pub struct Subtension(f64);

impl From<f64> for Subtension {
    /// コンストラクタ
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl Meter for Subtension {
    fn meter(self) -> f64 {
        self.0
    }
}

impl From<Subtension> for f64 {
    /// キャスト
    fn from(f: Subtension) -> Self {
        f.0
    }
}

impl Mul<f64> for Subtension {
    type Output = Self;

    /// 掛け算
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div for Subtension {
    type Output = f64;

    /// 比率
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

/// 円弧
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arc {
    /// 曲率
    k: Curvature,

    /// 長さ
    len: Subtension,
}

impl Arc {
    /// コンストラクタ
    pub fn new(k: Curvature, len: Subtension) -> Self {
        Self { k, len }
    }

    /// 中心角
    pub fn angle(&self) -> Radian {
        self.k.angle(self.len)
    }

    /// 直線なら `true`
    pub fn is_straight(&self) -> bool {
        self.k.is_straight()
    }

    /// 左カーブなら `true`
    pub fn is_left(&self) -> bool {
        self.k.is_left()
    }

    /// 右カーブなら `true`
    pub fn is_right(&self) -> bool {
        self.k.is_left()
    }

    /// 半径
    pub fn r(&self) -> Option<Radius> {
        self.k.r()
    }

    /// 長さ
    pub fn len(&self) -> Subtension {
        self.len
    }
}

/// 曲率 (1/m)
///
/// 右カーブが正、左カーブが負。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Add, Sub)]
pub struct Curvature(f64);

pub const STRAIGHT: Curvature = Curvature(0.0);

impl From<Radius> for Curvature {
    /// 半径 `r` (m) から曲率を作成する。
    fn from(r: Radius) -> Self {
        Self(r.0.recip())
    }
}

impl Curvature {
    /// 直線なら `true`
    pub fn is_straight(&self) -> bool {
        *self == STRAIGHT
    }

    /// 左カーブなら `true`
    pub fn is_left(&self) -> bool {
        *self < STRAIGHT
    }

    /// 右カーブなら `true`
    pub fn is_right(&self) -> bool {
        *self > STRAIGHT
    }

    /// 弧長 `s` から中心角を計算する。
    pub fn angle(&self, s: Subtension) -> Radian {
        // 中心角 = 曲率 * 弧長
        // 反時計回りが正になるように曲率の符号を反転する。
        Radian(-self.0 * s.meter())
    }

    /// 半径 (m)
    ///
    /// 直線は `None`
    pub fn r(&self) -> Option<Radius> {
        (*self != STRAIGHT).then(|| Radius(self.0.recip()))
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
/// 右カーブが正、左カーブが負。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radius(pub f64);

impl Meter for Radius {
    fn meter(self) -> f64 {
        self.0
    }
}

// /// 回転角
// pub struct Rotation<T>(T);

// impl<T> Rotation<T>{

// }

/// 角度 (ラジアン)
///
/// 反時計回りが正、時計回りが負。
#[derive(Debug, Clone, Copy, PartialEq, Add, Sub)]
pub struct Radian(f64);

impl From<f64> for Radian {
    /// コンストラクタ
    fn from(rad: f64) -> Self {
        Self(rad)
    }
}

impl Radian {
    /// サイン
    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    /// コサイン
    pub fn cos(&self) -> f64 {
        self.0.cos()
    }
}

// impl Mul for Radian {
//     type Output = Self;

//     /// 掛け算
//     fn mul(self, rhs: Self) -> Self::Output {
//         Self(self.0 * rhs.0)
//     }
// }

impl From<Degree> for Radian {
    /// 度から変換する。
    fn from(rad: Degree) -> Self {
        Self(rad.0.to_radians())
    }
}

/// 角度 (度)
///
/// 反時計回りが正、時計回りが負。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degree(pub f64);

impl From<Radian> for Degree {
    /// ラジアンから変換する。
    fn from(rad: Radian) -> Self {
        Self(rad.0.to_degrees())
    }
}

// /// 単位
// trait Unit {
//     fn v() -> f64;
// }

// impl<T: Unit + From<f64>> Add for T {
//     type Output = T;

//     fn add(self, rhs: Self) -> Self::Output {
//         T::from(self.v() + rhs.v())
//     }
// }
