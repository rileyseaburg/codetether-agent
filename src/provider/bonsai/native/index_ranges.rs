//! Alignment, file bounds and overlap validation for tensor payloads.
use super::tensor_info::TensorInfo;
use anyhow::{Result, ensure};
use serde_json::Value;
use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom},
};
pub(super) fn validate<R: Read + Seek>(
    reader: &mut R,
    metadata: &HashMap<String, Value>,
    directory: &HashMap<String, TensorInfo>,
) -> Result<u64> {
    let alignment = metadata
        .get("general.alignment")
        .and_then(Value::as_u64)
        .unwrap_or(32);
    ensure!(
        alignment.is_power_of_two() && alignment <= 65536,
        "Invalid GGUF alignment"
    );
    let end = reader.stream_position()?;
    let data_offset = end
        .checked_add(alignment - 1)
        .ok_or_else(|| anyhow::anyhow!("GGUF offset overflow"))?
        & !(alignment - 1);
    let size = reader.seek(SeekFrom::End(0))?;
    let mut ranges = Vec::new();
    for tensor in directory.values() {
        let start = data_offset
            .checked_add(tensor.offset)
            .ok_or_else(|| anyhow::anyhow!("GGUF offset overflow"))?;
        let end = start
            .checked_add(tensor.bytes)
            .ok_or_else(|| anyhow::anyhow!("GGUF tensor overflow"))?;
        ensure!(
            tensor.offset % alignment == 0 && end <= size,
            "GGUF tensor extends outside the model"
        );
        ranges.push((start, end));
    }
    ranges.sort_unstable();
    ensure!(
        ranges.windows(2).all(|w| w[0].1 <= w[1].0),
        "GGUF tensor payloads overlap"
    );
    Ok(data_offset)
}
