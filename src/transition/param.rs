/// 緩和曲線パラメータ
#[derive(Debug)]
pub struct Param {
    pub func: Func,
    pub r1: Option<f64>,
    pub r2: Option<f64>,
    pub tcl: f64,
    pub dx: f64,
}

/// 緩和曲線関数
#[derive(Debug)]
pub enum Func {
    Sin,
    Linear,
}
