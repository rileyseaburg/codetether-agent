//! Incremental tracking of alternate-screen ANSI mode transitions.

use memchr::memchr_iter;

const HISTORY_LEN: usize = 7;
const ENTER: [&[u8]; 3] = [b"\x1b[?47h", b"\x1b[?1047h", b"\x1b[?1049h"];
const LEAVE: [&[u8]; 3] = [b"\x1b[?47l", b"\x1b[?1047l", b"\x1b[?1049l"];

/// Stateful detector for alternate-screen enter and leave sequences.
pub(in crate::mux) struct TerminalMode {
    active: bool,
    history: Vec<u8>,
}

impl TerminalMode {
    pub(in crate::mux) fn new(active: bool) -> Self {
        Self {
            active,
            history: Vec::new(),
        }
    }

    pub(in crate::mux) fn observe(&mut self, data: &[u8]) {
        let mut bytes = Vec::with_capacity(self.history.len() + data.len());
        bytes.extend_from_slice(&self.history);
        bytes.extend_from_slice(data);
        for index in memchr_iter(b'\x1b', &bytes) {
            if let Some(active) = mode_at(&bytes[index..]) {
                self.active = active;
            }
        }
        let keep_from = bytes.len().saturating_sub(HISTORY_LEN);
        self.history.clear();
        self.history.extend_from_slice(&bytes[keep_from..]);
    }

    pub(in crate::mux) fn active(&self) -> bool {
        self.active
    }
}

fn mode_at(bytes: &[u8]) -> Option<bool> {
    if ENTER.iter().any(|sequence| bytes.starts_with(sequence)) {
        Some(true)
    } else if LEAVE.iter().any(|sequence| bytes.starts_with(sequence)) {
        Some(false)
    } else {
        None
    }
}
