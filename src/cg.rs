//! ドメインオブジェクト

use derive_more::Constructor;

use crate::unit::*;

pub struct Track(pub Vec<Anchor>);

impl Track {
    // 連続線はcadのXY座標系になっている...
    // pub fn calculate(/* polylines: Polyline */) -> Self {
    //     Self(vec![])
    // }
}

pub struct Anchor {
    z: Meter, // 距離程
    x: Meter, // 自線との間隔
    r: f64,   // 相対半径
}

// 自線の線分の端点または、他線の線分の端点に最も近い自線上の点。
#[derive(Constructor)]
pub struct Anchor0(Meter); // 距離程Z
