//! Worker SSE connection consumption and durable stream progress.
//!
//! [connect_stream] loads a worker cursor, requests the stream, and processes
//! notifications, periodic reconciliation and byte chunks. [Host] supplies
//! application operations; frame dispatch and bounded ingestion stay shared.
mod host;
mod ingest;
mod run;
pub use host::Host;
pub use run::connect_stream;
/// Resume-cursor file path for a given worker id.
fn cursor_path(worker_id: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("codetether-stream-cursor-{worker_id}"))
}
