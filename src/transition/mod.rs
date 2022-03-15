pub mod formula;
pub mod param;
pub mod spiral;

pub use param::Param;

mod segment;
#[cfg(test)]
mod test;

use spiral::{Line, Spiral};

use self::segment::Segmentation;

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Spiral {
    // 区間に分割する。
    Segmentation::new(param.l0, param.tcl)
        .map(|segment| {
            // 区間の曲率
            let k = param.diminish.k(param.tcl, segment.s(), param.k0, param.k1);
            (k, segment.len())
        })
        .scan((param.p0, param.t0), |(p0, t0), (k, len)| {
            // 区間の線分
            let line = Line::new(*p0, *t0, len, k);
            *p0 = line.p1();
            *t0 = line.t1(*t0);
            Some(line)
        })
        .collect()
}
