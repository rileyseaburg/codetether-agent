//! Load Gated Delta Net projections and small unquantized recurrence parameters.
use super::{linear_attention::Recurrent, weights::Weights};
use anyhow::Result;
use std::io::{Read, Seek};
pub(super) fn load<R: Read + Seek>(w: &mut Weights<'_, R>) -> Result<Recurrent> {
    Ok(Recurrent {
        qkv: w.linear("attn_qkv")?,
        gate: w.linear("attn_gate")?,
        alpha: w.linear("ssm_alpha")?,
        beta: w.linear("ssm_beta")?,
        out: w.linear("ssm_out")?,
        conv: w.tensor("ssm_conv1d.weight")?,
        dt: w.tensor("ssm_dt.bias")?,
        a: w.tensor("ssm_a")?,
        norm: w.tensor("ssm_norm.weight")?,
        history: None,
        state: None,
    })
}
