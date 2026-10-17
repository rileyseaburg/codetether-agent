//! Stop-string filtering across token boundaries, without leaking a partial stop marker.
#[cfg(test)]
#[path = "stop_tests.rs"]
mod tests;
use anyhow::Result;
pub(super) struct Filter {
    pending: String,
    stops: Vec<String>,
    stopped: bool,
}
impl Filter {
    pub fn new(stops: Vec<String>) -> Self {
        Self {
            pending: String::new(),
            stops,
            stopped: false,
        }
    }
    pub fn push(&mut self, text: &str, emit: &mut dyn FnMut(&str) -> Result<()>) -> Result<bool> {
        self.pending.push_str(text);
        if let Some(index) = self.stops.iter().filter_map(|s| self.pending.find(s)).min() {
            emit(&self.pending[..index])?;
            self.pending.clear();
            self.stopped = true;
            return Ok(false);
        }
        let mut keep = 0;
        for stop in &self.stops {
            for length in 1..=stop.len().min(self.pending.len()) {
                if stop.is_char_boundary(length) && self.pending.ends_with(&stop[..length]) {
                    keep = keep.max(length);
                }
            }
        }
        let emit_len = self.pending.len() - keep;
        emit(&self.pending[..emit_len])?;
        self.pending.drain(..emit_len);
        Ok(true)
    }
    pub fn finish(&mut self, emit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        if !self.stopped {
            emit(&self.pending)?;
            self.pending.clear();
        }
        Ok(())
    }
}
