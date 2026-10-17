//! Current training stub: select an output path and report empty metrics.
use super::{TrainConfig, TrainResult, version::next_version};
use anyhow::Result;
use std::path::Path;
/// Launch a LoRA fine-tuning run (stub — wires to Candle when LoRA support lands).
///
/// # Arguments
/// * `config` - Output directory and future training parameters.
/// * `records_path` - Logged input path; the current stub does not read it.
///
/// # Returns
/// The next adapter version and placeholder zero-valued metrics. No files are
/// written and no training is performed.
///
/// # Errors
/// The current stub returns no errors; the result type permits future training
/// failures. Unreadable output directories currently select version one.
///
/// # Panics
/// Version increment may overflow when the largest existing version is
/// `usize::MAX` and integer overflow checks are enabled.
///
/// # Examples
/// ```no_run
/// use codetether_distill::trainer::{TrainConfig, train_lora};
/// let result = train_lora(&TrainConfig::default(), std::path::Path::new("runs.jsonl"))?;
/// assert_eq!(result.records_used, 0);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn train_lora(config: &TrainConfig, records_path: &Path) -> Result<TrainResult> {
    let version = next_version(Path::new(&config.output_dir));
    let adapter_path = format!("{}/v{}", config.output_dir, version);
    tracing::info!(
        version, records_path = %records_path.display(), base_model = %config.base_model_path,
        "LoRA training initiated (stub)"
    );
    Ok(TrainResult {
        adapter_path,
        version,
        train_loss: 0.0,
        val_loss: 0.0,
        records_used: 0,
    })
}
