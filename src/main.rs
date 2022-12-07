use std::io::{BufRead, Write};

fn main() -> anyhow::Result<()> {
    println!();

    let args = std::env::args_os();
    for arg in args.into_iter() {
        println!("{}", arg.to_str().expect("UTF-8への変換に失敗"));
    }

    let sjis = encoded::shift_jis!("日本語");
    dbg!(sjis);

    // let jwc_temp = std::env::var_os("JWC_TEMP").expect("環境変数がありません");
    // println!("{}", jwc_temp.to_string_lossy());

    // 日本語を含むファイルをio::BufReader::linesで読んだら大丈夫か？
    // for line in std::io::BufReader::new(std::fs::File::open("JWC_TEMP.txt")?).lines() {
    //     print!("{}\n", line.unwrap_or("行の読み込みに失敗".to_string()))
    // }

    // rfd::FileDialog::new()
    //     .add_filter("BVEマップファイル", &["txt"])
    //     .save_file();

    return Ok(());

    // dbg!(&args);
    // f5rail::layout(args)
    f5rail::Plugin::cli(args)
}
