mod line_counter;

use line_counter::LineCounter;
use std::char;
use std::error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

const ASCII_PERIOD: u8 = 46;

/**
 * TODO:
 * - [ ] gutter size of zero
 * - [ ] fix spacing after ascii row when gutter is at end (cargo run ./src/main.rs -g 4 -n 8)
 */

#[derive(StructOpt, Debug)]
#[structopt(name = "pseudo_hexdump", author = "Benjamin Liden")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present. Really should just use bash for this
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    // number of bytes wide to print
    #[structopt(short, long, default_value = "16")]
    num_bytes: usize,

    // after how many bytes to print columns
    #[structopt(short, long, default_value = "4")]
    gutter_interval: usize,
}

struct Formatting {
    bytes_to_read: usize,
    gutter_per_n_bytes: usize,
    ascii_line_width: usize,
    hex_line_width: usize,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();

    let cfg = Formatting {
        bytes_to_read: opt.num_bytes,
        gutter_per_n_bytes: opt.gutter_interval,
        ascii_line_width: (opt.num_bytes * 3)  // 2 chars per byte + 1 space
            + (opt.num_bytes / opt.gutter_interval), // 1 char gutter per every n bytes
        hex_line_width: opt.num_bytes,
    };

    let mut file = File::open(&opt.input)?;

    let mut out: Box<dyn Write> = match opt.output {
        Some(p) => Box::new(File::create(p)?),
        None => Box::new(io::stdout()),
    };

    let mut buf: Vec<u8> = vec![0; cfg.bytes_to_read];
    let mut lc = LineCounter::new(cfg.bytes_to_read);

    while let Ok(n_bytes) = file.read(&mut buf) {
        // write ADDR
        write!(out, "[{:#08x}]", lc)?;

        // if no bytes, write \n and quit
        if n_bytes == 0 {
            writeln!(&mut out)?;
            return Ok(());
        }

        let nbuf = &buf[..n_bytes];

        // write HEX LINE
        let line = get_line_hex(nbuf, &cfg)?;
        let remainder = cfg.ascii_line_width.saturating_sub(line.len());
        write!(out, "  {}{}", line, pad_spaces(remainder))?;

        // write ASCII LINE
        let line = get_line_ascii(nbuf, &cfg)?;
        write!(out, "|{}|", line)?;

        writeln!(out)?;
        lc.increment();
    }
    Ok(())
}

fn get_line_hex(buf: &[u8], cfg: &Formatting) -> Result<String, Box<dyn error::Error>> {
    let mut s = Vec::<char>::with_capacity(cfg.ascii_line_width);

    for (index, byte) in buf.bytes().enumerate() {
        let byte = byte.unwrap_or(ASCII_PERIOD);
        s.extend(format!("{:02x} ", byte).chars());

        if is_gutter(index, cfg.gutter_per_n_bytes, cfg.bytes_to_read) {
            s.push(' ');
        };
    }

    Ok(s.into_iter().collect())
}

fn get_line_ascii(buf: &[u8], cfg: &Formatting) -> Result<String, Box<dyn error::Error>> {
    let mut s = Vec::<char>::with_capacity(cfg.hex_line_width);

    for byte in buf.bytes() {
        match byte.unwrap_or(ASCII_PERIOD) as char {
            c if is_crlf(c) => s.push('.'),
            c if c.is_ascii_whitespace() => s.push(' '),
            c if c.is_ascii_graphic() => s.push(c),
            _ => s.push('.'),
        };
    }

    Ok(s.into_iter().collect())
}

fn pad_spaces(n: usize) -> String {
    let mut s = String::new();
    for _ in 0..n {
        s.push(' ');
    }
    s
}

fn is_gutter(index: usize, interval: usize, max: usize) -> bool {
    (index + 1) % interval == 0 && (index + 1) != max
}

fn is_crlf(c: char) -> bool {
    c == 0x0a as char || c == 0x0d as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_cr() {
        assert_eq!(is_crlf('\r'), true);
        assert_eq!(is_crlf(0x0a as char), true);
    }

    #[test]
    fn reports_lf() {
        assert_eq!(is_crlf('\n'), true);
        assert_eq!(is_crlf(0x0d as char), true);
    }

    #[test]
    fn pad_spaces_works() {
        assert_eq!(pad_spaces(0), String::from(""));
        assert_eq!(pad_spaces(5), String::from("     "));
        assert_eq!(pad_spaces(10), String::from("          "));
    }

    #[test]
    fn is_gutter_works() {
        let gutter_interval_in_bytes = 4;
        let number_of_intervals = 4;
        let max = gutter_interval_in_bytes * number_of_intervals;
        let break_indices = vec![3, 7, 11];

        for n in 0..max {
            match n {
                n if break_indices.contains(&n) => {
                    assert_eq!(is_gutter(n, gutter_interval_in_bytes, max), true);
                }
                _ => {
                    assert_eq!(is_gutter(n, gutter_interval_in_bytes, max), false);
                }
            };
        }
    }
}
