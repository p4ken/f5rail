use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use anyhow::Result;

use crate::{dir::Dir, make::Make};

mod bat;
mod dir;
mod make;

fn main() -> Result<()> {
    let in_dir = &Dir::open("./bat")?;
    let out_dir = &Dir::create("./外部変形")?;

    // 外部変形batファイル
    for bat in in_dir.bats()? {
        let out = out_dir.join(bat.strip_prefix(in_dir)?);
        println!("Encoding {} -> {}", bat.display(), out.display());
        bat.make(&out)?;
    }

    // README
    let readme_path = out_dir.join("readme.txt");
    println!("Creating {}", readme_path.display());
    let mut readme_file = File::create(readme_path)?;
    write!(
        &mut readme_file,
        "f5rail v{}\r\n\r\n",
        env!("CARGO_PKG_VERSION")
    )?;
    write!(&mut readme_file, "BVE layout tool for Jw_cad.\r\n\r\n")?;
    let mut license = Vec::<u8>::new();
    File::open("./LICENSE")?.read_to_end(&mut license)?;
    readme_file.write_all(&license)?;

    // 実行ファイル
    let from_path = Path::new("./target/release/f5rail.exe");
    let to_path = out_dir.join(from_path.file_name().unwrap());
    println!("Copying {} -> {}", from_path.display(), to_path.display());
    fs::copy(from_path, to_path)?;

    println!("Successfully built distributable package.");
    Ok(())
}
