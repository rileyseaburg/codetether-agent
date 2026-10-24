use anyhow::{Result, anyhow, ensure};

pub(super) fn trim_start(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|b| !matches!(b, b'\t' | b'\n' | b'\r' | b' '));
    &bytes[start.unwrap_or(bytes.len())..]
}

pub(super) fn batch_bounds(bytes: &[u8]) -> Result<(usize, usize)> {
    let newline = bytes
        .iter()
        .position(|b| *b == b'\n')
        .ok_or_else(|| anyhow!("truncated_rpc_stream"))?;
    let marker = std::str::from_utf8(&bytes[..newline])
        .map_err(|_| anyhow!("invalid_batch_length"))?
        .trim();
    ensure!(
        !marker.is_empty() && marker.bytes().all(|b| b.is_ascii_digit()),
        "invalid_batch_length"
    );
    let length: usize = marker.parse().map_err(|_| anyhow!("batch_length_limit"))?;
    ensure!(
        (1..=16 * 1024 * 1024).contains(&length),
        "batch_length_limit"
    );
    let end = newline
        .checked_add(length)
        .ok_or_else(|| anyhow!("batch_length_limit"))?;
    ensure!(end <= bytes.len(), "truncated_rpc_stream");
    Ok((newline, end))
}
