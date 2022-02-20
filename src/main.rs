use std::env;

mod jww;
use jww::param::Param;

mod geo;
use geo::transition;

fn main() {
    let param = Param::parse(env::args());
    transition::draw(&param);
}
