use std::result::Result::Ok;
use std::{collections::HashMap, ffi::OsString};

use anyhow::{bail, ensure, Context, Result};

use crate::transition;
use crate::transition::formula::{Diminish, Radius};

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

    /// 文字コード
    Encode(String),
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
            .filter_map(|s| s.trim_start_matches("/").split_once(":"))
            .collect::<ArgMap>();

        if let Some(formula) = args.get("TRANSITION").ok() {
            let file = args.get("FILE")?.as_str().to_owned();
            let param = transition::Param::parse(&formula, &args);
            Ok(Self::Transition(file, param))
        } else if let Some(encode) = args.get("ENCODE").ok() {
            Ok(Self::Encode(encode.as_str().to_owned()))
        } else {
            bail!("機能を指定してください")
        }
    }
}

impl transition::Param {
    /// コマンドライン引数を緩和曲線パラメータにパースする。
    fn parse(diminish: &ArgValue, args: &ArgMap) -> Result<Self> {
        let tcl: f64 = args.get("TCL")?.try_into()?;
        ensure!(tcl.is_sign_positive(), "TCLに正の数を入力してください");

        Ok(Self::new(
            diminish.try_into()?,
            args.get("R0").ok().try_into()?,
            args.get("R1").ok().try_into()?,
            tcl,
            0.0,
        ))
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

impl<'a> TryFrom<ArgValue<'a>> for f64 {
    type Error = anyhow::Error;
    /// 小数に変換する。
    fn try_from(value: ArgValue<'a>) -> Result<Self, Self::Error> {
        value
            .1
            .parse()
            .with_context(|| format!("{}を数値で入力してください", value.0))
    }
}

impl<'a> TryFrom<Option<ArgValue<'a>>> for Radius {
    type Error = anyhow::Error;
    /// 半径に変換する。
    fn try_from(value: Option<ArgValue<'a>>) -> Result<Self, Self::Error> {
        let r = match value {
            Some(v) => Some(v.try_into()?),
            None => None,
        };
        Ok(Radius(r))
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