//! Select one validated hybrid layer and load its residual/MLP weights.
use super::{
    Index,
    layer::{Layer, Mixer},
    weights::Weights,
};
use anyhow::Result;
use candle_core::{Device, Tensor};
use std::{
    collections::HashMap,
    io::{Read, Seek},
};
pub(super) fn load<R: Read + Seek>(
    index: &Index,
    reader: &mut R,
    number: usize,
    signs: &HashMap<usize, Tensor>,
    device: &Device,
) -> Result<Layer> {
    let mut weights = Weights {
        index,
        reader,
        signs,
        device,
        prefix: format!("blk.{number}."),
    };
    let mixer = if (number + 1) % 4 == 0 {
        Mixer::Attention(super::load_attention::load(&mut weights)?)
    } else {
        Mixer::Recurrent(super::load_recurrent::load(&mut weights)?)
    };
    Ok(Layer {
        norm: weights.tensor("attn_norm.weight")?,
        post: weights.tensor("post_attention_norm.weight")?,
        mixer,
        up: weights.linear("ffn_up")?,
        gate: weights.linear("ffn_gate")?,
        down: weights.linear("ffn_down")?,
    })
}
