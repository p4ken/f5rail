use std::{collections::HashMap, ffi::OsStr};

use anyhow::{Context, Result};



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

    pub fn get<'k>(&self, key: &'k str) -> Result<ArgValue<'k, '_>> {
        let value = self
            .buf
            .get(key)
            .with_context(|| format!("{key}を指定してください"))?;
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

    pub fn key(&self) -> &'k str {
        self.0
    }
}
