pub mod curve;
pub mod param;
pub mod spiral;

pub use param::Param;

mod distance;
#[cfg(test)]
mod test;

use spiral::{Atom, Spiral};

use self::distance::Segmentation;

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Spiral {
    // 距離程を区間に分割する。
    Segmentation::new(param.l0, param.tcl)
        .map(|segment| {
            // 区間の曲率を計算する。
            let k = param.diminish.k(param.tcl, segment.s(), param.k0, param.k1);

            (k, segment.len())
        })
        .scan((param.p0, param.t0), |(p0, t0), (k, len)| {
            // 区間の図形を作成する。
            let atom = Atom::new(p0, *t0, len, k);

            // 今回の終点を次回の始点にする。
            *p0 = atom.p1();
            *t0 = atom.t1(*t0);

            Some(atom)
        })
        .collect()
}
