//! Tensor descriptors with exact byte sizes for Prism PQ2_0, F32 and BF16.
use super::binary::Reader;
use anyhow::{Result, bail, ensure};
use std::io::{Read, Seek};
#[derive(Debug)]
pub(super) struct TensorInfo {
    pub shape: Vec<usize>,
    pub kind: u32,
    pub offset: u64,
    pub bytes: u64,
}
pub(super) fn read<R: Read + Seek>(r: &mut Reader<R>) -> Result<(String, TensorInfo)> {
    let name = r.text()?;
    let rank = r.u32()?;
    ensure!((1..=4).contains(&rank), "Invalid GGUF tensor rank");
    let shape = (0..rank)
        .map(|_| Ok(usize::try_from(r.u64()?)?))
        .collect::<Result<Vec<_>>>()?;
    let kind = r.u32()?;
    let offset = r.u64()?;
    ensure!(
        shape.iter().all(|n| *n > 0),
        "GGUF tensor has an empty dimension"
    );
    let elements = shape
        .iter()
        .try_fold(1u64, |n, d| n.checked_mul(*d as u64))
        .ok_or_else(|| anyhow::anyhow!("GGUF tensor size overflow"))?;
    let bytes = match kind {
        0 => elements.checked_mul(4),
        30 => elements.checked_mul(2),
        142 => {
            ensure!(shape[0] % 128 == 0, "PQ2_0 rows require 128-element blocks");
            (elements / 128).checked_mul(34)
        }
        _ => bail!("Unsupported native Bonsai tensor type {kind}"),
    }
    .ok_or_else(|| anyhow::anyhow!("GGUF tensor byte size overflow"))?;
    Ok((
        name,
        TensorInfo {
            shape,
            kind,
            offset,
            bytes,
        },
    ))
}
