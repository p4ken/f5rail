use super::{
    canvas::{Point, Tangent},
    curve::{Curvature, Diminish, Subtension},
    distance::Distance,
};

/// 緩和曲線パラメータ
#[derive(Debug)]
pub struct Param {
    /// 逓減関数
    pub diminish: Diminish,

    /// 始点・終点の曲率
    pub k: (Curvature, Curvature),

    /// 始点の距離程
    pub l0: Distance<f64>,

    /// 緩和曲線長
    pub tcl: Subtension,

    /// 始点の接線
    pub t0: Tangent,
}

impl Param {
    pub fn new(
        diminish: Diminish,
        k0: Curvature,
        k1: Curvature,
        tcl: Subtension,
        l0: Distance<f64>,
    ) -> Self {
        Self {
            diminish,
            k: (k0, k1),
            l0,
            tcl,
            t0: Tangent::new(Point(0.0, 0.0), 0.0.into()),
        }
    }
}
