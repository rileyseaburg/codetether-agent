//! Exact Prism group-128 PQ2_0 decoding: fp16 scale plus 32 packed bytes.
//! Reference: PrismML llama.cpp d8f26eec7, dequantize_row_pq2_0.
use anyhow::{Result, ensure};
pub(super) const BLOCK_VALUES: usize = 128;
pub(super) const BLOCK_BYTES: usize = 34;
pub(super) fn scale(bits: u16) -> f32 {
    let sign = if bits & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exponent = (bits >> 10) & 31;
    let mantissa = (bits & 1023) as f32;
    match exponent {
        0 => sign * mantissa * 2f32.powi(-24),
        31 if mantissa == 0.0 => sign * f32::INFINITY,
        31 => f32::NAN,
        _ => sign * (1.0 + mantissa / 1024.0) * 2f32.powi(exponent as i32 - 15),
    }
}
pub(super) fn decode(block: &[u8]) -> Result<[f32; BLOCK_VALUES]> {
    ensure!(block.len() == BLOCK_BYTES, "PQ2_0 block must be 34 bytes");
    let delta = scale(u16::from_le_bytes([block[0], block[1]]));
    ensure!(delta.is_finite(), "PQ2_0 block has non-finite scale");
    Ok(std::array::from_fn(|i| {
        let q = (block[2 + i / 4] >> ((i % 4) * 2)) & 3;
        (q as f32 - 1.0) * delta
    }))
}
