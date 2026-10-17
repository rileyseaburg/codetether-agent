use super::*;
impl BusRecorder {
    /// Create a recorder retaining at most `cap` envelopes.
    pub fn new(cap: usize) -> Self {
        Self {
            cap: cap.max(1),
            buf: Mutex::new(VecDeque::with_capacity(cap.max(1))),
            pushed: AtomicU64::new(0),
        }
    }
}
