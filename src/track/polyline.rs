use anyhow::Result;

/// 1つの軌道を表す連続線
pub struct Polyline(Vec<Stroke>);

impl Polyline {
    pub fn new() -> Result<Self> {
        Ok(Self(vec![]))
    }
}

// 連続線を作成するために必要な情報
//

// 連続線が保持する具体的な情報 これはimpl？
pub enum Stroke {
    // 直線
    Straight(Point, Point),

    // 円弧
    Arc(Point, Radius, Degree, Degree),
}

pub struct Point(pub f64, pub f64);
pub struct Radius(pub f64);
pub struct Degree(pub f64);
