use std::env;

mod jww;
use jww::param::Param;

fn main() {
    let args = env::args();
    dbg!(&args);
    let args = Param::parse(args);
    dbg!(&args);
    println!("h#サイン半波長逓減");
    println!("pl");
    println!("0 0");
    println!("100 -100");
    println!("200 -400");
    println!("300 -900");
    println!("#");
}
