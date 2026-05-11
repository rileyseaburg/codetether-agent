//! Origin-aware pairing repair for incremental derivation (issue #231
//! item 5).
//!
//! Mirrors
//! [`crate::session::helper::experimental::pairing::repair_orphans`]
//! but updates a parallel [`MessageOrigin`] vector so the caller can
//! recompute `dropped_ranges` accurately after repair + clamp.

use crate::provider::Message;

use super::incremental_repair_drop::drop_orphan_results;
use super::incremental_repair_inject::inject_synthetic;
use super::incremental_types::MessageOrigin;

/// Run pairing repair in place, keeping `origins` aligned with
/// `messages`.
///
/// # Panics
///
/// Panics in debug builds if the input lengths are out of sync.
pub fn repair_with_origins(messages: &mut Vec<Message>, origins: &mut Vec<MessageOrigin>) {
    debug_assert_eq!(messages.len(), origins.len());
    inject_synthetic(messages, origins);
    drop_orphan_results(messages, origins);
    debug_assert_eq!(messages.len(), origins.len());
}
