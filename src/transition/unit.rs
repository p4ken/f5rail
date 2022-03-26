
// /// 単位
// pub trait Unit: From<f64> {
//     fn float(self) -> f64;
// }

// impl<T: Unit> Mul<f64> for T {
//     type Output = Self;

//     fn mul(self, rhs: f64) -> Self::Output {
//         Self::from(self.float() * rhs)
//     }
// }

/// メートル
pub trait Meter /*: From<f64>*/ {
    fn meter(self) -> f64;
}

/// ラジアン
pub trait Rad {
    /// 一般角。反時計回りが正。
    fn rad(&self) -> f64;
    /// サイン
    fn sin(&self) -> f64 {
        self.rad().sin()
    }
    /// コサイン
    fn cos(&self) -> f64 {
        self.rad().cos()
    }
}

/// 度
///
/// 反時計回りが正、時計回りが負。
pub trait Deg {
    fn deg(self) -> f64;
}

/// ベクトル
///
/// 極座標 `(Meter, Rad)` に自動で実装される。
pub trait Vector {
    fn x(self) -> f64;
    fn y(self) -> f64;
}

impl<T: Meter, U: Rad> Vector for (T, U) {
    fn x(self) -> f64 {
        self.0.meter() * self.1.cos()
    }
    fn y(self) -> f64 {
        self.0.meter() * self.1.sin()
    }
}
