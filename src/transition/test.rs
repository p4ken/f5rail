use super::canvas::*;
use super::curve::*;
use super::*;

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(300.0).into(),
        Radius(-300.0).into(),
        19.0.into(),
        0.0.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 19);
    assert_eq!(format!("{:.2}", spiral[0].unwrap_r()), "301.03");
    assert_eq!(format!("{:.2}", spiral[1].unwrap_r()), "309.47");
    assert_eq!(format!("{:.2}", spiral[2].unwrap_r()), "327.59");
    assert_eq!(format!("{:.2}", spiral[3].unwrap_r()), "358.35");
    assert_eq!(format!("{:.2}", spiral[4].unwrap_r()), "407.76");
    assert_eq!(format!("{:.2}", spiral[5].unwrap_r()), "488.43");
    assert_eq!(format!("{:.2}", spiral[6].unwrap_r()), "630.32");
    assert_eq!(format!("{:.2}", spiral[7].unwrap_r()), "923.93");
    assert_eq!(format!("{:.0}", spiral[8].unwrap_r()), "1823");
    assert!(spiral[9].is_straight());
}

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m_始点1m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(300.0).into(),
        Radius(-300.0).into(),
        19.0.into(),
        1.0.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 19);
    assert_eq!(format!("{:.2}", spiral[0].unwrap_r()), "301.03");
    assert_eq!(format!("{:.2}", spiral[1].unwrap_r()), "309.47");
    assert_eq!(format!("{:.2}", spiral[2].unwrap_r()), "327.59");
    assert_eq!(format!("{:.2}", spiral[3].unwrap_r()), "358.35");
    assert_eq!(format!("{:.2}", spiral[4].unwrap_r()), "407.76");
    assert_eq!(format!("{:.2}", spiral[5].unwrap_r()), "488.43");
    assert_eq!(format!("{:.2}", spiral[6].unwrap_r()), "630.32");
    assert_eq!(format!("{:.2}", spiral[7].unwrap_r()), "923.93");
    assert_eq!(format!("{:.0}", spiral[8].unwrap_r()), "1823");
    assert!(spiral[9].is_straight());
}

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m_始点0_5m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(300.0).into(),
        Radius(-300.0).into(),
        19.0.into(),
        0.5.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 20);
    // 0.5m
    assert_eq!(format!("{:.2}", spiral[0].unwrap_r()), "300.26");
    // 1m
    assert_eq!(format!("{:.2}", spiral[1].unwrap_r()), "304.15");
    assert_eq!(format!("{:.2}", spiral[2].unwrap_r()), "317.19");
    assert_eq!(format!("{:.2}", spiral[3].unwrap_r()), "341.11");
    assert_eq!(format!("{:.2}", spiral[4].unwrap_r()), "380.16");
    assert_eq!(format!("{:.2}", spiral[5].unwrap_r()), "442.95");
    assert_eq!(format!("{:.2}", spiral[6].unwrap_r()), "548.50");
    assert_eq!(format!("{:.2}", spiral[7].unwrap_r()), "746.83");
    assert_eq!(format!("{:.0}", spiral[8].unwrap_r()), "1222");
    assert_eq!(format!("{:.0}", spiral[9].unwrap_r()), "3633");
    assert_eq!(format!("{:.0}", spiral[10].unwrap_r()), "-3633");
    assert_eq!(format!("{:.0}", spiral[11].unwrap_r()), "-1222");
}

#[test]
fn 反向緩和曲線長19_5m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(
        Diminish::Sine,
        Radius(300.0).into(),
        Radius(-300.0).into(),
        19.5.into(),
        0.0.into(),
    );
    let spiral = plot(&param);
    assert_eq!(spiral.0.len(), 20);
    assert_eq!(format!("{:.2}", spiral[0].unwrap_r()), "300.98");
    assert_eq!(format!("{:.2}", spiral[1].unwrap_r()), "308.98");
    assert_eq!(format!("{:.2}", spiral[2].unwrap_r()), "326.09");
    assert_eq!(format!("{:.2}", spiral[3].unwrap_r()), "354.95");
    assert_eq!(format!("{:.2}", spiral[4].unwrap_r()), "400.80");
    assert_eq!(format!("{:.2}", spiral[5].unwrap_r()), "474.35");
    assert_eq!(format!("{:.2}", spiral[6].unwrap_r()), "600.00");
    assert_eq!(format!("{:.2}", spiral[7].unwrap_r()), "846.01");
    assert_eq!(format!("{:.0}", spiral[8].unwrap_r()), "1500");
    assert_eq!(format!("{:.0}", spiral[9].unwrap_r()), "7450");
    assert_eq!(format!("{:.0}", spiral[10].unwrap_r()), "-2489");
    assert_eq!(format!("{:.2}", spiral[19].unwrap_r()), "-300.24");
}

impl Stroke {
    fn unwrap_r(&self) -> f64 {
        self.r().unwrap().0
    }
}
