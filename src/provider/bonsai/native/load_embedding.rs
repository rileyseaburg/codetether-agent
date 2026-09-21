//! Load the packed host embedding and its inverse-rotation signs.
use super::{Index, config::Config, embedding::Embedding};
use anyhow::{Context, Result, ensure};
use candle_core::Tensor;
use std::{
    collections::HashMap,
    io::{Read, Seek},
};
pub(super) fn load<R: Read + Seek>(
    index: &Index,
    reader: &mut R,
    config: &Config,
    signs: &HashMap<usize, Tensor>,
) -> Result<Embedding> {
    let info = index
        .tensors
        .get("token_embd.weight")
        .context("Missing token embeddings")?;
    ensure!(
        info.kind == 142 && info.shape.len() == 2 && info.shape[0] == config.hidden,
        "Invalid Bonsai embedding geometry"
    );
    Ok(Embedding {
        bytes: index.data(reader, "token_embd.weight")?,
        width: config.hidden,
        rows: info.shape[1],
        signs: signs
            .get(&config.hidden)
            .context("Missing embedding signs")?
            .clone(),
    })
}
