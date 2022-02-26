use anyhow::{Ok, Result};

use crate::jww::{jww_temp::JwwTemp, param::Tc};

/// JW_CADに緩和曲線を描画する
pub fn draw(jww_temp: &mut JwwTemp, param: &Result<Tc>) -> Result<()> {
    plot(jww_temp);
    jww_temp.flush();
    Ok(())
}

/// 座標ファイルに緩和曲線を出力する
fn plot(jww_temp: &mut JwwTemp) {
    // match param.func.as_str() {
    //     "sin" => jww_temp.notice("サイン半波長逓減"),
    //     "linear" => jww_temp.notice("直線逓減"),
    //     _ => (),
    // }
}
