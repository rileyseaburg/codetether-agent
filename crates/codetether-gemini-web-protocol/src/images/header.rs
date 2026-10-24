use super::IMAGE_MODEL_ID;
use serde_json::json;
use uuid::Uuid;

/// Encode the image-mode `x-goog-ext-525001261-jspb` header.
///
/// # Arguments
/// * `build_label` - Current build label obtained from the session bootstrap.
///
/// # Returns
/// A JSON header value containing the observed selector and a fresh request ID.
/// No browser account identifiers or captured credentials are copied.
///
/// # Examples
/// ```
/// use codetether_gemini_web_protocol::images::{model_header, IMAGE_MODEL_ID};
/// let value: serde_json::Value = serde_json::from_str(&model_header("build")).unwrap();
/// assert_eq!(value[4], IMAGE_MODEL_ID);
/// assert_eq!(value[17][0], "build");
/// ```
pub fn model_header(build_label: &str) -> String {
    json!([
        1,
        null,
        null,
        null,
        IMAGE_MODEL_ID,
        null,
        null,
        0,
        [4, 5, 6, 8, 16, 19, 20],
        null,
        null,
        2,
        null,
        null,
        5,
        null,
        Uuid::new_v4().to_string(),
        [build_label]
    ])
    .to_string()
}
