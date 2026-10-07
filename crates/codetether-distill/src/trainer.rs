//! LoRA fine-tuning pipeline for self-training.

use anyhow::Result;
use std::path::Path;

/// Training configuration for LoRA fine-tune.
#[derive(Debug, Clone)]
pub struct TrainConfig {
    pub base_model_path: String,
    pub output_dir: String,
    pub lora_rank: usize,
    pub learning_rate: f32,
    pub epochs: usize,
    pub batch_size: usize,
}

impl Default for TrainConfig {
    fn default() -> Self {
        Self {
            base_model_path: String::new(),
            output_dir: ".codetether/lora".to_string(),
            lora_rank: 16,
            learning_rate: 2e-4,
            epochs: 3,
            batch_size: 4,
        }
    }
}

/// Result of a training run.
#[derive(Debug, Clone)]
pub struct TrainResult {
    pub adapter_path: String,
    pub version: usize,
    pub train_loss: f32,
    pub val_loss: f32,
    pub records_used: usize,
}

/// Launch a LoRA fine-tuning run (stub — wires to Candle when LoRA support lands).
pub fn train_lora(config: &TrainConfig, records_path: &Path) -> Result<TrainResult> {
    let version = std::fs::read_dir(&config.output_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    e.file_name()
                        .to_str()?
                        .strip_prefix('v')?
                        .parse::<usize>()
                        .ok()
                })
                .max()
                .unwrap_or(0)
                + 1
        })
        .unwrap_or(1);
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
