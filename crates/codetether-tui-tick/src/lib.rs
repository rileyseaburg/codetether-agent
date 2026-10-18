//! Ordered periodic UI updates with compact redraw change detection.
//!
//! [run] drains all subsystem updates in order before applying redraw policy.
//! [Snapshot] describes visible change counters. [animation_epoch] throttles
//! decorative redraws to the existing processing/interlude cadence.
mod animation;
mod host;
mod run;
mod snapshot;
pub use animation::{animation_epoch, animation_epoch_at};
pub use host::Host;
pub use run::run;
pub use snapshot::Snapshot;
