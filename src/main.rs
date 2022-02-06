use std::env;

#[derive(Debug)]
struct Param {
    func: String,
}

impl Param {
    pub fn parse(args: &Vec<String>) -> Param {


        Param {
            func: String::from("aaa"),
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let param = Param::parse(&args);
    dbg!(args);
    // println!("h#サイン半波長逓減");
    // println!("pl");
    // println!("0 0");
    // println!("100 -100");
    // println!("200 -400");
    // println!("300 -900");
    // println!("#");
}
