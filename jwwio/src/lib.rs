use std::io::{self, BufRead, BufReader, ErrorKind};

pub struct TempFormat;

impl TempFormat {
    pub fn load(file: impl io::Read) -> io::Result<()> {
        for line in BufRead::lines(BufReader::new(file)) {
            match line {
                Err(e) if e.kind() == ErrorKind::InvalidData => continue, // ignore non-UTF-8
                Err(e) => return Err(e),
                Ok(_) => (), // TODO: parse
            }
        }
        Ok(()) // TODO: return self
    }

    pub fn dump(file: &mut impl io::Write) -> io::Result<()> {

        Ok(())
    }
}
