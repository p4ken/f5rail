//! Makefileの代わり。
//! RustにすることでWindowsで実行できるようになったが、
//! ファイル操作の開発効率はMakefileのほうが優れている思われる。

use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use anyhow::Result;

use crate::{dir::Dir, make::Make, package::Package};

mod bat;
mod dir;
mod make;
mod package;

fn main() -> Result<()> {
    let mut zip = Package::new_in("./外部変形")?;

    // .batファイル
    for bat in Dir::open("./bat")?.all_bats()? {
        let bat_path = bat.path_str();
        println!("Encoding {}", bat_path);
        let file = &mut zip.create_file(bat_path)?;
        bat.make(file)?;
    }

    // README
    let readme_path = "readme.txt";
    println!("Creating {}", readme_path);
    let mut readme_file = zip.create_file(readme_path)?;
    write!(
        &mut readme_file,
        "f5rail v{}\r\n\r\n",
        // f5railと同じパッケージとしてビルドされている必要がある
        env!("CARGO_PKG_VERSION")
    )?;
    write!(&mut readme_file, "BVE layout tool for Jw_cad.\r\n\r\n")?;
    let mut license = Vec::<u8>::new();
    File::open("./LICENSE")?.read_to_end(&mut license)?;
    readme_file.write_all(&license)?;

    // 実行ファイル
    // Read / Writeが必要
    // let from_path = "./target/release/f5rail.exe";
    // let to_path = out_dir.join(from_path.file_name().unwrap());
    // println!("Copying {} -> {}", from_path.display(), to_path.display());
    // fs::copy(from_path, to_path)?;

    zip.finish()?;
    println!("Successfully built distributable package.");
    Ok(())
}
