use std::ops::{Add, Sub};

use super::{curve::Subtension, unit::Meter};

/// 距離程の区間分割
///
/// 1m単位の区間に分割される。
pub struct Ruler {
    /// 初回区間 (始点, 終点)
    first: (Distance<f64>, Distance<i32>),

    /// 最終区間 (始点, 終点)
    last: (Distance<i32>, Distance<f64>),

    /// 現在区間の終点
    l1: Distance<i32>,
}

impl Ruler {
    /// 始点の距離程と長さから、緩和曲線の区間分割を作成する。
    pub fn new(first_l0: Distance<f64>, tcl: Subtension) -> Self {
        let l1 = first_l0.floor();
        let last_l1 = first_l0 + tcl;

        Self {
            first: (first_l0, l1.next()),
            last: (last_l1.prev(), last_l1),
            l1,
        }
    }
}

impl Iterator for Ruler {
    type Item = Interval;

    /// 次回区間を取得する。
    fn next(&mut self) -> Option<Self::Item> {
        // 終了判定
        if self.l1 > self.last.0 {
            return None;
        }

        // 次回区間
        self.l1.advance();
        Some(Interval::new(&self))
    }

    /// 区間数
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.last.0 - self.first.1 + 2) as usize;
        (size, Some(size))
    }
}

/// 距離程 (m)
///
/// 距離程の原点(0m)は任意の場所にある。
///
/// 区間境界は1m単位の距離程になる。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Distance<T>(T);

impl From<f64> for Distance<f64> {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl<T: Meter> Add<T> for Distance<f64> {
    type Output = Self;

    /// 足し算
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs.meter())
    }
}

impl Distance<f64> {
    /// 切り捨て
    fn floor(&self) -> Distance<i32> {
        Distance(self.0.floor() as i32)
    }

    /// 切り上げ
    fn ceil(&self) -> Distance<i32> {
        Distance(self.0.ceil() as i32)
    }

    /// 前の区間境界
    fn prev(&self) -> Distance<i32> {
        self.ceil().prev()
    }
}

impl<T: Sub<Output = T>> Sub for Distance<T> {
    type Output = T;

    /// 2点間の距離
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Distance<i32> {
    /// 小数の距離程
    fn as_float(&self) -> Distance<f64> {
        Distance(self.0 as f64)
    }

    /// 次の区間境界
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    /// 前の区間境界
    fn prev(&self) -> Self {
        Self(self.0 - 1)
    }

    /// 次の区間境界に進める
    fn advance(&mut self) {
        *self = self.next()
    }
}

/// 1つの区間
#[derive(Debug)]
pub struct Interval {
    /// 緩和曲線始点から区間始点までの弧長
    s0: Subtension,

    /// 緩和曲線始点から区間終点までの弧長
    s1: Subtension,
}

impl Interval {
    ///区間の弧長を作成する。
    pub fn new(ruler: &Ruler) -> Self {
        // 区間始点までの弧長
        let s0 = match ruler.l1 == ruler.first.1 {
            true => 0.0, // 初回区間
            false => ruler.l1.prev().as_float() - ruler.first.0,
        };

        // 区間終点までの弧長
        let s1 = match ruler.l1 > ruler.last.0 {
            true => ruler.last.1, // 最終区間
            false => ruler.l1.as_float(),
        } - ruler.first.0;

        Self {
            s0: s0.into(),
            s1: s1.into(),
        }
    }

    /// 緩和曲線始点から区間中央までの弧長
    pub fn s(&self) -> Subtension {
        (self.s1 + self.s0) * 0.5
    }

    /// 区間長
    pub fn len(&self) -> Subtension {
        self.s1 - self.s0
    }
}
