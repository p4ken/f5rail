use std::f64::consts::FRAC_2_PI;

use rstest::rstest;

use crate::transition::unit::XY;

use super::canvas::Point;
use super::curve::*;
use super::param::Param;
use super::unit::Deg;
use super::*;

fn 緩和曲線(r0 :f64, r1: f64, tcl: f64, btc: f64) -> Param {
    Param {
        diminish: Diminish::Sine,
        k0: Radius(r0).into(),
        k1: Radius(r1).into(),
        tcl: tcl.into(),
        l0: btc.into(),
        p0: (0.0, 0.0).into(),
        t0: 0.0.into(),
    }
}

fn 円(r: f64, c: (f64, f64)) -> Param {
    Param {
        diminish: Diminish::Linear,
        k0: Radius(r).into(),
        k1: Radius(r).into(),
        tcl: 4.0.into(),
        l0: 0.0.into(),
        p0: c.into(),
        t0: Degree(90.0).into(),
    }
}

fn 直線(t0: f64) -> Param {
    Param {
        diminish: Diminish::Linear,
        k0: STRAIGHT,
        k1: STRAIGHT,
        tcl: 2.0.into(),
        l0: 0.0.into(),
        p0: (0.0, 0.0).into(),
        t0: Degree(t0).into(),
    }
}

#[rstest]
#[case(緩和曲線(300.0, -300.0, 19.0, 0.0), 19)]
#[case(緩和曲線(300.0, -300.0, 19.0, 1.0), 19)]
#[case(緩和曲線(300.0, -300.0, 19.0, 0.5), 20)]
#[case(緩和曲線(300.0, -300.0, 19.5, 0.0), 20)]
#[case(緩和曲線(30000.0, -30000.0, 19.5, 0.0), 20)]
fn 区間数(#[case] param: Param, #[case] expected: usize) {
    let spiral = plot(&param);
    assert_eq!(spiral.len(), expected);
}

#[rstest]
#[case(緩和曲線(300.0, -300.0, 19.0, 0.0), vec!["301.03", "309.47", "327.59", "358.35", "407.76", "488.43", "630.32", "923.93", "1823", "", "-1823"])]
#[case(緩和曲線(300.0, -300.0, 19.0, 1.0), vec!["301.03", "309.47", "327.59", "358.35", "407.76", "488.43", "630.32", "923.93", "1823", "", "-1823"])]
#[case(緩和曲線(300.0, -300.0, 19.0, 0.5), vec!["300.26", "304.15", "317.19", "341.11", "380.16", "442.95", "548.50", "746.83", "1222", "3633", "-3633"])]
#[case(緩和曲線(300.0, -300.0, 19.5, 0.0), vec!["300.98", "308.98", "326.09", "354.95", "400.80", "474.35", "600.00", "846.01", "1500", "7450", "-2489"])]
#[case(緩和曲線(30000.0, -30000.0, 19.0, 0.0), vec!["30103", "30947", "32759", "35835", "40776", "48843", "63032", "92393"])]
#[case(円(500.0, (0.0, 0.0)), vec!["500"])]
#[case(円(-500.0, (0.0, 0.0)), vec!["-500"])]
#[case(直線(90.0), vec![""])]
fn 半径(#[case] param: Param, #[case] expected: Vec<&str>) {
    plot(&param)
        .iter()
        .zip(expected)
        .for_each(|(stroke, expected)| match expected.is_empty() {
            true => assert!(stroke.r().is_none()),
            false => assert_eq!(stroke.r().unwrap(), expected),
        });
}

#[rstest]
#[case(円(-FRAC_2_PI, (FRAC_2_PI, 0.0)), vec![180.0, 270.0, 360.0, 450.0])]
#[case(円(-FRAC_2_PI * 10.0, (FRAC_2_PI * 10.0, 0.0)), vec![99.0, 108.0, 117.0, 126.0, 135.0])]
// #[case(円(-FRAC_2_PI * 1000.0, (FRAC_2_PI * 1000.0, 0.0)), vec![90.09, 90.18])]
#[case(直線(90.0), vec![90.0, 90.0])]
#[case(直線(-100.0), vec![-100.0, -100.0])]
fn 接線方向(#[case] param: Param, #[case] expected: Vec<f64>) {
    plot(&param)
        .iter()
        .zip(expected)
        .for_each(|(stroke, expected)| {
            assert_eq!(stroke.t1().deg(), expected);
        });
}

#[rstest]
#[case(円(-FRAC_2_PI, (FRAC_2_PI, 0.0)), ("0.00000", "0.00000"))]
fn 中心点(#[case] param: Param, #[case] expected: (&str, &str)) {
    plot(&param).iter().for_each(|stroke| {
        assert_eq!(stroke.center().unwrap(), expected);
    });
}

#[rstest]
#[case(円(-FRAC_2_PI, (FRAC_2_PI, 0.0)), vec![("0.63662", "0.00000"), ("0.00000", "0.63662"), ("-0.63662", "0.00000"), ("0.00000", "-0.63662"), ("0.63662", "0.00000")])]
#[case(円(FRAC_2_PI, (-FRAC_2_PI, 0.0)), vec![("-0.63662", "0.00000"), ("0.00000", "0.63662"), ("0.63662", "0.00000"), ("0.00000", "-0.63662"), ("-0.63662", "0.00000")])]
#[case(直線(90.0), vec![("0.00000", "0.00000"), ("0.00000", "1.00000"), ("0.00000", "2.00000")])]
fn 座標(#[case] param: Param, #[case] expected: Vec<(&str, &str)>) {
    plot(&param).iter().enumerate().for_each(|(i, stroke)| {
        assert_eq!(stroke.p0(), expected[i]);
        assert_eq!(stroke.p1(), expected[i + 1]);
    });
}

impl PartialEq<&str> for Radius {
    fn eq(&self, other: &&str) -> bool {
        let fractional_digit = other
            .split_once(".")
            .map_or(0, |(_, fractional)| fractional.len() as u32);
        let ratio_to_integer = 10_i32.pow(fractional_digit) as f64;
        let rounded = (self.0 * ratio_to_integer).round();
        rounded.to_string() == other.replace(".", "")
    }
}

#[test]
fn radius_eq_test() {
    assert_eq!(Radius(300.004), "300.00");
    assert_ne!(Radius(300.004), "-300.00");
    assert_ne!(Radius(-300.004), "300.00");
    assert_ne!(Radius(300.005), "300.00");
}

impl From<Degree> for Tangential {
    fn from(degree: Degree) -> Self {
        degree.0.to_radians().into()
    }
}

impl PartialEq<(&str, &str)> for Point {
    fn eq(&self, rhs: &(&str, &str)) -> bool {
        [self.x(), self.y()]
            .into_iter()
            .map(|f| format!("{:.5}", f).replace("-0.00000", "0.00000"))
            .zip([rhs.0, rhs.1])
            .all(|(lhs, rhs)| lhs == rhs)
    }
}
