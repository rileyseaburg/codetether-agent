//! Load full-attention Q/G, K, V and normalization weights.
use super::{attention::Attention, weights::Weights};
use anyhow::Result;
use std::io::{Read, Seek};
pub(super) fn load<R: Read + Seek>(w: &mut Weights<'_, R>) -> Result<Attention> {
    Ok(Attention {
        q: w.linear("attn_q")?,
        k: w.linear("attn_k")?,
        v: w.linear("attn_v")?,
        out: w.linear("attn_output")?,
        qnorm: w.tensor("attn_q_norm.weight")?,
        knorm: w.tensor("attn_k_norm.weight")?,
        cache: None,
    })
}
