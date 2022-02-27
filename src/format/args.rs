use crate::transition::param::{Func as TcFunc, Param as TcParam};
use anyhow::{bail, Context, Ok, Result};
use std::{collections::HashMap, ffi::OsString};

type ArgMap<'a> = HashMap<&'a str, &'a str>;

const ARG_PREFIX: &str = "/";
const ARG_SEPARATOR: &str = ":";
const ARG_KEY_FUNC: &str = "FUNC";
const ARG_KEY_R1: &str = "R1";
const ARG_KEY_R2: &str = "R2";
const ARG_KEY_TCL: &str = "TCL";
const ARG_KEY_DX: &str = "DX";
const ARG_KEY_FILE: &str = "FILE";
const FUNC_SIN: &str = "sin";
const FUNC_LINEAR: &str = "linear";
const DX_DEFAULT: f64 = 0.1;

/// コマンドライン引数
#[derive(Debug)]
pub struct Args {
    pub param: Param,
    pub file: String,
}

impl Args {
    /// コマンドライン引数をパースする。
    ///
    /// BATファイルの起動オプション（参考）
    /// https://www.tmk-s.com/jww/bat-file.html#c
    pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Self> {
        let args = args
            .into_iter()
            .filter_map(|os| os.into_string().ok())
            .collect::<Vec<_>>();

        let args = args
            .iter()
            .filter_map(|s| s.trim_start_matches(ARG_PREFIX).split_once(ARG_SEPARATOR))
            .collect::<ArgMap>();

        let param = Param::parse(&args)?;

        let file = args
            .get(ARG_KEY_FILE)
            .context("FILEを指定してください")?
            .to_owned()
            .to_owned();

        Ok(Self { param, file })
    }
}

/// 機能パラメータ
#[derive(Debug)]
pub enum Param {
    Transition(Result<TcParam>),
    _Relative,
}

impl Param {
    fn parse(args: &ArgMap) -> Result<Self> {
        let func = args.get(ARG_KEY_FUNC).context("FUNCを指定してください")?;
        if let Some(func) = Self::to_tc_func(func) {
            let param = Self::to_tc_param(func, &args);
            Ok(Self::Transition(param))
        } else {
            bail!("FUNCの値が不正です")
        }
    }

    fn to_tc_func(func: &str) -> Option<TcFunc> {
        match func {
            FUNC_SIN => Some(TcFunc::Sin),
            FUNC_LINEAR => Some(TcFunc::Linear),
            _ => None,
        }
    }

    fn to_tc_param(func: TcFunc, args: &ArgMap) -> Result<TcParam> {
        let r1 = args
            .get(ARG_KEY_R1)
            .and_then(|s| Some(s.parse().context("R1を整数で入力してください").ok()?));

        let r2 = args
            .get(ARG_KEY_R2)
            .and_then(|s| Some(s.parse().context("R2を整数で入力してください").ok()?));

        let tcl = args
            .get(ARG_KEY_TCL)
            .context("TCLを指定してください")?
            .parse()
            .context("TCLを整数で入力してください")?;

        let dx = args
            .get(ARG_KEY_DX)
            .and_then(|s| Some(s.parse().context("DXを数値で入力してください").ok()?))
            .unwrap_or(DX_DEFAULT);

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
            OsString::from("/R2:2"),
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
        assert_eq!(tc.r1, Some(1));
        assert_eq!(tc.r2, Some(2));
        assert_eq!(tc.tcl, 3);
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
    fn 緩和曲線の引数に長さがなければエラー() {
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
    fn 緩和曲線の引数の長さ以外は省略可能() {
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
        assert_eq!(tc.tcl, 3);
        assert_eq!(tc.dx, 0.1);
    }
}