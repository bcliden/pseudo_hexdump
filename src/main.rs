mod line_counter;

use line_counter::LineCounter;
use std::char;
use std::env;
use std::error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

const ASCII_PERIOD: u8 = 46;
const LINE_WIDTH: usize = 16;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .map(|p| Path::new(p))
        .ok_or("Invalid path given")?;

    let mut file = File::open(&path)?;
    let mut buf: [u8; LINE_WIDTH] = [0; LINE_WIDTH];
    let mut ln = LineCounter::new(LINE_WIDTH);

    let mut out= io::stdout();
    // let mut out = File::create(Path::new("./out.txt"))?;

    while let Ok(_) = file.read_exact(&mut buf) {
        write!(out, "[{:#08x}] ", ln)?;
        write_line_hex(Box::new(&mut out), &buf);
        write_line_ascii(Box::new(&mut out), &buf);
        write!(out, "\n")?;
        ln.increment();
    }
    Ok(())
}

fn write_line_hex(out: Box<&mut dyn Write>, buf: &[u8]) {
    for (index, byte) in buf.bytes().enumerate() {
        let byte = byte.unwrap_or(ASCII_PERIOD);
        write!(*out, "{:02x} ", byte).ok();

        // if we've printed 4 bytes, add a gutter
        if (index + 1) % 4 == 0 {
            write!(*out, " ").ok();
        };
    }
}

fn is_crlf(c: char) -> bool {
    c == 0x0a as char || c == 0x0d as char
}

fn write_line_ascii(out: Box<&mut dyn Write>, buf: &[u8]) {
    write!(*out, "|").ok();
    for byte in buf.bytes() {
        match byte.unwrap_or(ASCII_PERIOD) as char {
            c if is_crlf(c) => write!(*out, "{}", '.').ok(),
            c if c.is_ascii_whitespace() => write!(*out, "{}", ' ').ok(),
            c if c.is_ascii_graphic() => write!(*out, "{}", c).ok(),
            _ => write!(*out, "{}", '.').ok(),
        };
    }
    write!(*out, "|").ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_cr() {
        assert_eq!(is_crlf('\r'), true)
    }

    #[test]
    fn reports_lf() {
        assert_eq!(is_crlf('\n'), true)
    }
}