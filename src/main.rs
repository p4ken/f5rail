use anyhow::Result;
use f5rail;
use std::env;

fn main() -> Result<()> {
    let args = env::args_os();
    // dbg!(&args);
    f5rail::layout(args)
}
