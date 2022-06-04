use anyhow::{ensure, Result};

/// 1つの軌道を表す連続線
pub struct Polyline(Vec<Stroke>);

impl FromIterator<Stroke> for Result<Polyline> {
    fn from_iter<T: IntoIterator<Item = Stroke>>(iter: T) -> Self {
        let mut iter = iter.into_iter().peekable();
        ensure!(iter.peek().is_some(), "線データが選択されていません");
        // TODO
        Ok(Polyline(vec![Stroke::ToDo]))
    }
}

// 連続線を作成するために必要な情報：始終点の座標
pub trait Edge {
    /// 始点座標
    fn start(&self) -> Option<Point>;

    /// 終点座標
    fn end(&self) -> Option<Point>;
}

// 連続線が保持する具体的な情報 これはrelativeの実装？
pub enum Stroke {
    // 直線
    Straight(Point, Point),

    // 円弧
    Arc(Point, Radius, Degree, Degree),

    ToDo,
}

pub struct Point(pub f64, pub f64);

pub struct Radius(pub f64);
pub struct Degree(pub f64);
