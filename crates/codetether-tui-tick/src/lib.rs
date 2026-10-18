//! Ordered periodic UI updates with compact redraw change detection.
//!
//! [run] drains all subsystem updates in order before applying redraw policy.
//! [Snapshot] describes visible change counters. [animation_epoch] throttles
//! decorative redraws to the existing processing/interlude cadence.
mod animation;
mod binding;
mod host;
mod run;
mod snapshot;
pub use animation::{animation_epoch, animation_epoch_at};
pub use host::Host;
pub use run::run;
pub use snapshot::Snapshot;

/// Approval overlay reconciliation after external decisions.
pub mod approval;
/// Cursor-gated worker roster updates.
pub mod worker_sync;

/// Keyboard-responsive streaming redraw rate limiting.
pub mod redraw;

/// Background source ordering and completion-notice delivery.
pub mod background;
