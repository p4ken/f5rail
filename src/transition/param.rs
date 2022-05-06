use std::path::PathBuf;

use crate::{
    agent::bat::{ArgValue, Args},
    transition::curve::{Radius, STRAIGHT},
};

use anyhow::{bail, ensure, Context, Result};

use super::{
    canvas::Point,
    curve::{Curvature, Diminish, Subtension, Tangential},
    distance::Distance,
};

/// 緩和曲線パラメータ
#[derive(Debug)]
pub struct Param {
    /// 逓減関数
    pub diminish: Diminish,

    /// 始点の曲率
    pub k0: Curvature,

    /// 終点の曲率
    pub k1: Curvature,

    /// 始点の距離程
    pub l0: Distance<f64>,

    /// 緩和曲線長
    pub tcl: Subtension,

    /// 始点の座標
    pub p0: Point,

    /// 始点の接線方向
    pub t0: Tangential,
}

impl Param {
    /// コマンドライン引数を緩和曲線パラメータにパースする。
    pub fn parse(diminish: &ArgValue, args: &Args) -> Result<Self> {
        Ok(Self {
            diminish: diminish.try_into()?,
            // 半径は無くてもよいが、あるなら適切な値でなければならない。
            k0: args.get("R0").ok().try_into()?,
            k1: args.get("R1").ok().try_into()?,
            l0: 0.0.into(),
            tcl: args.get("TCL")?.try_into()?,
            p0: (0.0, 0.0).into(),
            t0: 0.0.into(),
        })
    }
}

impl TryFrom<&ArgValue<'_, '_>> for f64 {
    type Error = anyhow::Error;
    /// 小数に変換する。
    fn try_from(value: &ArgValue) -> Result<Self, Self::Error> {
        value
            .str()
            .parse()
            .with_context(|| format!("{}を数値で入力してください", value.key()))
    }
}

impl From<ArgValue<'_, '_>> for String {
    fn from(value: ArgValue) -> Self {
        value.str().into()
    }
}

impl From<ArgValue<'_, '_>> for PathBuf {
    fn from(value: ArgValue) -> Self {
        value.str().into()
    }
}

impl TryFrom<Option<ArgValue<'_, '_>>> for Curvature {
    type Error = anyhow::Error;

    fn try_from(value: Option<ArgValue>) -> Result<Self, Self::Error> {
        let v = match value {
            Some(v) => v,
            None => return Ok(STRAIGHT),
        };
        let r = Radius((&v).try_into()?);
        ensure!(r != Radius(0.0), "{}に0を指定できません", v.key());
        Ok(r.into())
    }
}

impl TryFrom<ArgValue<'_, '_>> for Subtension {
    type Error = anyhow::Error;

    fn try_from(value: ArgValue) -> Result<Self, Self::Error> {
        let tcl: f64 = (&value).try_into()?;
        ensure!(
            tcl > 0.0,
            "{}に0より大きい値を入力してください",
            value.key()
        );
        Ok(tcl.into())
    }
}

impl TryFrom<&ArgValue<'_, '_>> for Diminish {
    type Error = anyhow::Error;
    /// 緩和曲線関数に変換する。
    fn try_from(pair: &ArgValue<'_, '_>) -> Result<Self, Self::Error> {
        match pair.str() {
            "1" => Ok(Diminish::Sine),
            "2" => Ok(Diminish::Linear),
            _ => bail!("緩和曲線関数に正しい値を入力してください"),
        }
    }
}

impl From<Radius> for Curvature {
    fn from(r: Radius) -> Self {
        r.0.recip().into()
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use anyhow::Result;
    use rstest::rstest;

    use crate::transition::{
        self,
        curve::{Curvature, Diminish, Radius, STRAIGHT},
    };

    use super::*;

    #[test]
    fn コマンドライン引数をパースできる() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/TRANSITION:1"),
            OsString::from("/R0:1.1"),
            OsString::from("/R1:2"),
            OsString::from("/TCL:3"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        let args = args.unwrap();
        let transition = args.unwrap_transition();
        assert_eq!(transition.0, "./JWC_TEMP.TXT");
        let param = transition.1.as_ref().unwrap();
        assert!(matches!(param.diminish, Diminish::Sine));
        assert_eq!(param.k0.r(), Some(Radius(1.1)));
        assert_eq!(param.k1.r(), Some(Radius(2.0)));
        assert_eq!(param.tcl, 3.0.into());
    }

    #[test]
    fn 緩和曲線の長さ以外は省略可能() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/TRANSITION:1"),
            OsString::from("/TCL:3"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        let args = args.unwrap();
        let transition = args.unwrap_transition();
        assert_eq!(transition.0, "./JWC_TEMP.TXT");
        let param = transition.1.as_ref().unwrap();
        assert!(matches!(param.diminish, Diminish::Sine));
        assert!(param.k0.is_straight());
        assert!(param.k1.is_straight());
        assert_eq!(param.tcl, 3.0.into());
    }

    #[rstest]
    #[case("/TRANSITION:0",Err(anyhow::anyhow!("緩和曲線関数に正しい値を入力してください")))]
    #[case("/TRANSITION:2", Ok(Diminish::Linear))]
    fn 緩和曲線関数をパースする(#[case] arg: &str, #[case] expected: Result<Diminish>) {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from(arg),
            OsString::from("/TCL:3"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let (_, param) = Args::parse(args).unwrap().unwrap_transition();
        match (param, expected) {
            (Ok(param), Ok(_expected)) => assert!(matches!(param.diminish, _expected)),
            (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
            _ => panic!(),
        }
    }

    #[rstest]
    #[case("", "TCLを指定してください")]
    #[case("/TCL:0", "TCLに0より大きい値を入力してください")]
    #[case("/TCL:-1", "TCLに0より大きい値を入力してください")]
    fn 緩和曲線の長さのエラーチェック(#[case] arg: &str, #[case] err: &str) {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/TRANSITION:1"),
            OsString::from(arg),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        let (_, param) = args.unwrap().unwrap_transition();
        let e = param.as_ref().unwrap_err();
        assert_eq!(e.to_string(), err);
    }

    #[rstest]
    #[case("/R0:abc", "R0を数値で入力してください")]
    #[case("/R1:abc", "R1を数値で入力してください")]
    #[case("/R0:0", "R0に0を指定できません")]
    #[case("/R1:0", "R1に0を指定できません")]
    fn 緩和曲線の半径のエラーチェック(#[case] arg: &str, #[case] err: &str) {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/TRANSITION:1"),
            OsString::from(arg),
            OsString::from("/TCL:3"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        let args = args.unwrap();
        let transition = args.unwrap_transition();
        assert_eq!(transition.0, "./JWC_TEMP.TXT");
        let e = transition.1.as_ref().unwrap_err();
        assert_eq!(e.to_string(), err);
    }

    impl Args {
        fn unwrap_transition(&self) -> (String, Result<transition::Param>) {
            if let Ok(formula) = self.get("TRANSITION") {
                let file = self.get("FILE").unwrap().into();
                let param = Param::parse(&formula, &self);
                (file, param)
            } else {
                panic!("This is not a transition.")
            }
        }
    }

    impl Curvature {
        fn is_straight(&self) -> bool {
            *self == STRAIGHT
        }
    }
}
