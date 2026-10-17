//! encode for image loading.
use super::*;
impl ImageTool {
    /// Encode image data as base64 data URL
    pub(super) fn encode_as_data_url(data: &[u8], mime_type: &str) -> String {
        let base64 = STANDARD.encode(data);
        format!("data:{};base64,{}", mime_type, base64)
    }
}
