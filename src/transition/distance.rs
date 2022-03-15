/// 区間に分割された距離程。
///
/// 距離程の原点 (0m) は緩和曲線の始点とは限らず、任意の場所にある。
///
/// 区間同士の境界では距離程が整数になり、最初の始点と最後の終点のみ小数になりうる。
pub struct Segmentation {
    /// 初回区間 (始点, 終点)
    first: (f64, i32),

    /// 最終区間 (始点, 終点)
    last: (i32, f64),

    /// 現在区間の終点
    l1: i32,
}

impl Segmentation {
    /// 緩和曲線の始点の距離程 `l0` と緩和曲線長 `tcl` から区間に分割する。
    pub fn new(l0: f64, tcl: f64) -> Self {
        let l1 = l0 + tcl;
        Self {
            first: (l0, l0.floor() as i32 + 1),
            last: (l1.ceil() as i32 - 1, l1),
            l1: l0 as i32,
        }
    }
}

impl Iterator for Segmentation {
    type Item = Segment;

    /// 次回区間を取得する。
    fn next(&mut self) -> Option<Self::Item> {
        // 終了判定
        if self.l1 > self.last.0 {
            return None;
        }

        // 次回区間
        self.l1 += 1;
        let segment = Segment::new(&self);

        Some(segment)
    }

    /// 区間数
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.last.0 - self.first.1 + 2) as usize;
        (size, Some(size))
    }
}

/// ひとつの区間の弧長
///
/// 弧長は緩和曲線始点からの距離。距離程とは異なる。
#[derive(Debug)]
pub struct Segment {
    /// 区間始点の弧長
    s0: f64,

    /// 区間終点の弧長
    s1: f64,
}

impl Segment {
    /// 区間の距離程から弧長を作成する。
    pub fn new(distance: &Segmentation) -> Self {
        // 区間始点の弧長
        let s0 = match distance.l1 == distance.first.1 {
            true => 0.0, // 初回区間
            false => (distance.l1 - 1) as f64 - distance.first.0,
        };

        // 区間終点の距離程
        let l1 = match distance.l1 > distance.last.0 {
            true => distance.last.1, // 最終区間
            false => distance.l1 as f64,
        };

        // 区間終点の弧長
        let s1 = l1 - distance.first.0;

        Self { s0, s1 }
    }

    /// 弧長の代表値
    pub fn s(&self) -> f64 {
        // 区間中央の値を利用する。
        (self.s1 + self.s0) / 2.0
    }

    /// 区間長
    pub fn len(&self) -> f64 {
        self.s1 - self.s0
    }
}
