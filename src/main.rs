fn main() -> anyhow::Result<()> {
    let args = std::env::args_os();
    // dbg!(&args);
    // f5rail::layout(args)
    f5rail::Plugin::cli(args)
}
