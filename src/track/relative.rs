use anyhow::Result;

/// Bveの相対座標
pub trait Relative{
    // agentに持たせていいかも？ BVE特有なので。
    // track特有でもあるのでtrackでいい。ドメイン中心。
    fn distance(&self) -> f64;
    fn spacing(&self) -> f64;
    fn radius(&self) -> f64;
}

// impl IntoIterator for Relative {
//     type Item = ();

//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

pub struct Relative_ {
    z: Trip,    // 距離程
    x: Spacing, // 自線との距離
    r: f64,     // 自線との相対半径
}

impl Relative_ {
    /// 他線のBVE相対座標を計算する。
    ///
    /// エラーが出ないようにしたい。
    pub fn between(_track_0: &Polyline, _track_x: &Polyline) -> Result<Vec<Relative_>> {
        // 測距点の距離程を決める (等間隔 or 変化点)
        // 測距点毎に、自線～他線のスペースを算出する
        // 距離程とスペースをマップに出力する

        todo!()
    }
}

pub struct Polyline {
    v: Vec<Stroke>,
}

pub enum Stroke {
    Straight,
    Curve,
}

impl Stroke {
    // 自線.anchor() -> 自線上の座標、距離程
    // 他線.anchor() -> 他線上の座標
    fn anchor(&self) -> &Anchor {
        todo!()
    }

    // 自線.trip(自線アンカー) -> 距離程
    fn trip(&self, _p: &Anchor) -> Option<f64> {
        todo!()
    }
}

struct Anchor {
    x: f64,
    y: f64,
}

impl Anchor {
    // 自線アンカー.between(他線) -> X, Z
    // 他線アンカー.between(自線) -> X, Z
    fn between(&self, _stroke: &Stroke) -> Option<(Spacing, Trip)> {
        // TODO: 点と直線・円弧の距離
        // TODO: 距離程も計算
        // - 自線アンカー ->
        todo!()
    }

    // fn nearest_

    // 他線アンカー.trip_on(自線)
    fn trip_on(&self, _stroke: &Stroke) -> Option<f64> {
        // TODO: 最も近い直線・円弧上の点
        todo!()
    }
}

struct Spacing(f64);

struct Trip(f64);
