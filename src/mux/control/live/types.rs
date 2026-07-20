//! Serializable mux output frame exposed by the Rust SSE surface.

/// One bounded output chunk attributed to its original mux session.
#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct MuxLiveOutput {
    pub session: String,
    pub data: String,
    pub offset: u64,
    pub running: bool,
}
