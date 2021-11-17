pub const ASCII_PERIOD: u8 = 46;

/// All needed formatting values for a CLI hex display
pub struct Formatting {
    /// How many bytes to read at a time?
    pub bytes_to_read: usize,
    /// How often to draw a gutter?
    pub gutter_per_n_bytes: usize,
    /// width of ASCII section
    pub ascii_line_width: usize,
    /// width of Hexadecimal section
    pub hex_line_width: usize,
}

/// get a specific number of spaces as string
/// 
/// # Arguments
/// * n -  number of spaces to print
pub fn pad_spaces(n: usize) -> String {
    let mut s = String::new();
    for _ in 0..n {
        s.push(' ');
    }
    s
}

/// Is this index a gutter?
pub fn is_gutter(index: usize, interval: usize, max: usize) -> bool {
    (index + 1) % interval == 0 && (index + 1) != max
}

/// Is this character an ASCII \r or \n?
pub fn is_crlf(c: char) -> bool {
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
