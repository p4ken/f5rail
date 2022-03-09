pub mod formula;
pub mod param;
pub mod xy;

#[cfg(test)]
mod test;

use anyhow::{Result};
use param::Param;
use xy::Polyline;

/// 緩和曲線を描画する
pub fn plot(param: &Param) -> Result<Polyline> {
    let tcl = param.tcl; // 緩和曲線長
    let s = 1.; // ※分割数にしたほうが良いかも
    let size = (tcl / s) as usize + 1; // 分割数 ※切り捨て
    let mut v = Vec::with_capacity(size);
    // polyline.push(*p0);
    let mut p = param.p0; // 緩和曲線上の点

    for i in 1..size {
        let l = s * (i as f64); // p0からpまでの弧長
        let k = param.spiral.k(s); // pの曲率
        if k.is_straight() {
            {}// 直線
        }
        // let c =  // 弦長  
        // let vc =  // 弦ベクトル
        // // 終点p
        // let p = 
        // s += ds;
    }

    Ok(v)
}
