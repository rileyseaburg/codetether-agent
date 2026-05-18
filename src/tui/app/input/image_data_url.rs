use base64::Engine;

pub(crate) fn encode(mime_type: &str, bytes: &[u8]) -> String {
    let payload = base64::engine::general_purpose::STANDARD.encode(bytes);
    format!("data:{mime_type};base64,{payload}")
}

pub(crate) fn decoded_len(data_url: &str) -> Option<usize> {
    let (_, payload) = data_url.split_once(";base64,")?;
    base64::engine::general_purpose::STANDARD
        .decode(payload.trim())
        .ok()
        .map(|bytes| bytes.len())
}
