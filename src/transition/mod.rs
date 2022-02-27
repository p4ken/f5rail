pub mod param;
pub mod polyline;

use anyhow::{Result, bail};
use param::Param;
use polyline::Polyline;

/// 緩和曲線を描画する
pub fn plot(param: &Result<Param>) -> Result<Polyline> {
    
    bail!("未実装")
}