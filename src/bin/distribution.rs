use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use anyhow::Result;
use encoding_rs::SHIFT_JIS;

fn main() -> Result<()> {
    let utf8_dir = Path::new("./bat");
    assert!(utf8_dir.exists());

    let sjis_dir = Path::new("./外部変形");
    if !sjis_dir.exists() {
        fs::create_dir(sjis_dir)?;
    }

    for entry in utf8_dir.read_dir()? {
        let utf8_path = entry?.path();
        if utf8_path.extension() != Some(OsStr::new("bat")) {
            continue;
        }
        
        // 外部変形batファイル
        let sjis_path = sjis_dir.join(utf8_path.file_name().unwrap());
        println!(
            "Encoding {} -> {}",
            utf8_path.display(),
            sjis_path.display()
        );

        let mut utf8 = String::new();
        File::open(&utf8_path)?.read_to_string(&mut utf8)?;
        // バージョン表示
        utf8 = utf8.replace("(VERSION)", env!("CARGO_PKG_VERSION"));
        // 文字コード変換
        let (cow, _, _) = SHIFT_JIS.encode(&utf8);
        File::create(&sjis_path)?.write_all(&cow[..])?;
    }

    // README
    let readme_path = sjis_dir.join("readme.txt");
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
    let to_path = sjis_dir.join(from_path.file_name().unwrap());
    println!("Copying {} -> {}", from_path.display(), to_path.display());
    fs::copy(from_path, to_path)?;

    println!("Successfully built distributable package.");
    Ok(())
}
