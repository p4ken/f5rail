use std::{collections::HashMap, ffi::OsStr, path::PathBuf};

use anyhow::{bail, ensure, Context, Result};

use crate::transition::{
    self,
    curve::{Curvature, Diminish, Radius, Subtension, STRAIGHT},
};

/// コマンドライン引数
///
/// (参考) BATファイルの起動オプション
/// https://www.tmk-s.com/jww/bat-file.html#c
// TODO: OsString
pub struct Args {
    buf: HashMap<String, String>,
}

impl Args {
    pub fn parse(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<Self> {
        // let args = args.into_iter().map(|s| s.as_ref()).collect::<Vec<_>>();

        let args = args
            .into_iter()
            .map(|os| os.as_ref().to_str().map(str::to_owned))
            .collect::<Option<Vec<String>>>()
            .context("非UTF-8文字は使えません")?;

        let args = args
            .iter()
            .filter_map(|s| s.trim_start_matches('/').split_once(":"))
            .collect::<Self>();

        Ok(args)
    }

    pub fn get<'k>(&self, key: &'k str) -> Result<ArgValue<'k, '_>> {
        let value = self
            .buf
            .get(key)
            .with_context(|| format!("{}を指定してください", key))?;
        Ok(ArgValue(key, value))
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for Args {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        let iter = iter
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()));
        Self {
            buf: HashMap::from_iter(iter),
        }
    }
}

/// ひとつの引数の値
// エラーメッセージ生成用にキーの参照も持っている
pub struct ArgValue<'k, 'v>(&'k str, &'v str);

impl<'k, 'v> ArgValue<'k, 'v> {
    pub fn str(&self) -> &'v str {
        self.1
    }
}

// ---------------------------------------------
// 過去の遺物

impl transition::Param {
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
            .1
            .parse()
            .with_context(|| format!("{}を数値で入力してください", value.0))
    }
}

impl From<ArgValue<'_, '_>> for String {
    fn from(value: ArgValue) -> Self {
        value.1.into()
    }
}

impl From<ArgValue<'_, '_>> for PathBuf {
    fn from(value: ArgValue) -> Self {
        value.1.into()
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
        ensure!(r != Radius(0.0), "{}に0を指定できません", v.0);
        Ok(r.into())
    }
}

impl TryFrom<ArgValue<'_, '_>> for Subtension {
    type Error = anyhow::Error;

    fn try_from(value: ArgValue) -> Result<Self, Self::Error> {
        let tcl: f64 = (&value).try_into()?;
        ensure!(tcl > 0.0, "{}に0より大きい値を入力してください", value.0);
        Ok(tcl.into())
    }
}

impl TryFrom<&ArgValue<'_, '_>> for Diminish {
    type Error = anyhow::Error;
    /// 緩和曲線関数に変換する。
    fn try_from(pair: &ArgValue<'_, '_>) -> Result<Self, Self::Error> {
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
