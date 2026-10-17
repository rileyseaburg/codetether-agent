//! Restrict the first native decoder to the verified checkpoint geometry.
use super::{Index, config::Config};
use anyhow::{Result, ensure};
pub(super) fn validate(config: &Config, index: &Index) -> Result<()> {
    let n = |key: &str| super::metadata::integer(index, key);
    ensure!(
        (
            config.layers,
            config.hidden,
            config.heads,
            config.kv_heads,
            config.head,
            config.keys,
            config.values,
            config.state,
            config.conv,
            config.rotary
        ) == (64, 5120, 24, 4, 256, 16, 48, 128, 4, 64),
        "Unsupported Bonsai geometry"
    );
    ensure!(
        n("qwen35.full_attention_interval")? == 4 && n("qwen35.attention.value_length")? == 256,
        "Unsupported attention layout"
    );
    ensure!(
        config.eps.is_finite() && config.eps > 0.0 && config.base.is_finite() && config.base > 1.0,
        "Invalid normalization/RoPE constants"
    );
    ensure!(
        index
            .metadata
            .get("qwen35.context_length")
            .and_then(serde_json::Value::as_u64)
            .is_some_and(|n| n >= config.context as u64),
        "Invalid trained context bound"
    );
    ensure!(
        index
            .metadata
            .get("qwen35.rope.dimension_sections")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|v| v.len() == 4
                && v.iter().all(|n| n.as_u64().is_some())
                && v.iter().filter_map(serde_json::Value::as_u64).sum::<u64>() == 32),
        "Unsupported MRoPE geometry"
    );
    Ok(())
}
