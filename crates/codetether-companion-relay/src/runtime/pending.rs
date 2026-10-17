//! In-flight owner request and queued reply records.
/// Owner fresh-frame request; the question never reaches the device.
pub(crate) struct Pending {
    pub id: String,
    pub question: String,
    pub created: i64,
}
/// Owner reply queued for the device to type; re-served until acknowledged.
pub(crate) struct QueuedReply {
    pub id: String,
    pub text: String,
    pub created: i64,
}
