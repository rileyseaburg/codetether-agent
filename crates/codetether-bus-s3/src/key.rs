//! Date-partitioned keys for S3 training archives.
use chrono::Utc;

/// Build a date-partitioned, unique S3 object key for a JSONL batch.
pub(super) fn build_s3_key(prefix: &str, now: chrono::DateTime<Utc>) -> String {
    let prefix = if prefix.is_empty() {
        String::new()
    } else if prefix.ends_with('/') {
        prefix.to_string()
    } else {
        format!("{prefix}/")
    };
    let date_path = now.format("%Y/%m/%d/%H").to_string();
    let timestamp = now.format("%Y%m%dT%H%M%S").to_string();
    let uuid = uuid::Uuid::new_v4();
    format!("{prefix}v2/{date_path}/batch_{timestamp}_{uuid}.jsonl")
}
