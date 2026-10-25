use anyhow::{Result, anyhow, ensure};
use reqwest::{
    RequestBuilder, Response,
    header::{COOKIE, HeaderValue},
};

pub(super) fn cookie(request: RequestBuilder, value: String) -> Result<RequestBuilder> {
    if value.is_empty() {
        return Ok(request);
    }
    let mut header = HeaderValue::from_str(&value).map_err(|_| anyhow!("cookie_header_invalid"))?;
    header.set_sensitive(true);
    Ok(request.header(COOKIE, header))
}
pub(super) async fn send(request: RequestBuilder) -> Result<Response> {
    request.send().await.map_err(|error| {
        anyhow!(
            "image_transport_failed ({})",
            super::network_error::classify(&error)
        )
    })
}
pub(super) fn append(output: &mut Vec<u8>, chunk: &[u8], limit: usize) -> Result<()> {
    ensure!(
        output.len() <= limit && chunk.len() <= limit.saturating_sub(output.len()),
        "image_response_too_large"
    );
    output.extend_from_slice(chunk);
    Ok(())
}
pub(super) async fn body(mut response: Response, limit: usize) -> Result<Vec<u8>> {
    ensure!(
        response
            .content_length()
            .is_none_or(|size| size <= limit as u64),
        "image_response_too_large"
    );
    let mut output = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|error| {
        anyhow!(
            "image_response_read_failed ({})",
            super::network_error::classify(&error)
        )
    })? {
        append(&mut output, &chunk, limit)?;
    }
    Ok(output)
}
