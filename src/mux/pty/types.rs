//! Shared PTY dimensions and output chunk types.

/// Terminal dimensions supplied by an attached client.
#[derive(Clone, Copy, Debug)]
pub(in crate::mux) struct TerminalSize {
    pub columns: u16,
    pub rows: u16,
}

impl TerminalSize {
    pub(in crate::mux) fn new(columns: u16, rows: u16) -> Self {
        Self { columns, rows }
    }
}

/// Reconnect position and display mode captured before a PTY resize.
pub(in crate::mux) struct PtyAttach {
    pub offset: u64,
    pub replay_until: u64,
    pub alternate_screen: bool,
}

/// One bounded slice of persistent terminal output.
pub(in crate::mux) struct PtyChunk {
    pub data: Vec<u8>,
    pub next_offset: u64,
    pub running: bool,
}
