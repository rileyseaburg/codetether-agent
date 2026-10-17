//! Bounded event retention and receipt counting.
use super::*;
impl BusBridge {
    /// Push an envelope into the ring buffer, evicting oldest if full.
    pub(super) async fn push_envelope(&self, envelope: BusEnvelope) {
        let mut buf = self.buffer.write().await;
        if buf.len() >= self.capacity {
            buf.pop_front();
        }
        buf.push_back(envelope);
        drop(buf);
        self.total_received.fetch_add(1, Ordering::Relaxed);
    }
}
