pub mod formula;
pub mod param;
pub mod spiral;

pub use param::Param;

mod segment;
#[cfg(test)]
mod test;

use anyhow::Result;
use spiral::Line;

use self::segment::Segmentation;

/// 緩和曲線を描画する。
pub fn plot(param: &Param) -> Result<Vec<Line>> {
    let tcl = param.l1 - param.l0;
    let mut s = Segmentation::new(param.l0, param.l1, &param.p0);
    let v = s
        .map(|segment| {
            let k = param.diminish.k(tcl, segment.s, param.k0, param.k1);
            Line::_Mock
            // if k.is_straight() {
            //     Line::straight(segment.p0, segment.a0, segment.len)
            // } else {
            //     Line::curve(
            //         segment.p0,
            //         k.into(),
            //         segment.a0,
            //         k.angle(segment.len).into(),
            //     )
            // }
        })
        .collect();

    Ok(v)
}
