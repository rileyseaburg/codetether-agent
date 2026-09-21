//! Decode a bounded GGUF header and directory.
use super::{Index, binary::Reader, tensor_info, value};
use anyhow::{Result, ensure};
use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom},
};
impl Index {
    pub fn read<R: Read + Seek>(input: &mut R) -> Result<Self> {
        input.seek(SeekFrom::Start(0))?;
        let mut r = Reader::new(input);
        ensure!(r.bytes(4)? == b"GGUF", "Not a GGUF file");
        ensure!(matches!(r.u32()?, 2 | 3), "Unsupported GGUF version");
        let tensors = r.u64()?;
        let fields = r.u64()?;
        ensure!(
            tensors <= 100_000 && fields <= 10_000,
            "GGUF directory is too large"
        );
        let mut metadata = HashMap::new();
        for _ in 0..fields {
            let key = r.text()?;
            let kind = r.u32()?;
            ensure!(
                metadata
                    .insert(key, value::read(&mut r, kind, 0)?)
                    .is_none(),
                "Duplicate GGUF metadata"
            );
        }
        let mut directory = HashMap::new();
        for _ in 0..tensors {
            let (name, tensor) = tensor_info::read(&mut r)?;
            ensure!(
                directory.insert(name, tensor).is_none(),
                "Duplicate GGUF tensor"
            );
        }
        let data_offset = super::index_ranges::validate(&mut r.inner, &metadata, &directory)?;
        Ok(Self {
            metadata,
            tensors: directory,
            data_offset,
        })
    }
}
