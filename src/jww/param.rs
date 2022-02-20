use std::array::IntoIter;

use anyhow::Result;

const ARG_PREFIX: &str = "/";
const ARG_SEPARATOR: &str = ":";
const ARG_KEY_FUNC: &str = "FUNC";

/// パラメータ
#[derive(Debug)]
pub enum Param {
    Transition,
}

impl Param {
    /// コマンドライン引数をパースする。
    ///
    /// BATファイルの起動オプション（参考）
    /// https://www.tmk-s.com/jww/bat-file.html#c
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Param> {
        let args = args
            .into_iter()
            .map(|s| s.trim_start_matches(ARG_PREFIX))
            .map(|s| s.split_once(ARG_SEPARATOR))
            .flatten();

        match args.find(|pair| pair.0 == ARG_KEY_FUNC ).1 {
            "sin" => Transition::parse(args.into_iter()),
            _ => Err(),
        }
    }
}

/// 緩和曲線描画パラメータ
pub struct Transition {
    pub func: String,
    pub r1: Option<i32>,
    pub r2: Option<i32>,
    pub tcl: i32,
    pub dx: i32,
    pub file: String,
}

impl Transition {
    pub fn parse(args:impl IntoIterator) -> Result<Transition> {
        
    }
}

/// 緩和曲線関数
pub enum Func {
    Sin,
    Linear,
}

impl Func {
    pub fn to_string(&self) -> &str {
        match self {
            Sin => "サイン半波長逓減",
            Linear => "直線逓減",
        }
    }
}

#[test]
fn パースする() {
    let v = vec![
        String::from("transition.exe"),
        String::from("/FUNC:sin"),
        String::from("/FILE:./JWC_TEMP.TXT"),
    ];
    dbg!(&v);
    let param = Param::parse(v);
    assert_eq!(param.func, "sin");
    dbg!(&param);
}
