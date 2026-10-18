//! Client-to-server mux operations.

/// One authenticated mux control request.
///
/// A connection binds to one session at [`ClientRequest::Authenticate`];
/// window, program, agent, and runtime requests then act only on that
/// session. [`ClientRequest::Coordinate`] is server-wide because leases guard
/// the checkout every session shares.
/// # Examples
/// ```
/// let request = codetether_mux_protocol::ClientRequest::Snapshot;
/// assert!(matches!(request, codetether_mux_protocol::ClientRequest::Snapshot));
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientRequest {
    Authenticate {
        /// Authentication credential.
        token: String,
        /// Session this connection operates on; `None` selects server-only
        /// operations (`CreateSession`, `Coordinate`, `Shutdown`, `Snapshot`).
        #[serde(default)]
        /// Optional session selected for this connection.
        session: Option<String>,
    },
    Snapshot,
    CreateSession {
        /// Session name.
        name: String,
        /// Requested workspace directory.
        workspace: std::path::PathBuf,
    },
    CloseSession {
        /// Session name.
        name: String,
    },
    CreateWindow {
        /// Requested workspace directory.
        workspace: std::path::PathBuf,
    },
    SelectWindow {
        /// Target window ID.
        id: u64,
    },
    CloseWindow {
        /// Target window ID.
        id: u64,
    },
    ChangeDirectory {
        /// Requested workspace directory.
        workspace: std::path::PathBuf,
    },
    Program {
        /// Nested operation.
        request: super::ProgramRequest,
    },
    Agent {
        /// Nested operation.
        request: super::AgentRequest,
    },
    ReportRuntime {
        /// Optional semantic runtime state.
        status: Option<codetether_mux_model::MuxRuntimeStatus>,
    },
    Coordinate {
        /// Nested operation.
        request: codetether_worktree_lease::CoordinationRequest,
    },
    Detach,
    Shutdown,
}
