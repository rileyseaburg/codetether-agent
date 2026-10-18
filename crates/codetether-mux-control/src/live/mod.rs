//! Fan-in for event-driven output from every registered mux server.
//!
//! [subscribe_live_output] follows discovered sessions and forwards
//! [MuxLiveOutput] frames through a bounded channel.

mod connection;
mod session;
mod types;

pub use types::MuxLiveOutput;

/// Follow current registered sessions through a bounded output channel.
///
/// # Returns
/// A receiver for at most 256 queued frames, with one reconnecting task per session.
/// Discovery errors close the channel without producing frames.
/// # Panics
/// Panics if called outside a Tokio runtime.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let mut output = codetether_mux_control::subscribe_live_output();
/// if let Some(frame) = output.recv().await { assert!(!frame.session.is_empty()); }
/// # }
/// ```
pub fn subscribe_live_output() -> tokio::sync::mpsc::Receiver<MuxLiveOutput> {
    let (sender, receiver) = tokio::sync::mpsc::channel(256);
    tokio::spawn(async move {
        let Ok(records) = codetether_mux_registry::list().await else {
            return;
        };
        for record in records {
            for session in record.session_names() {
                tokio::spawn(session::follow(session.to_string(), sender.clone()));
            }
        }
    });
    receiver
}

mod output;
