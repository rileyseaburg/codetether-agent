use super::TaskCheckpoint;

#[derive(Debug, Clone)]
pub struct CheckpointEntry {
    pub checkpoint: TaskCheckpoint,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub elapsed_ms: u64,
    pub detail: Option<String>,
}
