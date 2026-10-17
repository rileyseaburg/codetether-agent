//! Metadata-to-geometry construction.
use super::{Index, config::Config};
use anyhow::Result;
impl Config {
    pub fn read(index: &Index) -> Result<Self> {
        let n = |key: &str| super::metadata::integer(index, key);
        let config = Self {
            layers: n("qwen35.block_count")?,
            hidden: n("qwen35.embedding_length")?,
            heads: n("qwen35.attention.head_count")?,
            kv_heads: n("qwen35.attention.head_count_kv")?,
            head: n("qwen35.attention.key_length")?,
            keys: n("qwen35.ssm.group_count")?,
            values: n("qwen35.ssm.time_step_rank")?,
            state: n("qwen35.ssm.state_size")?,
            conv: n("qwen35.ssm.conv_kernel")?,
            rotary: n("qwen35.rope.dimension_count")?,
            eps: super::metadata::float(index, "qwen35.attention.layer_norm_rms_epsilon")?,
            base: super::metadata::float(index, "qwen35.rope.freq_base")?,
            context: 4096,
        };
        super::config_check::validate(&config, index)?;
        Ok(config)
    }
}
