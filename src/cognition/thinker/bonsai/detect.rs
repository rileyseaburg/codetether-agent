//! Read architecture metadata without interpreting unsupported tensor type IDs.
use anyhow::{Result, ensure};
use std::{fs::File, io::BufReader};
pub(super) fn matches(config: &super::super::ThinkerConfig) -> Result<bool> {
    if let Some(arch) = config.candle_arch.as_deref() {
        return Ok(matches!(arch, "qwen35" | "bonsai2"));
    }
    let Some(path) = config.candle_model_path.as_deref() else {
        return Ok(false);
    };
    let mut reader = super::binary::Reader::new(BufReader::new(File::open(path)?));
    ensure!(reader.bytes(4)? == b"GGUF", "Not a GGUF model");
    ensure!(matches!(reader.u32()?, 2 | 3), "Unsupported GGUF version");
    let _ = reader.u64()?;
    let count = reader.u64()?;
    ensure!(count <= 10000, "GGUF metadata count exceeds limit");
    for _ in 0..count {
        let key = reader.text()?;
        let kind = reader.u32()?;
        let value = super::value::read(&mut reader, kind, 0)?;
        if key == "general.architecture" {
            return Ok(value.as_str() == Some("qwen35"));
        }
    }
    Ok(false)
}
