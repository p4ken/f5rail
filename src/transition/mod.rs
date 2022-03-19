pub mod curve;
pub mod param;
pub mod canvas;

pub use param::Param;

mod distance;
#[cfg(test)]
mod test;

use distance::Ruler;
use canvas::{Stroke, Spiral};

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Spiral {
    // 距離程を区間に分割する。
    Ruler::new(param.l0, param.tcl)
        .map(|interval| {
            // 区間の曲率を計算する。
            let k = param.diminish.k(param.tcl, interval.s(), param.k0, param.k1);

            (k, interval.len())
        })
        .scan((param.p0, param.t0), |(p0, t0), (k, len)| {
            // 区間の図形を作成する。
            let stroke = Stroke::new(p0, *t0, len, k);

            // 今回の終点を次回の始点にする。
            *p0 = stroke.p1();
            *t0 = stroke.t1(*t0);

            Some(stroke)
        })
        .collect()
}
