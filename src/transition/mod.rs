pub mod formula;
pub mod param;
pub mod spiral;

pub use param::Param;

mod segment;
#[cfg(test)]
mod test;

use anyhow::Result;
use segment::Head;
use spiral::{Line, Spiral};

use self::segment::Segmentation;

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Spiral {
    let tcl = param.l1 - param.l0;
    Segmentation::new(param.l0, param.l1, &param.p0)
        .scan(Head::new(&param.p0, param.a0), |head, segment| {
            let k = param.diminish.k(tcl, segment.s, param.k0, param.k1);
            let line = Line::new(head.p0, head.a0, segment.len, k);
            *head += &line;
            Some(line)
        })
        .collect()
}
