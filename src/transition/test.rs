use rstest::rstest;

use super::canvas::*;
use super::curve::*;
use super::*;

mod 曲率計算 {
    use super::*;

    fn param(tcl: f64, btc: f64) -> Param {
        Param {
            diminish: Diminish::Sine,
            k0: Radius(300.0).into(),
            k1: Radius(-300.0).into(),
            tcl: tcl.into(),
            l0: btc.into(),
            p0: Point(0.0, 0.0),
            t0: 0.0.into(),
        }
    }

    #[rstest]
    #[case(param(19.0, 0.0), 19)]
    #[case(param(19.0, 1.0), 19)]
    #[case(param(19.0, 0.5), 20)]
    #[case(param(19.5, 0.0), 20)]
    fn 区間数(#[case] param: Param, #[case] expected: usize) {
        let spiral = plot(&param);
        assert_eq!(spiral.len(), expected);
    }

    #[rstest]
    #[case(param(19.0, 0.0), vec!["301.03", "309.47", "327.59", "358.35", "407.76", "488.43", "630.32", "923.93", "1823", "", "-1823"])]
    #[case(param(19.0, 1.0), vec!["301.03", "309.47", "327.59", "358.35", "407.76", "488.43", "630.32", "923.93", "1823", "", "-1823"])]
    #[case(param(19.0, 0.5), vec!["300.26", "304.15", "317.19", "341.11", "380.16", "442.95", "548.50", "746.83", "1222", "3633", "-3633"])]
    #[case(param(19.5, 0.0), vec!["300.98", "308.98", "326.09", "354.95", "400.80", "474.35", "600.00", "846.01", "1500", "7450", "-2489"])]
    fn 曲線半径(#[case] param: Param, #[case] expected: Vec<&str>) {
        let spiral = plot(&param);
        spiral
            .into_iter()
            .zip(expected)
            .for_each(|(stroke, expected)| match expected.is_empty() {
                true => assert!(stroke.r().is_none()),
                false => assert_eq!(stroke.r().unwrap(), expected),
            });
    }
}

mod 座標計算 {
    
}

impl PartialEq<&str> for Radius {
    /// ```
    /// assert_eq!(Radius(300.004), "300.00");
    /// assert_ne!(Radius(300.004), "-300.00");
    /// assert_ne!(Radius(-300.004), "300.00");
    /// assert_ne!(Radius(300.005), "300.00");
    /// ```
    fn eq(&self, other: &&str) -> bool {
        let fractional_digit = other
            .split_once(".")
            .map_or(0, |(_, fractional)| fractional.len() as u32);
        let ratio_to_integer = 10_i32.pow(fractional_digit) as f64;
        let rounded = (self.0 * ratio_to_integer).round();
        rounded.to_string() == other.replace(".", "")
    }
}
