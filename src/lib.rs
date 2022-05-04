mod agent;
mod transition;

use std::ffi::OsStr;

use anyhow::Result;

use agent::{
    bat::{Args, TrackArgs},
    bve::{MapFile, MapPath},
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
    // temp以外のエラーをtempに出力する層がほしい
    let jwc_temp = jwc_temp::read(&args.temp)?;
    let map_path = MapPath::new(&args.map);
    let map_path = match map_path.absolute() {
        Some(map_path) => map_path.to_path_buf(),
        None => map_path.relative(&jwc_temp.project_dir()?),
    };
    let _map = MapFile::create(&map_path)?;

    Ok(())
}
