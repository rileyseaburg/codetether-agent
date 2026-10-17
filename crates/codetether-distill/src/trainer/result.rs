//! Adapter location and metrics reported by a training run.
/// Result of a training run.
///
/// # Examples
/// ```
/// use codetether_distill::trainer::TrainResult;
/// let result = TrainResult { adapter_path: "adapters/v1".into(), version: 1,
///     train_loss: 0.0, val_loss: 0.0, records_used: 0 };
/// assert_eq!(result.version, 1);
/// ```
#[derive(Debug, Clone)]
pub struct TrainResult {
    pub adapter_path: String,
    pub version: usize,
    pub train_loss: f32,
    pub val_loss: f32,
    pub records_used: usize,
}
