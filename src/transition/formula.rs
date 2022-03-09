use std::fmt::Debug;

/// 半径
pub struct Radius(pub Option<f64>);

impl Radius {
    /// 曲率に変換する
    pub fn k(&self) -> Curvature {
        let k = self.0.map_or(0., |r| 1. / r);
        Curvature(k)
    }
}

/// 曲率
#[derive(Debug)]
pub struct Curvature(pub f64);

impl Curvature{
    pub fn is_straight(&self) -> bool {
        self.0 == 0.
    }
}

/// 逓減の定義
#[derive(Debug)]
pub enum Spiral {
    /// サイン半波長逓減曲線
    Sine,

    /// クロソイド曲線
    Clothoid,
}

impl Spiral {
    /// 曲率を計算する
    pub fn k(&self, s: f64) -> Curvature {
        Curvature(0.)
    }
}
