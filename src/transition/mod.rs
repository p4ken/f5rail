pub mod canvas;
pub mod curve;
pub mod param;

pub use param::Param;

mod distance;
#[cfg(test)]
mod test;
mod unit;

use canvas::{Spiral, Stroke};
use curve::Arc;
use distance::Ruler;

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Spiral {
    // 距離程を区間に分割する。
    Ruler::new(param.l0, param.tcl)
        .map(|interval| {
            // 曲率を計算する。
            let k = param
                .diminish
                .k(param.tcl, interval.s(), param.k.0, param.k.1);

            // 円弧
            Arc::new(k, interval.len())
        })
        .scan(param.t0, |t0, arc| {
            // 線を描画する。
            let stroke = Stroke::new(arc, *t0);

            // 接線を更新する。
            *t0 = stroke.t1();

            Some(stroke)
        })
        .collect()
}
