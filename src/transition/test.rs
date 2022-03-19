use super::curve::*;
use super::spiral::*;
use super::*;

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(Some(300.0)),
        Radius(Some(-300.0)),
        19.0,
        0.0.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 19);
    assert_eq!(format!("{:.2}", spiral.0[0].r().unwrap()), "301.03");
    assert_eq!(format!("{:.2}", spiral.0[1].r().unwrap()), "309.47");
    assert_eq!(format!("{:.2}", spiral.0[2].r().unwrap()), "327.59");
    assert_eq!(format!("{:.2}", spiral.0[3].r().unwrap()), "358.35");
    assert_eq!(format!("{:.2}", spiral.0[4].r().unwrap()), "407.76");
    assert_eq!(format!("{:.2}", spiral.0[5].r().unwrap()), "488.43");
    assert_eq!(format!("{:.2}", spiral.0[6].r().unwrap()), "630.32");
    assert_eq!(format!("{:.2}", spiral.0[7].r().unwrap()), "923.93");
    assert_eq!(format!("{:.0}", spiral.0[8].r().unwrap()), "1823");
    assert!(spiral.0[9].r().is_none());
}

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m_始点1m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(Some(300.0)),
        Radius(Some(-300.0)),
        19.0,
        1.0.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 19);
    assert_eq!(format!("{:.2}", spiral.0[0].r().unwrap()), "301.03");
    assert_eq!(format!("{:.2}", spiral.0[1].r().unwrap()), "309.47");
    assert_eq!(format!("{:.2}", spiral.0[2].r().unwrap()), "327.59");
    assert_eq!(format!("{:.2}", spiral.0[3].r().unwrap()), "358.35");
    assert_eq!(format!("{:.2}", spiral.0[4].r().unwrap()), "407.76");
    assert_eq!(format!("{:.2}", spiral.0[5].r().unwrap()), "488.43");
    assert_eq!(format!("{:.2}", spiral.0[6].r().unwrap()), "630.32");
    assert_eq!(format!("{:.2}", spiral.0[7].r().unwrap()), "923.93");
    assert_eq!(format!("{:.0}", spiral.0[8].r().unwrap()), "1823");
    assert!(spiral.0[9].r().is_none());
}

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m_始点0_5m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(Some(300.0)),
        Radius(Some(-300.0)),
        19.0,
        0.5.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 20);
    // 0.5m
    assert_eq!(format!("{:.2}", spiral.0[0].r().unwrap()), "300.26");
    // 1m
    assert_eq!(format!("{:.2}", spiral.0[1].r().unwrap()), "304.15");
    assert_eq!(format!("{:.2}", spiral.0[2].r().unwrap()), "317.19");
    assert_eq!(format!("{:.2}", spiral.0[3].r().unwrap()), "341.11");
    assert_eq!(format!("{:.2}", spiral.0[4].r().unwrap()), "380.16");
    assert_eq!(format!("{:.2}", spiral.0[5].r().unwrap()), "442.95");
    assert_eq!(format!("{:.2}", spiral.0[6].r().unwrap()), "548.50");
    assert_eq!(format!("{:.2}", spiral.0[7].r().unwrap()), "746.83");
    assert_eq!(format!("{:.0}", spiral.0[8].r().unwrap()), "1222");
    assert_eq!(format!("{:.0}", spiral.0[9].r().unwrap()), "3633");
    assert_eq!(format!("{:.0}", spiral.0[10].r().unwrap()), "3633");
    assert_eq!(format!("{:.0}", spiral.0[11].r().unwrap()), "1222");
}

#[test]
fn 反向緩和曲線長19_5m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(Some(300.0)),
        Radius(Some(-300.0)),
        19.5,
        0.0.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 20);
    assert_eq!(format!("{:.2}", spiral.0[0].r().unwrap()), "300.98");
    assert_eq!(format!("{:.2}", spiral.0[1].r().unwrap()), "308.98");
    assert_eq!(format!("{:.2}", spiral.0[2].r().unwrap()), "326.09");
    assert_eq!(format!("{:.2}", spiral.0[3].r().unwrap()), "354.95");
    assert_eq!(format!("{:.2}", spiral.0[4].r().unwrap()), "400.80");
    assert_eq!(format!("{:.2}", spiral.0[5].r().unwrap()), "474.35");
    assert_eq!(format!("{:.2}", spiral.0[6].r().unwrap()), "600.00");
    assert_eq!(format!("{:.2}", spiral.0[7].r().unwrap()), "846.01");
    assert_eq!(format!("{:.0}", spiral.0[8].r().unwrap()), "1500");
    assert_eq!(format!("{:.0}", spiral.0[9].r().unwrap()), "7450");
    assert_eq!(format!("{:.0}", spiral.0[10].r().unwrap()), "2489");
    assert_eq!(format!("{:.2}", spiral.0[19].r().unwrap()), "300.24");
}

impl Stroke {
    fn r(&self) -> Option<f64> {
        match self {
            Self::Arc(_, r, _, _) => Some(*r),
            _ => None,
        }
    }
}
