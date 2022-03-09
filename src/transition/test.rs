use super::*;
use super::formula::*;

#[test]
fn 反向緩和曲線長19m_開始半径300m_終了半径マイナス300m() {
    let param = Param::new(Spiral::Sine, Some(300.), Some(-300.), 20.);
    let polyline = plot(&param);
    assert!(polyline.is_ok());
    let v = polyline.unwrap();
}
