use std::fmt::LowerHex;
use std::fmt::{self, Display};

pub struct LineCounter {
    count: usize,
    step: usize,
}
impl LineCounter {
    pub fn new(step_size: usize) -> Self {
        LineCounter {
            count: 0,
            step: step_size,
        }
    }
    pub fn increment(&mut self) {
        self.count += self.step;
    }
}
impl Default for LineCounter {
    fn default() -> Self {
        LineCounter::new(16)
    }
}
impl Display for LineCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.count, f) // delegate to usize's implementation
    }
}
impl LowerHex for LineCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.count, f) // delegate to usize's implementation
    }
}
