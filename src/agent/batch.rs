use std::{collections::HashMap, ffi::OsString, fmt};

use anyhow::{bail, Context, Ok, Result};

pub use crate::transition::param::{Func as TcFunc, Param as TcParam};

/// コマンドライン引数
///
/// BATファイルの起動オプション（参考）
/// https://www.tmk-s.com/jww/bat-file.html#c
#[derive(Debug)]
pub struct Args {
    pub param: Param,
    pub file: String,
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

        let param = Param::parse(&args)?;

        let file = args.get("FILE")?.str().to_owned();

        Ok(Self { param, file })
    }
}

struct ArgMap<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> FromIterator<(&'a str, &'a str)> for ArgMap<'a> {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        Self {
            map: HashMap::from_iter(iter),
        }
    }
}

impl ArgMap<'_> {
    fn get(&self, key: &str) -> Result<ArgValue> {
        let pair = self
            .map
            .get_key_value(key)
            .map(|p| (*p.0, *p.1))
            .with_context(|| format!("{}を指定してください", key))?;
        Ok(ArgValue { pair })
    }
}

struct ArgValue<'a> {
    pair: (&'a str, &'a str),
}

impl ArgValue<'_> {
    fn str(&self) -> &str {
        self.pair.1
    }

    fn f64(&self) -> Result<f64> {
        self.pair
            .1
            .parse()
            .with_context(|| format!("{}を数値で入力してください", self.pair.0))
    }
}

/// 機能パラメータ
#[derive(Debug)]
pub enum Param {
    Transition(Result<TcParam>),
    _Parallel,
    Encode,
}

impl Param {
    fn parse(args: &ArgMap) -> Result<Self> {
        let func = args.get("FUNC")?;
        let func = func.str();
        if let Some(func) = Self::to_tc_func(func) {
            let param = Self::to_tc_param(func, &args);
            Ok(Self::Transition(param))
        } else if func == "sjis" {
            Ok(Self::Encode)
        } else {
            bail!("FUNCの値が間違っています")
        }
    }

    fn to_tc_func(func: &str) -> Option<TcFunc> {
        match func {
            "sin" => Some(TcFunc::Sin),
            "linear" => Some(TcFunc::Linear),
            _ => None,
        }
    }

    fn to_tc_param(func: TcFunc, args: &ArgMap) -> Result<TcParam> {
        let r1 = args
            .get("R1")
            .map_or(Ok(None), |val| val.f64().map(|d| Some(d)))?;

        let r2 = args
            .get("R2")
            .map_or(Ok(None), |val| val.f64().map(|d| Some(d)))?;

        let tcl = args.get("TCL")?.f64()?;

        let dx = args.get("DX").map_or(Ok(0.1), |val| val.f64())?;

        Ok(TcParam {
            func,
            r1,
            r2,
            tcl,
            dx,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unwrap_tc_param(param: &Param) -> &Result<TcParam> {
        match param {
            Param::Transition(tc_param) => tc_param,
            _ => panic!(),
        }
    }

    #[test]
    fn コマンドライン引数をパースできる() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/FUNC:sin"),
            OsString::from("/R1:1"),
            OsString::from("/R2:2.2"),
            OsString::from("/TCL:3"),
            OsString::from("/DX:4"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.file, "./JWC_TEMP.TXT");
        assert!(matches!(args.param, Param::Transition(_)));
        let tc = unwrap_tc_param(&args.param);
        assert!(tc.is_ok());
        let tc = tc.as_ref().unwrap();
        assert!(matches!(tc.func, TcFunc::Sin));
        assert_eq!(tc.r1, Some(1.));
        assert_eq!(tc.r2, Some(2.2));
        assert_eq!(tc.tcl, 3.);
        assert_eq!(tc.dx, 4.);
    }

    #[test]
    fn コマンドライン引数にファイル名がなければエラー() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/FUNC:sin"),
        ];
        let args = Args::parse(args);
        assert!(args.is_err());
        let e = args.unwrap_err();
        assert_eq!(e.to_string(), "FILEを指定してください")
    }

    #[test]
    fn 緩和曲線の長さがなければエラー() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/FUNC:sin"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.file, "./JWC_TEMP.TXT");
        assert!(matches!(args.param, Param::Transition(_)));
        let tc = unwrap_tc_param(&args.param);
        assert!(tc.is_err());
        let e = tc.as_ref().unwrap_err();
        assert_eq!(e.to_string(), "TCLを指定してください");
    }

    #[test]
    fn 緩和曲線の長さ以外は省略可能() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/FUNC:sin"),
            OsString::from("/TCL:3"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.file, "./JWC_TEMP.TXT");
        assert!(matches!(args.param, Param::Transition(_)));
        let tc = unwrap_tc_param(&args.param);
        assert!(tc.is_ok());
        let tc = tc.as_ref().unwrap();
        assert!(matches!(tc.func, TcFunc::Sin));
        assert_eq!(tc.r1, None);
        assert_eq!(tc.r2, None);
        assert_eq!(tc.tcl, 3.);
        assert_eq!(tc.dx, 0.1);
    }

    #[test]
    fn 緩和曲線の半径が文字列ならエラー() {
        let args = vec![
            OsString::from("transition.exe"),
            OsString::from("/FUNC:sin"),
            OsString::from("/R1:abc"),
            OsString::from("/TCL:3"),
            OsString::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let args = Args::parse(args);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.file, "./JWC_TEMP.TXT");
        assert!(matches!(args.param, Param::Transition(_)));
        let tc = unwrap_tc_param(&args.param);
        assert!(tc.is_err());
        let e = tc.as_ref().unwrap_err();
        assert_eq!(e.to_string(), "R1を数値で入力してください");
    }
}
