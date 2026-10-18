//! Shared PTY dimensions and output chunk types.

/// Terminal dimensions supplied by an attached client.
///
/// # Examples
/// ```
/// let size = codetether_mux_pty::TerminalSize::new(80, 24);
/// assert_eq!(size.columns, 80);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TerminalSize {
    /// Width in character cells.
    pub columns: u16,
    /// Height in character cells.
    pub rows: u16,
}

impl TerminalSize {
    /// Construct terminal dimensions from columns and rows.
    pub fn new(columns: u16, rows: u16) -> Self {
        Self { columns, rows }
    }
}

/// Reconnect position and display mode captured before a PTY resize.
///
/// # Examples
/// ```
/// let state = codetether_mux_pty::PtyAttach { offset: 0, replay_until: 0, alternate_screen: false };
/// assert_eq!(state.offset, state.replay_until);
/// ```
pub struct PtyAttach {
    /// First retained byte to replay.
    pub offset: u64,
    /// End of output captured before attachment.
    pub replay_until: u64,
    /// Whether the captured terminal uses its alternate screen.
    pub alternate_screen: bool,
}

/// One bounded slice of persistent terminal output.
///
/// # Examples
/// ```
/// let chunk = codetether_mux_pty::PtyChunk { data: vec![], next_offset: 0, running: false };
/// assert!(!chunk.running);
/// ```
pub struct PtyChunk {
    /// Captured terminal bytes.
    pub data: Vec<u8>,
    /// Offset for the next read.
    pub next_offset: u64,
    /// Whether the output reader still considers the program live.
    pub running: bool,
}
