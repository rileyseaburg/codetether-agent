//! Decode typed GGUF metadata without consuming tensor payloads.
use super::binary::Reader;
use anyhow::{Result, bail, ensure};
use serde_json::{Value, json};
use std::io::{Read, Seek};
pub(super) fn read<R: Read + Seek>(r: &mut Reader<R>, kind: u32, depth: usize) -> Result<Value> {
    ensure!(depth < 8, "GGUF arrays are nested too deeply");
    Ok(match kind {
        0 => json!(r.bytes(1)?[0]),
        1 => json!(r.bytes(1)?[0] as i8),
        2 => json!(u16::from_le_bytes(r.bytes(2)?.try_into().unwrap())),
        3 => json!(i16::from_le_bytes(r.bytes(2)?.try_into().unwrap())),
        4 => json!(r.u32()?),
        5 => json!(r.u32()? as i32),
        6 => json!(f32::from_bits(r.u32()?)),
        7 => {
            let n = r.bytes(1)?[0];
            ensure!(n <= 1, "Invalid GGUF boolean");
            json!(n != 0)
        }
        8 => json!(r.text()?),
        9 => {
            let item = r.u32()?;
            let count = usize::try_from(r.u64()?)?;
            ensure!(count <= 2_000_000, "GGUF metadata array is too large");
            let items = (0..count)
                .map(|_| read(r, item, depth + 1))
                .collect::<Result<Vec<_>>>()?;
            Value::Array(items)
        }
        10 => json!(r.u64()?),
        11 => json!(r.u64()? as i64),
        12 => json!(f64::from_bits(r.u64()?)),
        _ => bail!("Unsupported GGUF metadata type {kind}"),
    })
}
