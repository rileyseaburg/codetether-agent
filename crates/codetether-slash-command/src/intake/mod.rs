//! Chat-input command intake before ordinary slash-command dispatch.
//!
//! [run] preserves shell, approval and mux priority before selecting a live
//! session or session-free UI commands. [with_session] sequences live-session
//! parity handling and ordinary command dispatch under one native borrow.
mod host;
mod run;
mod session;
pub use host::Host;
pub use run::run;
pub use session::{SessionHost, with_session};
