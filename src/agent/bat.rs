use std::result::Result::Ok;
use std::{collections::HashMap, ffi::OsString};

use anyhow::{bail, ensure, Context, Result};

use crate::transition;
use crate::transition::curve::{Curvature, Diminish, Radius, Subtension, STRAIGHT};

/// コマンドライン引数
///
/// BATファイルの起動オプション（参考）
/// https://www.tmk-s.com/jww/bat-file.html#c
#[derive(Debug)]
pub enum Args {
    /// 緩和曲線
    Transition(String, Result<transition::Param>),

    /// 他線座標
    _Parallel,
}

impl Args {
    /// コマンドライン引数をパースする。
    pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Self> {
        let args = args
            .into_iter()
            .filter_map(|os| os.into_string().ok())
            .collect::<Vec<_>>();

        let args = args
            .iter()
            .filter_map(|s| s.trim_start_matches('/').split_once(":"))
            .collect::<ArgMap>();

        if let Ok(formula) = args.get("TRANSITION") {
            let file = args.get("FILE")?.as_str().to_owned();
            let param = transition::Param::parse(&formula, &args);
            Ok(Self::Transition(file, param))
        } else {
            bail!("機能を指定してください")
        }
    }
}

impl transition::Param {
    /// コマンドライン引数を緩和曲線パラメータにパースする。
    fn parse(diminish: &ArgValue, args: &ArgMap) -> Result<Self> {
        Ok(Self {
            diminish: diminish.try_into()?,
            // 半径は無くてもOKだが、あるなら適切な値でなければならない。
            k0: args.get("R0").ok().try_into()?,
            k1: args.get("R1").ok().try_into()?,
            l0: 0.0.into(),
            tcl: args.get("TCL")?.try_into()?,
            p0: (0.0, 0.0).into(),
            t0: 0.0.into(),
        })
    }
}

/// 引数の配列
struct ArgMap<'a>(HashMap<&'a str, &'a str>);

impl<'a> FromIterator<(&'a str, &'a str)> for ArgMap<'a> {
    /// イテレータから変換する。
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl<'a> ArgMap<'a> {
    /// 値を取得する。
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

impl<'a> TryFrom<&ArgValue<'a>> for f64 {
    type Error = anyhow::Error;
    /// 小数に変換する。
    fn try_from(value: &ArgValue<'a>) -> Result<Self, Self::Error> {
        value
            .1
            .parse()
            .with_context(|| format!("{}を数値で入力してください", value.0))
    }
}

impl<'a> TryFrom<Option<ArgValue<'a>>> for Curvature {
    type Error = anyhow::Error;
    /// 曲率に変換する。
    fn try_from(value: Option<ArgValue<'a>>) -> Result<Self, Self::Error> {
        let v = match value {
            Some(v) => v,
            None => return Ok(STRAIGHT),
        };
        let r = Radius((&v).try_into()?);
        ensure!(r != Radius(0.0), "{}に0を指定できません", v.0);
        Ok(r.into())
    }
}

impl<'a> TryFrom<ArgValue<'a>> for Subtension {
    type Error = anyhow::Error;
    /// 緩和曲線長に変換する。
    fn try_from(value: ArgValue<'a>) -> Result<Self, Self::Error> {
        let tcl: f64 = (&value).try_into()?;
        ensure!(tcl > 0.0, "{}に0より大きい値を入力してください", value.0);
        Ok(tcl.into())
    }
}

impl<'a> TryFrom<&ArgValue<'a>> for Diminish {
    type Error = anyhow::Error;
    /// 緩和曲線関数に変換する。
    fn try_from(pair: &ArgValue<'a>) -> Result<Self, Self::Error> {
        match pair.1 {
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
