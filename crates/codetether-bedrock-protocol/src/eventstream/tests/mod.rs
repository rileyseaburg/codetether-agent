//! Existing frame-decoding regressions.
use super::*;

pub(super) fn build_frame(headers: &[(&str, &str)], payload: &[u8]) -> Vec<u8> {
    let mut header_bytes = Vec::new();
    for (k, v) in headers {
        header_bytes.push(k.len() as u8);
        header_bytes.extend_from_slice(k.as_bytes());
        header_bytes.push(7u8); // string type
        header_bytes.extend_from_slice(&(v.len() as u16).to_be_bytes());
        header_bytes.extend_from_slice(v.as_bytes());
    }
    let total_len = 16 + header_bytes.len() + payload.len();
    let mut frame = Vec::new();
    frame.extend_from_slice(&(total_len as u32).to_be_bytes());
    frame.extend_from_slice(&(header_bytes.len() as u32).to_be_bytes());
    frame.extend_from_slice(&0u32.to_be_bytes());
    frame.extend_from_slice(&header_bytes);
    frame.extend_from_slice(payload);
    frame.extend_from_slice(&0u32.to_be_bytes());
    frame
}
mod handles_chunked_delivery;
mod parses_multiple_frames;
mod parses_single_frame_with_headers;
