use super::*;
impl BusRecorder {
    /// Return up to `limit` most-recent envelopes (newest last), optionally
    /// filtered by a topic prefix.
    pub fn recent(&self, limit: usize, topic_prefix: Option<&str>) -> Vec<BusEnvelope> {
        let buf = match self.buf.lock() {
            Ok(b) => b,
            Err(p) => p.into_inner(),
        };
        let mut matched: Vec<BusEnvelope> = buf
            .iter()
            .filter(|e| topic_prefix.is_none_or(|p| e.topic.starts_with(p)))
            .cloned()
            .collect();
        if matched.len() > limit {
            matched.drain(0..matched.len() - limit);
        }
        matched
    }
}
