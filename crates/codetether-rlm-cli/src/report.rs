//! Attach split and persistence metadata to an oracle report.
use codetether_rlm as rlm;

/// Project RLM CLI report information.
/// # Arguments
/// * `result` — Oracle verification outcome.
/// * `split` — Optional split-write statistics.
/// * `persist` — Optional trace persistence result.
/// # Returns
/// JSON report with variant, split and persistence fields.
/// # Examples
/// ```no_run
/// # fn example(result: &codetether_rlm::OracleResult) {
/// let _ = codetether_rlm_cli::oracle_json_value(result, None, None);
/// # }
/// ```
pub fn oracle_json_value(
    result: &rlm::OracleResult,
    split: Option<rlm::oracle::SplitWriteStats>,
    persist: Option<rlm::OracleTracePersistResult>,
) -> serde_json::Value {
    let base = crate::report_base::value(result);

    let mut object = base.as_object().cloned().unwrap_or_default();
    object.insert(
        "split".to_string(),
        serde_json::to_value(split).unwrap_or_default(),
    );
    object.insert(
        "persist".to_string(),
        serde_json::to_value(persist).unwrap_or_default(),
    );
    serde_json::Value::Object(object)
}
