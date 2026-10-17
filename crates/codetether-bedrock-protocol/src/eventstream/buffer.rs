//! Buffering and extraction of complete eventstream frames.
use super::*;
impl FrameBuffer {
    /// Try to parse the next full frame from the buffer.
    ///
    /// Returns:
    /// - `Ok(Some(msg))` — a complete message was decoded and removed.
    /// - `Ok(None)` — need more bytes.
    /// - `Err(_)` — malformed frame.
    ///
    /// # Errors
    /// Returns an error for an invalid frame length or malformed string header.
    ///
    /// # Panics
    /// May panic on truncated non-string header values: the existing parser
    /// skips these values using unchecked slice lengths.
    ///
    /// # Examples
    /// See [FrameBuffer] for empty-buffer handling.
    pub fn next_frame(&mut self) -> Result<Option<EventMessage>> {
        if self.buf.len() < 12 {
            return Ok(None);
        }
        let total_len = u32::from_be_bytes(
            self.buf[0..4]
                .try_into()
                .map_err(|_| anyhow!("short read"))?,
        ) as usize;
        let headers_len = u32::from_be_bytes(
            self.buf[4..8]
                .try_into()
                .map_err(|_| anyhow!("short read"))?,
        ) as usize;

        if total_len < 16 || headers_len + 16 > total_len {
            return Err(anyhow!(
                "invalid eventstream frame: total_len={total_len}, headers_len={headers_len}"
            ));
        }
        if self.buf.len() < total_len {
            return Ok(None);
        }

        // prelude ends at 12; headers [12, 12+headers_len); payload
        // [12+headers_len, total_len-4); trailing CRC 4 bytes.
        let headers_start = 12usize;
        let headers_end = headers_start + headers_len;
        let payload_end = total_len - 4;

        let headers = parse_headers(&self.buf[headers_start..headers_end])?;
        let payload = self.buf[headers_end..payload_end].to_vec();

        self.buf.drain(..total_len);
        Ok(Some(EventMessage { headers, payload }))
    }
}
