use std::{collections::HashMap, ffi::OsStr};

use anyhow::{ensure, Context, Result};

#[derive(Debug)]
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

    pub fn track(&self) -> Result<&str> {
        self.get_str("TRACK")
    }
    pub fn transition(&self) -> Result<&str> {
        self.get_str("TRANSITION")
    }
    pub fn temp_path(&self) -> Result<&str> {
        self.get_str("TEMP")
    }
    pub fn temp_0_path(&self) -> Result<&str> {
        self.get_str("TEMP_0")
    }
    pub fn temp_x_path(&self) -> Result<&str> {
        self.get_str("TEMP_X")
    }
    pub fn map_name(&self) -> &str {
        self.get_str("出力ファイル名").unwrap_or("")
    }
    pub fn r0(&self) -> Result<Option<f64>> {
        self.get_radius("R0")
    }
    pub fn r1(&self) -> Result<Option<f64>> {
        self.get_radius("R1")
    }
    pub fn tcl(&self) -> Result<f64> {
        let tcl = self.get("TCL")?.float()?;
        ensure!(tcl > 0.0, "TCLに0より大きい値を入力してください");
        Ok(tcl.into())
    }

    fn get<'k>(&self, key: &'k str) -> Result<ArgValue<'k, '_>> {
        let value = self
            .buf
            .get(key)
            .with_context(|| format!("{key}を指定してください"))?;
        Ok(ArgValue(key, value))
    }
    fn get_str(&self, key: &str) -> Result<&str> {
        self.get(key).map(|val| val.str())
    }
    fn get_radius(&self, key: &str) -> Result<Option<f64>> {
        // 半径は無くてもよいが、あるなら適切な値でなければならない
        self.get(key).map_or(Ok(None), |val| val.radius())
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
    fn str(&self) -> &'v str {
        self.1
    }
    fn float(&self) -> Result<f64> {
        self.str()
            .parse()
            .with_context(|| format!("{}を数値で入力してください", self.key()))
    }
    fn radius(&self) -> Result<Option<f64>> {
        let r = self.float()?;
        ensure!(r != 0.0, "{}に0を指定できません", self.key());
        Ok(Some(r))
    }

    fn key(&self) -> &'k str {
        self.0
    }
}
