//! Pinned-constraint selection and incremental budget reservations.
//!
//! Pin indices are bounded by parallel page/message arrays. Recent entries
//! and pins reserve budget once when their keep flag changes.
use crate::pages::PageKind;
use codetether_provider_core::Message;
mod pinned_indices;
pub use pinned_indices::pinned_indices;
mod force_keep_pins;
pub use force_keep_pins::force_keep_pins;
mod force_keep_base;
pub use force_keep_base::force_keep_base;
