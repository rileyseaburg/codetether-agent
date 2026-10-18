//! Bounded PTY replay storage and incremental terminal-mode tracking.
//!
//! [OutputBuffer] retains a fixed-size window of output with absolute offsets.
//! [TerminalMode] recognizes alternate-screen transitions split across reads.
//! Neither component starts processes or writes terminal output.
mod buffer;
mod terminal_mode;
pub use buffer::OutputBuffer;
pub use terminal_mode::TerminalMode;
