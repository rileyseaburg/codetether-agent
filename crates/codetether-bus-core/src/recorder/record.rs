use super::*;
impl BusRecorder {
    /// Append an envelope, evicting the oldest when at capacity.
    pub fn record(&self, envelope: &BusEnvelope) {
        let mut buf = match self.buf.lock() {
            Ok(b) => b,
            Err(p) => p.into_inner(),
        };
        if buf.len() == self.cap {
            buf.pop_front();
        }
        buf.push_back(envelope.clone());
        // Increment under the buf lock so `pushed` stays consistent with len.
        self.pushed.fetch_add(1, Ordering::Relaxed);
    }
}
