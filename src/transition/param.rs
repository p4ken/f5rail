/// 緩和曲線パラメータ
pub struct Param {
    pub func: Func,
    pub r1: Option<i32>,
    pub r2: Option<i32>,
    pub tcl: i32,
    pub dx: f64,
}

/// 緩和曲線関数
pub enum Func {
    Sin,
    Linear,
}

impl Func {
    pub fn to_string(&self) -> &str {
        match self {
            Sin => "サイン半波長逓減",
            Linear => "直線逓減",
        }
    }
}
