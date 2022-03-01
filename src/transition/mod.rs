pub mod param;
pub mod polyline;

use anyhow::Result;
use param::Param;
use polyline::Polyline;

/// 緩和曲線を描画する
pub fn draw(param: &Param) -> Result<Polyline> {
    let vertex = vec![(0.0001, 0.), (100., 100.), (200., 400.), (300., 900.)];
    Ok(Polyline { vertex })
}
