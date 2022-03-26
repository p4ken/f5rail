use std::{
    f64::consts::{FRAC_2_PI, PI},
    ops::{Div, Mul},
};

use derive_more::{Add, From, Sub};

use super::unit::{Deg, Meter, Rad};

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
#[derive(Debug, Clone, Copy, PartialEq, From, Add, Sub)]
pub struct Subtension(f64);

impl Meter for Subtension {
    fn meter(self) -> f64 {
        self.0
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

/// 曲率 (1/m)
///
/// 右カーブが正、左カーブが負。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Add, Sub)]
pub struct Curvature(f64);

pub const STRAIGHT: Curvature = Curvature(0.0);

impl From<Radius> for Curvature {
    /// 半径 `r` から曲率を作成する。
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
    pub fn a(&self, s: Subtension) -> Central {
        // 中心角 = 曲率 * 弧長
        // 反時計回りが正になるように曲率の符号を反転する。
        Central(-self.0 * s.meter())
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
///
/// Data Transfer Object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radius(pub f64);

impl Meter for Radius {
    fn meter(self) -> f64 {
        self.0
    }
}

/// 中心角 (rad)
#[derive(Debug, Clone, Copy, PartialEq, From, Add, Sub)]
pub struct Central(f64);

impl Rad for Central {
    fn rad(&self) -> f64 {
        self.0
    }
}

/// 接線方向 (rad)
#[derive(Debug, Clone, Copy, PartialEq, From, Add)]
pub struct Tangential(f64);

impl Tangential {
    pub fn to_central(&self, k: Curvature) -> Central {
        let gap = if k.is_right() { FRAC_2_PI } else { -FRAC_2_PI };
        Central(self.rad() + gap)
    }
}

impl Rad for Tangential {
    fn rad(&self) -> f64 {
        self.0
    }
}

/// 角度 (度)
///
/// Data Transfer Object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degree(pub f64);

impl Deg for Degree {
    fn deg(self) -> f64 {
        self.0
    }
}
