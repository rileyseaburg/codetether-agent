//! Serializable mux output frame exposed by the Rust SSE surface.

/// One bounded output chunk attributed to its original mux session.
///
/// # Examples
/// ```
/// let output = codetether_mux_control::MuxLiveOutput {
///     session: "work".into(), data: "ready".into(), offset: 5, running: true,
/// };
/// assert_eq!(output.offset, 5);
/// ```
#[derive(Clone, Debug, serde::Serialize)]
pub struct MuxLiveOutput {
    /// Originating session name.
    pub session: String,
    /// Lossy UTF-8 rendering of terminal bytes.
    pub data: String,
    /// Next output byte offset.
    pub offset: u64,
    /// Whether the program remains running.
    pub running: bool,
}
