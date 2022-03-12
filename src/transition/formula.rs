use std::fmt::Debug;

/// 逓減の定義。
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
pub struct Radius(pub f64);

impl Radius {
    /// 曲率
    pub fn k(&self) -> Curvature {
        Curvature(1. / self.0)
    }
}

/// 曲率 (1/m)
#[derive(Debug, PartialEq)]
pub struct Curvature(pub f64);

const STRAIGHT: Curvature = Curvature(0.);

impl Curvature {
    pub fn is_straight(&self) -> bool {
        self.0 == STRAIGHT.0
    }
}

impl From<Option<Radius>> for Curvature {
    fn from(r: Option<Radius>) -> Self {
        r.map_or(STRAIGHT, |r| r.k())
    }
}

/// 角度 (度)
#[derive(Debug)]
pub struct Degree(pub f64);

/// 点
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
