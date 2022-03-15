pub mod formula;
pub mod param;
pub mod spiral;

pub use param::Param;

mod segment;
#[cfg(test)]
mod test;

use segment::Head;
use spiral::{Line, Spiral};

use self::segment::Segmentation;

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Spiral {
    // 区間に分割する。
    Segmentation::new(param.l0, param.tcl)
        .scan(Head::new(&param.p0, param.t0), |head, segment| {
            // 区間の曲率を計算する。
            let k = param.diminish.k(param.tcl, segment.s(), param.k0, param.k1);

            // 区間の線分を作成する。
            let line = Line::new(head.p0, head.t0, segment.len(), k);

            // 先端の状態を更新する。
            *head += &line;

            // 線分を出力する。
            Some(line)
        })
        .collect()
}
