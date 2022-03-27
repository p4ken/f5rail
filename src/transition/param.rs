use super::{
    canvas::Point,
    curve::{Curvature, Diminish, Subtension, Tangential},
    distance::Distance,
};

/// 緩和曲線パラメータ
#[derive(Debug)]
pub struct Param {
    /// 逓減関数
    pub diminish: Diminish,

    /// 始点の曲率
    pub k0: Curvature,
    
    /// 終点の曲率
    pub k1: Curvature,

    /// 始点の距離程
    pub l0: Distance<f64>,

    /// 緩和曲線長
    pub tcl: Subtension,

    /// 始点の座標
    pub p0: Point,

    /// 始点の接線方向
    pub t0: Tangential,
}
