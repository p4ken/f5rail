use f5rail;
use std::env;

fn main() {
    f5rail::boot(env::args_os());
}
