use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::{ensure, Result};
use encoding_rs::SHIFT_JIS;

use crate::{dir::Dir, zip::Package};

mod dir;
mod zip;

fn main() -> Result<()> {
    let mut zip = Package::new_in("./外部変形")?;

    // batファイル
    let bat_dir = "./bat";
    for bat_path in Dir::open(bat_dir)?.bat_iter() {
        print!("Encoding {} -> ", bat_path.display());

        // 読み込み
        let mut utf8 = String::new();
        File::open(bat_path)?.read_to_string(&mut utf8)?;

        // 文字列展開
        utf8 = utf8.replace("(VERSION)", env!("CARGO_PKG_VERSION"));

        // 文字コード変換
        let (sjis, _, unmappable) = SHIFT_JIS.encode(&utf8);
        ensure!(!unmappable, "SHIFT_JISに変換できない文字が含まれています");

        // 書き込み
        let mut out = zip.create_file(bat_path.strip_prefix(bat_dir)?)?;
        out.write_all(&sjis[..])?;
        out.flush()?;
    }

    // READMEファイル
    let readme_path = "readme.txt";
    print!("Creating ");
    let mut out_readme = zip.create_file(readme_path)?;
    write!(
        &mut out_readme,
        "f5rail v{}\r\n\r\n",
        // f5railと同じパッケージとしてビルドされている必要がある
        env!("CARGO_PKG_VERSION")
    )?;
    write!(&mut out_readme, "BVE layout tool for Jw_cad.\r\n\r\n")?;
    let mut license = Vec::<u8>::new();
    File::open("./LICENSE")?.read_to_end(&mut license)?;
    out_readme.write_all(&license)?;

    // 実行ファイル
    let out_exe_path = "f5rail.exe";
    let in_exe_path = format!("./target/release/{}", out_exe_path);
    print!("Copying {} -> ", in_exe_path);
    let mut out_exe = zip.create_file(out_exe_path)?;
    for byte in BufReader::new(File::open(in_exe_path)?).bytes() {
        out_exe.write_all(&[byte?])?;
    }

    zip.finish()?;
    println!("Successfully built distributable package.");
    Ok(())
}
