use anyhow::Result;

#[test]
fn relative() -> Result<()> {
    let args = vec!["/TRACK:N"];
    f5rail::layout(args)?;
    Ok(())
}
