use std::ops::AddAssign;

use super::{
    formula::{Degree, Point},
    spiral::Line,
};

/// 緩和曲線の距離程を分割する構造体。
///
/// - 距離程の原点(0m)は緩和曲線の始点とは限らず、任意の場所にある。
/// - 区間同士の境界では距離程が整数になり、最初の始点と最後の終点のみ小数になりうる。
pub struct Segmentation {
    /// 初回区間
    first: (f64, i32),

    /// 最終区間
    last: (i32, f64),

    /// 現在区間の始点
    l0: i32,

    /// 現在区間の始点までの回転角
    a0: Degree,

    /// 現在区間の始点の座標
    p0: Point,
}

impl Segmentation {
    /// `l0` で始まり `l1` で終わる距離程の分割を表す構造体を生成する。
    ///
    /// `l0 > l1` の場合は未定義動作。
    pub fn new(l0: f64, l1: f64, p0: &Point) -> Self {
        Self {
            first: (l0, l0.floor() as i32 + 1),
            last: (l1.ceil() as i32 - 1, l1),
            l0: l0 as i32,
            a0: Degree(0.0),
            p0: *p0,
        }
    }
}

impl Iterator for Segmentation {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        return None;
        // 終了判定
        if self.l0 - 1 == self.last.0 {
            return None;
        }

        // 区間始点
        let l0 = match self.l0 + 1 == self.first.1 {
            true => self.first.0, // 初回区間
            false => self.l0 as f64,
        };

        // 区間終点
        let l1 = match self.l0 == self.last.0 {
            true => self.last.1, // 最終区間
            false => (self.l0 + 1) as f64,
        };

        // 区間長
        let len = l1 - l0;

        // 緩和曲線始点から区間中央までの弧長
        let s = l0 - self.first.0 + (len / 2.0);

        // 次回区間
        self.l0 += 1;

        Some(Self::Item { s, len })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.last.0 - self.first.1 + 2) as usize;
        (size, Some(size))
    }
}

/// ひとつの区間
#[derive(Debug)]
pub struct Segment {
    /// 緩和曲線始点から区間中央までの弧長
    pub s: f64,

    /// 区間長
    pub len: f64,
}

/// 前回までの緩和曲線の先端の状態
#[derive(Debug, Clone)]
pub struct Head {
    /// 次回の始点座標
    pub p0: Point,

    /// 次回の始点回転角
    pub a0: Degree,
}

impl Head {
    /// コンストラクタ
    pub fn new(p0: &Point, a0: Degree) -> Self {
        Self { p0: *p0, a0 }
    }
}

impl AddAssign<&Line> for Head {
    /// 線分を加算する
    fn add_assign(&mut self, rhs: &Line) {
        match rhs {
            Line::Straight(_, p1) => self.p0 = *p1,
            Line::Curve(c, r, _, a1) => {
                self.p0 = c + (*r, *a1);
                self.a0 = *a1;
            }
        }
    }
}
