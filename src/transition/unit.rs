//! 単位
//!
//! 構造体の値の単位は意識する必要はなく、簡単に変換できたほうがよい。

/// メートル
pub trait Meter {
    fn meter(&self) -> f64;
}

/// ラジアン
pub trait Rad {
    /// 一般角。反時計回りが正。
    fn rad(&self) -> f64;
}

/// 度
pub trait Deg {
    fn deg(&self) -> f64;
}

impl<T: Rad> Deg for T {
    /// 度
    fn deg(&self) -> f64 {
        self.rad().to_degrees()
    }
}

/// ベクトル
///
/// 極座標 `(Meter, Rad)` に自動で実装される。
pub trait Vector {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl<T: Meter + Copy, U: Rad + Copy> Vector for (T, U) {
    fn x(&self) -> f64 {
        self.0.meter().abs() * self.1.rad().cos()
    }
    fn y(&self) -> f64 {
        self.0.meter().abs() * self.1.rad().sin()
    }
}
