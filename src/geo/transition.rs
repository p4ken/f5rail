use crate::jww::{jww_temp::JwwTemp, param::Param, self};

/// JW_CADに緩和曲線を描画する
pub fn draw(param: &Param) {
    // 座標ファイル
    let jww_temp = JwwTemp::new(&param.file);
    plot(&mut jww_temp);
    jww_temp.flush();
}

/// 座標ファイルに緩和曲線を出力する
fn plot(jww_temp: &mut JwwTemp) {
    // match param.func.as_str() {
    //     "sin" => jww_temp.notice("サイン半波長逓減"),
    //     "linear" => jww_temp.notice("直線逓減"),
    //     _ => (),
    // }
}

