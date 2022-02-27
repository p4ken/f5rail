pub mod param;
pub mod polyline;

use anyhow::{Result, bail};
use self::param::Param;

/// 緩和曲線を描画する
pub fn draw(param: &Param) -> Result<polyline::Polyline> {
    // plot(jww_temp, param);
    bail!("未実装")
}
