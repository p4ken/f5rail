//! ドメインオブジェクト

use derive_more::Constructor;

use crate::unit::*;

#[derive(Constructor)]
pub struct Polyline(Vec<Line>); // 連続線。内部で連続線判定

pub enum Line {
    Straight(Point, Point),
    Curve(Point, Radius, Angle, Angle),
}

// 線分の端点(連続線上の点)もPointを使っているが、特化した型を作りたい
#[derive(Constructor)]
pub struct Point(Meter, Meter);
// impl From<(Meter, Radian)> for Point {} // Derefでもいいかも

pub struct Radius(Meter);

pub struct Angle(Radian);
