//! Bracketed-paste marker detection with cross-read boundary memory.

use super::super::{END, START};

const TAIL_BYTES: usize = 5;

#[derive(Default)]
pub(super) struct Markers {
    bracketed: bool,
    tail: Vec<u8>,
}

impl Markers {
    pub(super) fn observe(&mut self, data: &[u8]) -> bool {
        let was_bracketed = self.bracketed;
        let mut scan = Vec::with_capacity(self.tail.len() + data.len());
        scan.extend_from_slice(&self.tail);
        scan.extend_from_slice(data);
        for marker in scan.windows(START.len()) {
            if marker == START {
                self.bracketed = true;
            } else if marker == END {
                self.bracketed = false;
            }
        }
        self.remember(&scan);
        was_bracketed || contains(&scan, START)
    }

    fn remember(&mut self, data: &[u8]) {
        let start = data.len().saturating_sub(TAIL_BYTES);
        self.tail.clear();
        self.tail.extend_from_slice(&data[start..]);
    }
}

fn contains(data: &[u8], marker: &[u8]) -> bool {
    data.windows(marker.len()).any(|value| value == marker)
}
