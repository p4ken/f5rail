use std::collections::HashMap;

use anyhow::{bail, Context, Ok, Result};

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

/// パラメータ
pub struct Param {
    pub func: Func,
    pub file: String,
}

/// 機能
pub enum Func {
    Tc(Result<Tc>),
}

impl Param {
    /// コマンドライン引数をパースする。
    ///
    /// BATファイルの起動オプション（参考）
    /// https://www.tmk-s.com/jww/bat-file.html#c
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Param> {
        let args = args.into_iter().collect::<Vec<String>>();
        let args = args
            .iter()
            .filter_map(|s| s.trim_start_matches(ARG_PREFIX).split_once(ARG_SEPARATOR))
            .collect::<HashMap<&str, &str>>();

        let func = args.get(ARG_KEY_FUNC).context("FUNCを指定してください")?;
        let file = args.get(ARG_KEY_FILE).context("FILEを指定してください")?;

        if let Some(func) = TcFunc::parse(func) {
            Ok(Param {
                func: Func::Tc(Tc::parse(func, &args)),
                file: file.to_owned().to_owned(),
            })
        } else {
            bail!("FUNCの値が不正です")
        }
    }
}

/// 緩和曲線描画パラメータ
pub struct Tc {
    pub func: TcFunc,
    pub r1: Option<i32>,
    pub r2: Option<i32>,
    pub tcl: i32,
    pub dx: f64,
}

impl Tc {
    pub fn parse(func: TcFunc, args: &HashMap<&str, &str>) -> Result<Tc> {
        Ok(Tc {
            func,
            r1: args.get(ARG_KEY_R1).and_then(|s| s.parse().ok()),
            r2: args.get(ARG_KEY_R2).and_then(|s| s.parse().ok()),
            tcl: args
                .get(ARG_KEY_TCL)
                .context("TCLを指定してください")?
                .parse()
                .context("TCLを整数で入力してください")?,
            dx: args
                .get(ARG_KEY_DX)
                .and_then(|s| s.parse().ok())
                .unwrap_or(DX_DEFAULT),
        })
    }
}

/// 緩和曲線関数
pub enum TcFunc {
    Sin,
    Linear,
}

impl TcFunc {
    fn parse(s: &str) -> Option<TcFunc> {
        match s {
            FUNC_SIN => Some(TcFunc::Sin),
            FUNC_LINEAR => Some(TcFunc::Linear),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            TcFunc::Sin => "サイン半波長逓減",
            TcFunc::Linear => "直線逓減",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn パースする() {
        let v = vec![
            String::from("transition.exe"),
            String::from("/FUNC:sin"),
            String::from("/R1:1"),
            String::from("/R2:2"),
            String::from("/TCL:3"),
            String::from("/DX:4"),
            String::from("/FILE:./JWC_TEMP.TXT"),
        ];
        let param = Param::parse(v);
        assert!(param.is_ok());
        let param = param.unwrap();
        assert_eq!(param.file, "./JWC_TEMP.TXT");
        assert!(matches!(param.func, Func::Tc(_)));
        let tc = match param.func {
            Func::Tc(tc) => tc,
            _ => panic!(""),
        };
        assert!(tc.is_ok());
        let tc = tc.unwrap();
        assert!(matches!(tc.func, TcFunc::Sin));
        assert_eq!(tc.r1, Some(1));
        assert_eq!(tc.r2, Some(2));
        assert_eq!(tc.tcl, 3);
        assert_eq!(tc.dx, 4.);
    }
}
