pub mod app;
pub mod canvas;
pub mod curve;
pub mod distance;
pub mod param;
pub mod unit;

#[cfg(test)]
mod test;

use canvas::{Spiral, Stroke};
use distance::Ruler;
use param::Param;

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Spiral {
    // 距離程を区間に分割する。
    Ruler::new(param.l0, param.tcl)
        .map(|interval| {
            // 曲率を計算する。
            let k = param
                .diminish
                .k(param.tcl, interval.s(), param.k0, param.k1);

            // 円弧
            (k, interval.len())
        })
        .scan((param.p0, param.t0), |state, arc| {
            // 線を描画する。
            let stroke = Stroke::new(arc.0, arc.1, state.0, state.1);

            // 始点の状態を更新する。
            *state = (stroke.p1(), stroke.t1());

            Some(stroke)
        })
        .collect()
}
