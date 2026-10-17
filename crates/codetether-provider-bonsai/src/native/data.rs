//! Read one validated tensor payload without dequantizing the model.
use super::Index;
use anyhow::{Context, Result};
use std::io::{Read, Seek, SeekFrom};
impl Index {
    pub fn data<R: Read + Seek>(&self, reader: &mut R, name: &str) -> Result<Vec<u8>> {
        let tensor = self.tensors.get(name).context("Missing Bonsai tensor")?;
        let offset = self
            .data_offset
            .checked_add(tensor.offset)
            .context("Tensor offset overflow")?;
        reader.seek(SeekFrom::Start(offset))?;
        let mut bytes = vec![0; usize::try_from(tensor.bytes)?];
        reader.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}
