//! Whether an interactive session is accepting inbound peer turns.
//!
//! Per process: the TUI calls [`attach`] by default once its A2A peer
//! endpoint is up (skipped entirely with `--no-a2a`), and [`detach`] on
//! `/a2a headless`. While detached, the server keeps its headless behaviour, so a
//! LAN peer can never inject a prompt into a session that did not ask.

use std::sync::atomic::{AtomicBool, Ordering};

static ATTACHED: AtomicBool = AtomicBool::new(false);

/// Start routing inbound peer turns to the live session.
/// Enable routing of new peer turns to the interactive session.
/// # Arguments
/// Takes no arguments.
/// # Returns
/// Unit after updating the attachment flag.
/// # Examples
/// ```rust
/// codetether_a2a_inbox::attach();
/// assert!(codetether_a2a_inbox::is_attached());
/// ```
pub fn attach() {
    ATTACHED.store(true, Ordering::SeqCst);
    tracing::info!(target: "codetether_agent::a2a::live_inbox::registry", "A2A inbound turns now route to the interactive session");
}

/// Stop routing; pending turns already handed over still complete.
/// Disable routing of new peer turns to the interactive session.
/// # Arguments
/// Takes no arguments.
/// # Returns
/// Unit after updating the attachment flag.
/// # Examples
/// ```rust
/// codetether_a2a_inbox::detach();
/// assert!(!codetether_a2a_inbox::is_attached());
/// ```
pub fn detach() {
    ATTACHED.store(false, Ordering::SeqCst);
}

/// Read whether the interactive session accepts new peer turns.
/// # Arguments
/// Takes no arguments.
/// # Returns
/// The current attachment flag.
/// # Examples
/// ```rust
/// codetether_a2a_inbox::attach();
/// assert!(codetether_a2a_inbox::is_attached());
/// ```
pub fn is_attached() -> bool {
    ATTACHED.load(Ordering::SeqCst)
}
