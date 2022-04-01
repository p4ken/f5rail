use std::env;

use anyhow::Result;

use f5rail;

fn main() -> Result<()> {
    let args = env::args_os();
    // dbg!(&args);
    f5rail::layout(args)
}
