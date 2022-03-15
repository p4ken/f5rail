use super::{formula::{Curvature, Diminish, Radian, Radius}, spiral::Point};

/// 緩和曲線パラメータ
#[derive(Debug)]
pub struct Param {
    /// 逓減
    pub diminish: Diminish,

    /// 始点の曲率
    pub k0: Curvature,

    /// 終点の曲率
    pub k1: Curvature,

    /// 始点の距離程
    pub l0: f64,

    /// 緩和曲線長
    pub tcl: f64,

    /// 始点の座標
    pub p0: Point,

    /// 始点の進行方向（接線の向き）
    pub t0: Radian,
}

impl Param {
    pub fn new(diminish: Diminish, r0: Radius, r1: Radius, tcl: f64, l0: f64) -> Self {
        Self {
            diminish,
            k0: r0.into(),
            k1: r1.into(),
            l0,
            tcl,
            p0: Point(0.0, 0.0),
            t0: Radian(0.0),
        }
    }
}
