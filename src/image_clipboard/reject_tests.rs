//! Tests for image data-URL rejection reasons.

use super::{RejectReason, reject_reason};
use crate::image_clipboard::data_url::MAX_BASE64_PAYLOAD_CHARS;

#[test]
fn oversized_payload_reports_too_large() {
    let text = format!(
        "data:image/png;base64,{}",
        "A".repeat(MAX_BASE64_PAYLOAD_CHARS + 1)
    );
    assert_eq!(reject_reason(&text), Some(RejectReason::TooLarge));
    assert!(RejectReason::TooLarge.message().contains("10 MB"));
}

#[test]
fn unsupported_mime_reports_unsupported() {
    let text = "data:text/plain;base64,aGVsbG8=";
    assert_eq!(reject_reason(text), Some(RejectReason::UnsupportedMime));
}

#[test]
fn truncated_payload_reports_malformed() {
    let text = "data:image/png;base64,!!!!";
    assert_eq!(reject_reason(text), Some(RejectReason::MalformedBase64));
}

#[test]
fn valid_and_non_data_url_have_no_reason() {
    use base64::Engine;
    let payload = base64::engine::general_purpose::STANDARD.encode("png bytes");
    let valid = format!("data:image/png;base64,{payload}");
    assert_eq!(reject_reason(&valid), None);
    assert_eq!(reject_reason("just prose"), None);
}
