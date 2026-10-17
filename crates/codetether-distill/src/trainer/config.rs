//! Configuration passed to the training entry point.
/// Training configuration for LoRA fine-tune.
///
/// # Examples
/// ```
/// use codetether_distill::trainer::TrainConfig;
/// let config = TrainConfig::default();
/// assert_eq!(config.lora_rank, 16);
/// ```
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
