use std::fmt::LowerHex;
use std::fmt::{self, Display};

/// Simple counter, taking a configurable step size
#[derive(Clone, Copy, Debug)]
pub struct LineCounter {
    count: usize,
    step: usize,
}
impl LineCounter {

    /// New LineCounter
    /// 
    /// # Arguments
    /// * step_size - how much to `increment` at a time
    pub fn new(step_size: usize) -> Self {
        LineCounter {
            count: 0,
            step: step_size,
        }
    }

    /// Increment counter using the given step size
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
