//! Parsing and formatting of the SSE `id:` field for resumable streams.
//!
//! An [`EventId`] is `<epoch>.<seq>` where `epoch` is an opaque server token
//! per stream lifetime and `seq` is a strictly monotonic `u64`. See
//! `docs/transport-phase1-wire-contract.md` section 3.1.

/// A parsed SSE event id: an epoch token plus a monotonic sequence number.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::event_id::EventId;
///
/// let id = EventId::parse("abc123.42").unwrap();
/// assert_eq!(id.epoch, "abc123");
/// assert_eq!(id.seq, 42);
/// assert_eq!(id.format(), "abc123.42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventId {
    /// Opaque server-minted token identifying the stream lifetime.
    pub epoch: String,
    /// Strictly monotonic sequence number within the epoch.
    pub seq: u64,
}

impl EventId {
    /// Parse an `id:` value of the form `<epoch>.<seq>`.
    ///
    /// # Returns
    ///
    /// `Some(EventId)` when well-formed, `None` when the epoch is empty, the
    /// separator is missing, or the sequence is not a `u64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::a2a::stream::event_id::EventId;
    ///
    /// assert!(EventId::parse("e.1").is_some());
    /// assert!(EventId::parse("noseq").is_none());
    /// assert!(EventId::parse(".5").is_none());
    /// assert!(EventId::parse("e.x").is_none());
    /// ```
    pub fn parse(raw: &str) -> Option<Self> {
        let (epoch, seq) = raw.rsplit_once('.')?;
        if epoch.is_empty() {
            return None;
        }
        Some(Self {
            epoch: epoch.to_string(),
            seq: seq.parse().ok()?,
        })
    }

    /// Format back into the wire form `<epoch>.<seq>`.
    pub fn format(&self) -> String {
        format!("{}.{}", self.epoch, self.seq)
    }
}

#[cfg(test)]
mod tests {
    use super::EventId;

    #[test]
    fn roundtrip() {
        let id = EventId::parse("abc.99").unwrap();
        assert_eq!(id.seq, 99);
        assert_eq!(id.format(), "abc.99");
    }

    #[test]
    fn epoch_with_dots_keeps_last_as_separator() {
        let id = EventId::parse("a.b.c.7").unwrap();
        assert_eq!(id.epoch, "a.b.c");
        assert_eq!(id.seq, 7);
    }

    #[test]
    fn rejects_malformed() {
        assert!(EventId::parse("noseq").is_none());
        assert!(EventId::parse(".5").is_none());
        assert!(EventId::parse("e.x").is_none());
        assert!(EventId::parse("e.-1").is_none());
    }
}
