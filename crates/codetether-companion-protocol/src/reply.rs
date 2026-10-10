use serde::{Deserialize, Serialize};

/// Owner reply queued for the device to type; the device re-receives it until acknowledged.
///
/// ```
/// use codetether_companion_protocol::DeviceReply;
/// let reply = DeviceReply { id: "reply-id".into(), text: "Hello".into() };
/// assert_eq!(reply.text, "Hello");
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct DeviceReply {
    /// Opaque UUID matching the owner's queued reply.
    pub id: String,
    /// Plain text the device types; limited to 2,000 characters by the relay.
    pub text: String,
}

/// Device acknowledgement sent after a reply was typed (or refused).
///
/// ```
/// use codetether_companion_protocol::TypedAck;
/// let ack = TypedAck { reply_id: "reply-id".into() };
/// assert_eq!(ack.reply_id, "reply-id");
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct TypedAck {
    /// Opaque UUID of the reply that was handled.
    pub reply_id: String,
}

/// Receipt for an owner's queued reply.
///
/// ```
/// use codetether_companion_protocol::ReplyReceipt;
/// let receipt = ReplyReceipt { reply_id: "reply-id".into() };
/// assert_eq!(receipt.reply_id, "reply-id");
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ReplyReceipt {
    /// Opaque UUID used to match the typed acknowledgement.
    pub reply_id: String,
}

/// Owner request body for queueing a reply the device will type.
///
/// ```
/// use codetether_companion_protocol::ReplyRequest;
/// let request = ReplyRequest { text: "Hello".into() };
/// assert_eq!(request.text.len(), 5);
/// ```
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ReplyRequest {
    /// Plain text to type, subject to relay validation and the 2,000-character limit.
    pub text: String,
}
