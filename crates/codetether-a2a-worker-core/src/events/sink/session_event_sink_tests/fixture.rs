//! Serializes test ownership of the process-global event sink.
pub(super) use super::capture_support::capture;
use super::install_sink;
use std::sync::{Mutex, MutexGuard};

static TEST_SINK: Mutex<()> = Mutex::new(());

/// Holds exclusive test ownership until after the sink has been cleared.
pub(super) struct SinkGuard {
    _lock: MutexGuard<'static, ()>,
}

/// Starts an isolated sink test, recovering if an earlier test panicked.
pub(super) fn isolated_sink() -> SinkGuard {
    let lock = TEST_SINK.lock().unwrap_or_else(|error| error.into_inner());
    install_sink(None);
    SinkGuard { _lock: lock }
}
impl Drop for SinkGuard {
    fn drop(&mut self) {
        install_sink(None); // Clear before unlocking, including on panic.
    }
}

#[test]
fn panic_clears_sink_before_releasing_test_lock() {
    let result = std::panic::catch_unwind(|| {
        let _guard = isolated_sink();
        let (_, sink) = capture();
        install_sink(Some(sink));
        panic!("exercise fixture cleanup");
    });
    assert!(result.is_err());
    // Inspect without isolated_sink(), which would clear a leaked sink.
    let _lock = TEST_SINK.lock().unwrap_or_else(|error| error.into_inner());
    assert!(super::sink().is_none());
}
