use super::formula::{Curvature, Radius, Spiral};

/// 緩和曲線パラメータ
#[derive(Debug)]
pub struct Param {
    /// 逓減の定義
    pub spiral: Spiral,

    /// 始点の座標
    pub p0: (f64, f64),

    /// 始点の曲率
    pub k0: Curvature,

    /// 終点の曲率
    pub k1: Curvature,

    /// 緩和曲線長
    pub tcl: f64,
}

impl Param {
    pub fn new(spiral: Spiral, r0: Option<f64>, r1: Option<f64>, tcl: f64) -> Self {
        Self {
            spiral,
            p0: (0., 0.),
            k0: Radius(r0).k(),
            k1: Radius(r1).k(),
            tcl,
        }
    }
}
