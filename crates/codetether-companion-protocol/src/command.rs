use super::reply::DeviceReply;
use serde::{Deserialize, Serialize};

/// Owner-only fresh-frame question. Never sent to the capture device.
///
/// ```
/// use codetether_companion_protocol::CaptureRequest;
/// let request = CaptureRequest { question: "What changed?".into() };
/// assert!(!request.question.is_empty());
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct CaptureRequest {
    /// Owner question, subject to relay validation and the 2,000-character limit.
    pub question: String,
}

/// Acknowledgement that the relay queued an owner's fresh-frame request.
///
/// ```
/// use codetether_companion_protocol::CaptureRequestReceipt;
/// let receipt = CaptureRequestReceipt { request_id: "request-id".into() };
/// assert_eq!(receipt.request_id, "request-id");
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct CaptureRequestReceipt {
    /// Opaque UUID used to match the requested frame.
    pub request_id: String,
}

/// Device polling response. Idle is exactly `{"request_id":null,"reply":null}`, not `{}`.
///
/// ```
/// use codetether_companion_protocol::DeviceCommand;
/// assert!(DeviceCommand { request_id: None, reply: None }.request_id.is_none());
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct DeviceCommand {
    /// Opaque UUID or null; the owner's question is never included.
    #[serde(deserialize_with = "Option::<String>::deserialize")]
    pub request_id: Option<String>,
    /// Queued owner reply for the device to type, or null when idle.
    #[serde(default, deserialize_with = "Option::<DeviceReply>::deserialize")]
    pub reply: Option<DeviceReply>,
}
