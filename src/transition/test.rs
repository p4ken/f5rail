use super::spiral::*;
use super::formula::*;
use super::*;

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(Diminish::Sine, Some(Radius(300.)), Some(Radius(-300.)), 19.0, 0.0);
    let segments = plot(&param);
    let segments = segments.unwrap();
    assert_eq!(segments.len(), 19);
    assert_eq!(segments[0].unwrap_curve().r.round2(), "301.03");
}

impl Radius {
    fn round2(&self) -> String {
        format!("{:.2}", self.0)
    }
}

impl Line {
    fn unwrap_curve(&self) -> &Curve {
        match self {
            Self::Curve(c) => c,
            _ => panic!("This is not curve."),
        }
    }
    fn unwrap_straight(&self) -> &Straight {
        match self {
            Self::Straight(l) => l,
            _ => panic!("This is not straight."),
        }
    }
}
