use pseudo_hexdump::{hex_reader::HexLineReader, text_utilities::Formatting};

use std::error;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();

    let format_cfg = Formatting {
        bytes_to_read: opt.num_bytes,
        gutter_per_n_bytes: opt.gutter_interval,
        ascii_line_width: (opt.num_bytes * 3)  // 2 chars per byte + 1 space
            + (opt.num_bytes / opt.gutter_interval), // 1 char gutter per every n bytes
        hex_line_width: opt.num_bytes,
    };

    let file = File::open(&opt.input)?;
    let reader = BufReader::new(file);

    // Get type of output
    let mut out: Box<dyn Write> = match opt.output {
        Some(p) => {
            let file = File::create(p)?;
            Box::new(BufWriter::new(file))
        }
        None => {
            /*
              I don't know why this works (yet), but:
              https://stackoverflow.com/a/41549727
              (read the Rust docs for Box::leak)
            */
            let stdout = Box::leak(Box::new(io::stdout()));
            Box::new(stdout.lock())
        },
    };

    // open HexLineReader and iterate over lines
    let hr = HexLineReader::new(Box::new(reader), format_cfg);
    for line in hr {
        write!(out, "{}", line)?;
    }
    Ok(())
}
