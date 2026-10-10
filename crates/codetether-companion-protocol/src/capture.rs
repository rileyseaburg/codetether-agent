use serde::{Deserialize, Deserializer, Serialize};

/// Capture causes: periodic, right-click, double-click, or fresh owner request.
/// Unknown strings fail deserialization.
///
/// ```
/// use codetether_companion_protocol::CaptureTrigger;
/// assert!(matches!(CaptureTrigger::Manual, CaptureTrigger::Manual));
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureTrigger {
    /// Local schedule, subject to the session interval.
    Periodic,
    /// Local right-click on the selected monitor.
    RightClick,
    /// Local double-click on the selected monitor.
    DoubleClick,
    /// Fresh frame satisfying an opaque owner request ID.
    Manual,
}

/// Upload model; deserialization does not validate JPEG bytes or freshness.
///
/// ```
/// use codetether_companion_protocol::Capture;
/// let frame = Capture { image: "test-only".into(),
///     captured_at: "2026-01-01T00:00:00Z".into(), trigger: None, request_id: None };
/// assert!(frame.trigger.is_none());
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Capture {
    /// Base64 JPEG; keep memory-only and exclude from diagnostics.
    pub image: String,
    /// ISO-8601 capture time; the relay normalizes accepted timestamps to UTC.
    pub captured_at: String,
    /// Omitted for legacy uploads; explicit JSON null is invalid.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub trigger: Option<CaptureTrigger>,
    /// Opaque request ID; omitted rather than null when absent.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub request_id: Option<String>,
}

fn present<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}
