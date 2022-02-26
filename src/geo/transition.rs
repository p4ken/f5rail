use anyhow::{Ok, Result};

use crate::jww::{jww_temp::JwwTemp, param::Tc};

/// 座標ファイルに緩和曲線を出力する
pub fn plot(jww_temp: &mut JwwTemp, param: &Result<Tc>) {
    // match param.func.as_str() {
    //     "sin" => jww_temp.notice("サイン半波長逓減"),
    //     "linear" => jww_temp.notice("直線逓減"),
    //     _ => (),
    // }
}
