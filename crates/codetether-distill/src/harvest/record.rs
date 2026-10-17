//! Serialized record exchanged with training consumers.
/// Harvested training record.
///
/// # Examples
/// ```
/// use codetether_distill::harvest::TrainingRecord;
/// let record = TrainingRecord {
///     prompt: "task".into(), completion: "answer".into(), quality_score: 0.9,
///     source_run_id: "run-1".into(), tools_used: vec![],
/// };
/// assert_eq!(record.source_run_id, "run-1");
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrainingRecord {
    pub prompt: String,
    pub completion: String,
    pub quality_score: f32,
    pub source_run_id: String,
    pub tools_used: Vec<String>,
}
