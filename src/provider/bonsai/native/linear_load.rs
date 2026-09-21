//! Upload packed matrices, leaving small floating-point projections unquantized.
use super::{Index, linear::Linear, matmul::Pq2};
use anyhow::{Context, Result};
use candle_core::{DType, Device, Tensor};
use std::{
    collections::HashMap,
    io::{Read, Seek},
};
impl Linear {
    pub fn load<R: Read + Seek>(
        index: &Index,
        reader: &mut R,
        name: &str,
        signs: &HashMap<usize, Tensor>,
        device: &Device,
    ) -> Result<Self> {
        let info = index
            .tensors
            .get(name)
            .with_context(|| format!("Missing {name}"))?;
        anyhow::ensure!(info.shape.len() == 2, "Linear weight must be a matrix");
        let (columns, rows) = (info.shape[0], info.shape[1]);
        let data = index.data(reader, name)?;
        if info.kind == 142 {
            let rotation = signs
                .get(&columns)
                .context("Missing rotation signs")?
                .clone();
            let weights = Tensor::from_slice(&data, data.len(), device)?;
            Ok(Self::from_packed(
                weights,
                Pq2 { rows, columns },
                rotation,
                name.contains(".ssm_out."),
            ))
        } else {
            anyhow::ensure!(
                matches!(info.kind, 0 | 30),
                "Unsupported dense projection dtype"
            );
            let dtype = if info.kind == 30 {
                DType::BF16
            } else {
                DType::F32
            };
            let weights = Tensor::from_raw_buffer(&data, dtype, &[rows, columns], device)?
                .to_dtype(DType::F32)?;
            Ok(Self::from_dense(weights))
        }
    }
}
