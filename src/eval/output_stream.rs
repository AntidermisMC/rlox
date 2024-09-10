use std::{
    fmt::Write,
    io::{Stdout, Write as WriteIo},
};

pub enum OutputStream {
    File(String),
    StdOut(Stdout),
}

impl Write for OutputStream {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self {
            OutputStream::File(str) => str.write_str(s),
            OutputStream::StdOut(out) => out
                .write(s.as_bytes())
                .map(|_| ())
                .map_err(|_| std::fmt::Error::default()),
        }
    }
}
