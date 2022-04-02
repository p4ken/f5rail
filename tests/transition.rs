use std::{
    ffi::OsString,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Result};
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;

use rstest::rstest;
use tempfile::{NamedTempFile, TempPath};

#[rstest]
#[case (vec!["heTCLを指定してください"])]
fn transition(#[case] expected: Vec<&str>) -> Result<()> {
    let path = NamedTempFile::new_in("./tests")?.into_temp_path();
    let args = vec![Args::file(&path), Args::transition(1)];
    f5rail::layout(args)?;

    let file = File::open(&path)?;
    let reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(SHIFT_JIS))
        .build(file);
    let reader = BufReader::new(reader);
    let count = reader
        .lines()
        .enumerate()
        .inspect(|(i, line)| {
            let line = line.as_ref().unwrap();
            assert_eq!(expected.get(*i), Some(&line.as_str()), "line {}", i + 1)
        })
        .count();
    assert_eq!(expected.get(count), None, "line {}", count + 1);

    path.close()?;
    Ok(())
}

struct Args;

impl Args {
    fn file(path: &TempPath) -> OsString {
        let mut arg = OsString::from("/FILE:");
        arg.push(path);
        arg
    }
    fn transition(value: i32) -> OsString {
        OsString::from(format!("/TRANSITION:{}", value))
    }
}
