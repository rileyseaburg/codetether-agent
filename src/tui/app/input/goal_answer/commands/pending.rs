//! One-shot continuation intentions, keyed by session rather than chat text.

use parking_lot::Mutex;
use std::{collections::HashSet, sync::LazyLock};

static PENDING: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(Mutex::default);

pub(super) fn mark(id: &str) {
    PENDING.lock().insert(id.to_string());
}

pub(super) fn take(id: &str) -> bool {
    PENDING.lock().remove(id)
}
