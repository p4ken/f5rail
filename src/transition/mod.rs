pub mod formula;
pub mod param;
pub mod spiral;

mod divider;
#[cfg(test)]
mod test;

use anyhow::Result;
use param::Param;
use spiral::Line;

use self::divider::Divider;

/// 緩和曲線を描画する
pub fn plot(param: &Param) -> Result<Vec<Line>> {
    let tcl = param.l1 - param.l0;
    let mut divider = Divider::new(param.l0, param.l1, &param.p0);
    let v = divider
        .map(|segment| {
            let k = param.diminish.k(tcl, segment.s, param.k0, param.k1);
            if k.is_straight() {
                Line::straight(segment.p0, segment.a0, segment.len)
            } else {
                Line::curve(segment.p0, k.into(), segment.a0, k.angle()/* 曲率と弧長から中心角*/)
            }
        })
        .collect();

    Ok(v)
}
