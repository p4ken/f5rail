use std::io;

fn main() {
    let stdin = io::stdin().lock();
    let _ = jwwio::TempFormat::load(stdin);
}
