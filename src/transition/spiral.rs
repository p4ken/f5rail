use super::formula::{Degree, Point, Radius};

/// 緩和曲線
/// - 円弧または直線の集合として表現される。
pub type Spiral = Vec<Line>;

/// 線分
pub enum Line {
    /// 円弧
    Curve {
        /// 中心点
        c: Point,

        /// 半径
        r: Radius,

        /// 始角
        a: Degree,

        /// 終角
        b: Degree,
    },

    /// 直線
    Straight {
        /// 始点
        p0: Point,

        /// 終点
        p1: Point,
    },
}

impl Line {
    /// 直線を生成する。
    pub fn straight(p0: Point, a: Degree, len: f64) -> Self {
        // 三角関数？

        Self::Straight { p0, p1: p0 }
    }

    /// 円弧を生成する。
    pub fn curve(c: Point, r: Radius, a: Degree, b: Degree) -> Self {
        Self::Curve { c, r, a, b }
    }
}

/// 円弧
pub struct Curve {
    /// 中心点
    pub c: Point,

    /// 半径
    pub r: Radius,

    /// 始角
    pub a: Degree,

    /// 終角
    pub b: Degree,
}

/// 直線
pub struct Straight {
    /// 始点
    pub p0: Point,

    /// 終点
    pub p1: Point,
}
