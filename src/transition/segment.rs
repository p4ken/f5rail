use std::ops::AddAssign;

use super::{
    formula::{Point, Radian},
    spiral::Line,
};

/// 緩和曲線の距離程を分割する構造体。
///
/// 距離程の原点(0m)は緩和曲線の始点とは限らず、任意の場所にある。
///
/// 区間同士の境界では距離程が整数になり、最初の始点と最後の終点のみ小数になりうる。
pub struct Segmentation {
    /// 初回区間
    first: (f64, i32),

    /// 最終区間
    last: (i32, f64),

    /// 現在区間の始点
    l0: i32,
}

impl Segmentation {
    /// コンストラクタ
    pub fn new(l0: f64, tcl: f64) -> Self {
        let l1 = l0 + tcl;
        Self {
            first: (l0, l0.floor() as i32 + 1),
            last: (l1.ceil() as i32 - 1, l1),
            l0: l0 as i32,
        }
    }
}

impl Iterator for Segmentation {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
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

        Some(Segment(l0, l1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.last.0 - self.first.1 + 2) as usize;
        (size, Some(size))
    }
}

/// 距離程の区間
#[derive(Debug)]
pub struct Segment(f64, f64);

impl Segment {
    /// 緩和曲線始点から区間中央までの弧長
    pub fn s(&self) -> f64 {
        (self.1 + self.0) / 2.0
    }

    /// 区間長
    pub fn len(&self) -> f64 {
        self.1 - self.0
    }
}

/// 緩和曲線の先端の状態
#[derive(Debug, Clone)]
pub struct Head {
    /// 次回の始点座標
    pub p0: Point,

    /// 次回の始点の進行方向（接線の向き）
    pub t0: Radian,
}

impl Head {
    /// コンストラクタ
    pub fn new(p0: &Point, t0: Radian) -> Self {
        Self { p0: *p0, t0 }
    }
}

impl AddAssign<&Line> for Head {
    /// 線分を加算する
    fn add_assign(&mut self, rhs: &Line) {
        match rhs {
            Line::Straight(_, p1) => self.p0 = *p1,
            Line::Curve(c, r, a0, a1) => {
                self.p0 = c + (*r, *a1);
                self.t0 = self.t0 - (*a1 - *a0); // 左カーブ→引く、右カーブ→足す
            }
        }
    }
}
