mod agent;
mod transition;

use std::ffi::OsStr;

use anyhow::{ensure, Result};

use agent::{
    bat::{Args, TrackArgs},
    bve::MapFile,
    jwc_temp,
};

/// 配線する
pub fn layout(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let args = Args::parse(args)?;

    match &args {
        Args::Transition(file, args) => plot(file, args),
        Args::Track(args) => export(args),
    }
}

/// 緩和曲線を描画する
fn plot(file: &str, param: &Result<transition::Param>) -> Result<()> {
    let mut jwc_temp = jwc_temp::create(file)?;
    match param {
        Ok(p) => {
            let spiral = transition::plot(p);
            jwc_temp.diminish(p.diminish)?.spiral(&spiral)
        }
        Err(e) => jwc_temp.error(&e),
    }
}

/// BVEマップに出力する
fn export(args: &TrackArgs) -> Result<()> {
    // TODO: temp_xに記載されたパスに書き出す
    // temp_n以外のエラーをtemp_nに出力する層がほしい
    let jwc_temp = jwc_temp::read(&args.temp)?;

    // TODO: 絶対パスならappendしない
    let mut map_path = jwc_temp.project_dir()?;
    map_path.push(args.map.as_str());

    let map = MapFile::create(&map_path)?;

    Ok(())
}
