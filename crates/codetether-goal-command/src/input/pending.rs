//! One-shot continuation intentions, keyed by session rather than chat text.

use parking_lot::Mutex;
use std::{collections::HashSet, sync::LazyLock};

static PENDING: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);

pub(super) fn mark(id: &str) {
    PENDING.lock().insert(id.to_string());
}

/// Consume a pending continuation intent exactly once for this session.
/// # Arguments
/// `id` identifies the session.
/// # Returns
/// Whether a pending intent existed.
/// # Examples
/// ```
/// assert!(!codetether_goal_command::input::take_pending("unmarked-example"));
/// ```
pub fn take(id: &str) -> bool {
    PENDING.lock().remove(id)
}
