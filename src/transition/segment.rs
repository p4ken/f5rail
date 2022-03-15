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

        // 区間の始点
        let l0 = match self.l0 + 1 == self.first.1 {
            true => self.first.0, // 初回区間
            false => self.l0 as f64,
        };

        // 区間の終点
        let l1 = match self.l0 == self.last.0 {
            true => self.last.1, // 最終区間
            false => (self.l0 + 1) as f64,
        };

        // 次回区間
        self.l0 += 1;

        Some(Segment(l0, l1))
    }

    /// 区間数
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
