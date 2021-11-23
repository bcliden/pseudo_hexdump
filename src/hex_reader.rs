use std::io::Read;
use std::iter::{IntoIterator, Iterator};
use anyhow::{Context, Result};

use crate::line_counter::LineCounter;
use crate::text_utilities::{is_crlf, is_gutter, pad_spaces, Formatting, ASCII_PERIOD};

/// Reads a file into lines of hex representation.
/// It's sort of a one-shot operation to get line-by-line formatted text
pub struct HexLineReader {
    /// source to read from
    in_file: Box<dyn Read>,
    /// internal counter for # lines read
    lc: LineCounter,

    /// buffer to be filled with source bytes
    buf: Vec<u8>,
    /// number of bytes read in most recent read
    bytes_read: usize,

    /// Formatting object for string-based output
    cfg: Formatting,
    /// Has this struct reached the end of file?
    exhausted: bool,
}

impl HexLineReader {

    /// Get new HexLineReader instance
    /// # Arguments
    /// * in_file - Reader from which to pull bytes
    /// * fmt - Formatting object containing the text layout to use
    pub fn new(in_file: Box<dyn Read>, fmt: Formatting) -> Self {
        HexLineReader {
            in_file,
            lc: LineCounter::new(fmt.bytes_to_read),
            buf: vec![0; fmt.bytes_to_read],
            cfg: fmt,
            exhausted: false,
            bytes_read: 0,
        }
    }

    /// Fill own buffer with bytes from in_file
    fn fill_buf_next(&mut self) -> Result<()> {
        match self.in_file.read(self.buf.as_mut_slice()) {
            Ok(n_bytes) => {
                self.bytes_read = n_bytes;
                Ok(())
            }
            Err(e) => {
                self.bytes_read = 0;
                Err(e).context("failed to get bytes from HexReader file")
            }
        }
    }

    /// Reference to buffer, limited by the amount of bytes read
    fn get_buf_ref(&self) -> &[u8] {
        &self.buf[..self.bytes_read]
    }

    /// Read current buffer as hex representation
    /// 
    /// This will include some formatting (gutters primarily)
    fn buf_as_hex_string(&self) -> Result<String> {
        let buf = self.get_buf_ref();
        let mut s = Vec::<char>::with_capacity(self.cfg.ascii_line_width);

        for (index, byte) in buf.bytes().enumerate() {
            let byte = byte.unwrap_or(ASCII_PERIOD);
            s.extend(format!("{:02x} ", byte).chars());

            if is_gutter(index, self.cfg.gutter_per_n_bytes, self.cfg.bytes_to_read) {
                s.push(' ');
            };
        }

        Ok(s.into_iter().collect())
    }

    /// Read current buffer as ASCII representation
    /// 
    /// This will include some substitutions and ignore whitespace
    ///     (\r || \n => '.')
    ///     (whitespace => ' ')
    ///     (unknown => '.')
    fn buf_as_ascii_string(&self) -> Result<String> {
        let buf = self.get_buf_ref();
        let mut s = Vec::<char>::with_capacity(self.cfg.hex_line_width);

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

    /// Get next line representation. 
    /// 
    /// Will print both address, hex, and ascii representation.
    /// 
    /// # Example
    /// `[address] hex add res ses |ascii.representation|`
    pub fn get_next_line(&mut self) -> Option<String> {
        if self.exhausted {
            return None;
        }

        // fill self.buf and set self.bytes_read
        self.fill_buf_next().ok()?;
        let mut out = String::new();

        // write ADDR
        let str = format!("[{:#08x}]", self.lc);
        out.push_str(str.as_ref());

        // if no bytes, write \n and set self.exhausted
        if self.bytes_read == 0 {
            // writeln!(&mut out)?;
            out.push('\n');
            self.exhausted = true;
            return Some(out);
        }

        // write HEX LINE
        let line = self.buf_as_hex_string().ok()?;
        let remainder = self.cfg.ascii_line_width.saturating_sub(line.len());
        out.push_str(format!("  {}{}", line, pad_spaces(remainder)).as_ref());

        // write ASCII LINE
        let line = self.buf_as_ascii_string().ok()?;
        out.push_str(format!("|{}|", line).as_ref());
        out.push('\n');

        self.lc.increment();

        Some(out)
    }
}

impl IntoIterator for HexLineReader {
    type Item = String;
    type IntoIter = HexReaderIterator;

    fn into_iter(self) -> Self::IntoIter {
        HexReaderIterator { hr: self }
    }
}

pub struct HexReaderIterator {
    hr: HexLineReader,
}

impl Iterator for HexReaderIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.hr.get_next_line()
    }
}
