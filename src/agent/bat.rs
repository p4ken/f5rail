use std::{collections::HashMap, ffi::OsString};

use anyhow::{bail, Context, Ok, Result};

use crate::transition::formula::Spiral;
use crate::transition::param::Param as TcParam;

/// コマンドライン引数
///
/// BATファイルの起動オプション（参考）
/// https://www.tmk-s.com/jww/bat-file.html#c
#[derive(Debug)]
pub enum Args {
    /// 緩和曲線
    Transition(Transition),

    /// 他線座標
    _Parallel,

    /// 文字コード
    Encode(String),
}

impl Args {
    /// コマンドライン引数をパースする
    pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Self> {
        let args = args
            .into_iter()
            .filter_map(|os| os.into_string().ok())
            .collect::<Vec<_>>();

        let args = args
            .iter()
            .filter_map(|s| s.trim_start_matches("/").split_once(":"))
            .collect::<ArgMap>();

        if let Some(formula) = args.get("TRANSITION").ok() {
            let file = args.get("FILE")?.as_str().to_owned();
            let param = TcParam::parse(&formula, &args);
            let transition = Transition { file, param };
            Ok(Self::Transition(transition))
        } else if let Some(encode) = args.get("ENCODE").ok() {
            Ok(Self::Encode(encode.as_str().to_owned()))
        } else {
            bail!("機能を指定してください")
        }
    }
}

/// 緩和曲線
#[derive(Debug)]
pub struct Transition {
    /// ファイル名
    pub file: String,

    /// パラメータ
    pub param: Result<TcParam>,
}

impl TcParam {
    /// コマンドライン引数を緩和曲線パラメータにパースする
    fn parse(spiral: &ArgValue, args: &ArgMap) -> Result<Self> {
        let spiral = spiral.try_into()?;

        let r0 = args
            .get("R1")
            .ok()
            .map_or(Ok(None), |v| v.try_into().map(|d| Some(d)))?;

        let r1 = args
            .get("R2")
            .map_or(Ok(None), |v| v.try_into().map(|d| Some(d)))?;

        let tcl = args.get("TCL")?.try_into()?;

        Ok(TcParam::new(spiral, r0, r1, tcl))
    }
}

/// 引数の配列
struct ArgMap<'a>(HashMap<&'a str, &'a str>);

impl<'a> FromIterator<(&'a str, &'a str)> for ArgMap<'a> {
    /// イテレータから変換する
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl<'a> ArgMap<'a> {
    /// 値を取得する
    fn get(&self, key: &'a str) -> Result<ArgValue> {
        let value = self
            .0
            .get(key)
            .with_context(|| format!("{}を指定してください", key))?;
        Ok(ArgValue(key, value))
    }
}

/// 引数の値
struct ArgValue<'a>(&'a str, &'a str);

impl<'a> ArgValue<'a> {
    fn as_str(&self) -> &str {
        self.1
    }
}

impl<'a> TryFrom<ArgValue<'a>> for f64 {
    type Error = anyhow::Error;
    /// 小数に変換する
    fn try_from(value: ArgValue<'a>) -> Result<Self, Self::Error> {
        value
            .1
            .parse()
            .with_context(|| format!("{}を数値で入力してください", value.0))
    }
}

impl<'a> TryFrom<&ArgValue<'a>> for Spiral {
    type Error = anyhow::Error;
    /// 緩和曲線関数に変換する
    fn try_from(pair: &ArgValue<'a>) -> Result<Self, Self::Error> {
        match pair.1 {
            "1" => Ok(Spiral::Sine),
            "2" => Ok(Spiral::Clothoid),
            _ => bail!("緩和曲線関数を指定してください"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn コマンドライン引数をパースできる() {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //         OsString::from("/R1:1"),
    //         OsString::from("/R2:2.2"),
    //         OsString::from("/TCL:3"),
    //         OsString::from("/ds:4"),
    //         OsString::from("/FILE:./JWC_TEMP.TXT"),
    //     ];
    //     let args = Args::parse(args);
    //     assert!(args.is_ok());
    //     let args = args.;
    //     assert!(matches!(args, Args::Transition(_)));
    //     let transition = args.transition();
    //     assert_eq!(transition.file, "./JWC_TEMP.TXT");
    //     let param = transition.param;
    //     assert!(matches!(transition.param., Formula::Sine));
    //     assert_eq!(tc.r0, Some(1.));
    //     assert_eq!(tc.r1, Some(2.2));
    //     assert_eq!(tc.tcl, 3.);
    //     assert_eq!(tc.ds, 4.);
    // }

    // #[test]
    // fn コマンドライン引数にファイル名がなければエラー() {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //     ];
    //     let args = Args::parse(args);
    //     assert!(args.is_err());
    //     let e = args.unwrap_err();
    //     assert_eq!(e.to_string(), "FILEを指定してください")
    // }

    // #[test]
    // fn 緩和曲線の長さがなければエラー() {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //         OsString::from("/FILE:./JWC_TEMP.TXT"),
    //     ];
    //     let args = Args::parse(args);
    //     assert!(args.is_ok());
    //     let args = args.unwrap();
    //     assert_eq!(args.file, "./JWC_TEMP.TXT");
    //     assert!(matches!(args.param, Param::Transition(_)));
    //     let tc = unwrap_transition(&args.param);
    //     assert!(tc.is_err());
    //     let e = tc.as_ref().unwrap_err();
    //     assert_eq!(e.to_string(), "TCLを指定してください");
    // }

    // #[test]
    // fn 緩和曲線の長さ以外は省略可能() {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //         OsString::from("/TCL:3"),
    //         OsString::from("/FILE:./JWC_TEMP.TXT"),
    //     ];
    //     let args = Args::parse(args);
    //     assert!(args.is_ok());
    //     let args = args.unwrap();
    //     assert_eq!(args.file, "./JWC_TEMP.TXT");
    //     assert!(matches!(args.param, Param::Transition(_)));
    //     let tc = unwrap_transition(&args.param);
    //     assert!(tc.is_ok());
    //     let tc = tc.as_ref().unwrap();
    //     assert!(matches!(tc.func, Formula::Sin));
    //     assert_eq!(tc.r0, None);
    //     assert_eq!(tc.r1, None);
    //     assert_eq!(tc.tcl, 3.);
    //     assert_eq!(tc.ds, 0.1);
    // }

    // #[test]
    // fn 緩和曲線の半径が文字列ならエラー() {
    //     let args = vec![
    //         OsString::from("transition.exe"),
    //         OsString::from("/TRANSITION:1"),
    //         OsString::from("/R1:abc"),
    //         OsString::from("/TCL:3"),
    //         OsString::from("/FILE:./JWC_TEMP.TXT"),
    //     ];
    //     let args = Args::parse(args);
    //     assert!(args.is_ok());
    //     let args = args.unwrap();
    //     assert_eq!(args.file, "./JWC_TEMP.TXT");
    //     assert!(matches!(args.param, Param::Transition(_)));
    //     let tc = unwrap_transition(&args.param);
    //     assert!(tc.is_err());
    //     let e = tc.as_ref().unwrap_err();
    //     assert_eq!(e.to_string(), "R1を数値で入力してください");
    // }

    // impl Args {
    //     fn transition(&self) -> &Transition {
    //         match self {
    //             Args::Transition(t) => t,
    //             _ => panic!(),
    //         }
    //     }
    // }
}
