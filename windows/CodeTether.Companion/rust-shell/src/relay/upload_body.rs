use codetether_companion_protocol::CaptureTrigger;
use serde::Serialize;

/// Borrowed upload body keeps the sole image encoding in zeroizing storage.
#[derive(Serialize)]
pub(super) struct UploadBody<'a> {
    pub(super) image: &'a str,
    pub(super) captured_at: &'a str,
    pub(super) trigger: CaptureTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) request_id: Option<&'a str>,
}
