/// メートル
pub trait Meter /*: From<f64>*/ {
    fn meter(self) -> f64;
}
