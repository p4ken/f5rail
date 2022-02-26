use anyhow::Result;
use f5rail;
use std::env;

fn main() -> Result<()> {
    f5rail::layout(env::args_os())
}
