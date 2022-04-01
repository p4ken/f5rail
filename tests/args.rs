use std::ffi::OsString;

use anyhow::Result;
use f5rail;
use tempfile::NamedTempFile;

#[test]
fn encode() -> Result<()> {
    let file = NamedTempFile::new_in("./tests")?;
    let file = file.into_temp_path();
    println!("TempPath: {}", file.display());
    let mut arg_file = OsString::from("/FILE:");
    arg_file.push(&file);
    let args = vec![arg_file, "/TRANSITION:1".into()];
    f5rail::layout(args)?;

    // todo

    file.close()?;
    Ok(())
}
