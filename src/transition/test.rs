use super::formula::*;
use super::spiral::*;
use super::*;

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(Some(300.0)),
        Radius(Some(-300.0)),
        19.0,
        0.0,
    );
    let segments = plot(&param);
    assert_eq!(segments.0.len(), 19);
    assert_eq!(format!("{:.2}", segments.0[0].r().unwrap()), "301.03");
    assert_eq!(format!("{:.2}", segments.0[1].r().unwrap()), "309.47");
    // assert_eq!(format!("{:.2}", segments.0[2].r().unwrap()), "327.58");
    assert_eq!(format!("{:.2}", segments.0[3].r().unwrap()), "358.35");
    assert_eq!(format!("{:.2}", segments.0[4].r().unwrap()), "407.76");
    assert_eq!(format!("{:.2}", segments.0[5].r().unwrap()), "488.43");
    assert_eq!(format!("{:.2}", segments.0[6].r().unwrap()), "630.32");
    assert_eq!(format!("{:.2}", segments.0[7].r().unwrap()), "923.93");
    assert_eq!(format!("{:.0}", segments.0[8].r().unwrap()), "1823");
    assert!(segments.0[9].r().is_none());
}

impl Line {
    fn r(&self) -> Option<f64> {
        match self {
            Self::Curve(_, r, _, _) => Some(*r),
            _ => None,
        }
    }
}
