use super::formula::*;
use super::spiral::*;
use super::*;

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(Some(300.)),
        Radius(Some(-300.)),
        19.0,
        0.0,
    );
    let segments = plot(&param);
    assert_eq!(segments.0.len(), 19);
    assert_eq!(format!("{:.2}", segments.0[0].r().unwrap()), "301.03");
}

impl Line {
    fn r(&self) -> Option<f64> {
        match self {
            Self::Curve(_, r, _, _) => Some(*r),
            _ => None,
        }
    }
}
