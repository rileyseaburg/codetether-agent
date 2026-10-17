//! Decode-loop result contract for Candle inference.

/// Outcome of the autoregressive decode loop.
pub(super) struct Decoded {
    /// Newly generated token IDs.
    pub generated: Vec<u32>,
    /// Why generation stopped: `stop` or `length`.
    pub finish_reason: String,
    /// Additional tokens written into the KV cache.
    pub cache_write_tokens: u32,
}
