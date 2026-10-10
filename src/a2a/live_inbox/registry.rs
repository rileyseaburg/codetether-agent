//! Whether an interactive session is accepting inbound peer turns.
//!
//! Per process: the TUI calls [`attach`] by default once its A2A peer
//! endpoint is up (skipped entirely with `--no-a2a`), and [`detach`] on
//! `/a2a headless`. While detached, the server keeps its headless behaviour, so a
//! LAN peer can never inject a prompt into a session that did not ask.

use std::sync::atomic::{AtomicBool, Ordering};

static ATTACHED: AtomicBool = AtomicBool::new(false);

/// Start routing inbound peer turns to the live session.
pub fn attach() {
    ATTACHED.store(true, Ordering::SeqCst);
    tracing::info!("A2A inbound turns now route to the interactive session");
}

/// Stop routing; pending turns already handed over still complete.
pub fn detach() {
    ATTACHED.store(false, Ordering::SeqCst);
}

pub fn is_attached() -> bool {
    ATTACHED.load(Ordering::SeqCst)
}
