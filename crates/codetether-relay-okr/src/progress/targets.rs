//! Load measurable targets only for an associated objective and run.
use std::collections::HashMap;
use uuid::Uuid;
/// Load target values when both objective and run identities are present.
/// # Arguments
/// * `okr_id` - Associated objective.
/// * `okr_run_id` - Associated execution run.
/// # Returns
/// Target values keyed by KR UUID strings, or an empty map when unavailable.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let targets = codetether_relay_okr::progress::targets(None, None).await;
/// assert!(targets.is_empty());
/// # }
/// ```
pub async fn targets(okr_id: Option<Uuid>, okr_run_id: Option<Uuid>) -> HashMap<String, f64> {
    if let (Some(okr_id_val), Some(_run_id)) = (okr_id, okr_run_id) {
        if let Ok(repo) = codetether_okr::persistence::OkrRepository::from_config().await {
            if let Ok(Some(okr)) = repo.get_okr(okr_id_val).await {
                okr.key_results
                    .iter()
                    .map(|kr| (kr.id.to_string(), kr.target_value))
                    .collect()
            } else {
                std::collections::HashMap::new()
            }
        } else {
            std::collections::HashMap::new()
        }
    } else {
        std::collections::HashMap::new()
    }
}
