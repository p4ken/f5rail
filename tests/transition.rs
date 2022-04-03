use std::io::Result as IoResult;
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use anyhow::Result;
use derive_more::IntoIterator;
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;

use rstest::rstest;
use tempfile::NamedTempFile;

#[rstest]
#[case(vec!["/TRANSITION:1"], vec!["heTCLを指定してください"])]
#[case(vec!["/TRANSITION:1", "/R1:-123.4", "/TCL:1"],
       vec!["h#サイン半波長逓減曲線を描画しました。",
            "ci 0.00000000000001511214150147834 246.8 246.8 -90 -89.76784530181085"])]
fn transition(#[case] strv: Vec<&str>, #[case] expected: Vec<&str>) -> Result<()> {
    let path = NamedTempFile::new_in("./tests")?.into_temp_path();
    f5rail::layout(Args::new(&path, &strv))?;

    let file = File::open(&path)?;
    let reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(SHIFT_JIS))
        .build(file);
    let reader = BufReader::new(reader);
    let lines = reader.lines().into_iter().collect::<IoResult<Vec<_>>>()?;
    let lines = lines
        .iter()
        .map(AsRef::as_ref)
        .map(Option::Some)
        .chain(iter::once(None));
    let expects = expected
        .into_iter()
        .map(Option::Some)
        .chain(iter::once(None));
    lines
        .zip(expects)
        .enumerate()
        .for_each(|(i, (line, expect))| assert_eq!(line, expect, "line {}", i + 1));

    path.close()?;
    Ok(())
}

#[derive(IntoIterator)]
struct Args(Vec<OsString>);

impl Args {
    fn new(path: &impl AsRef<OsStr>, strv: &Vec<impl AsRef<OsStr>>) -> Self {
        let mut arg_file = OsString::from("/FILE:");
        arg_file.push(path);
        let arg_file = arg_file;
        let args = strv
            .into_iter()
            .map(|s| s.into())
            .chain(iter::once(arg_file))
            .collect();
        Self(args)
    }
}
