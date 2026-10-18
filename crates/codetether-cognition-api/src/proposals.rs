//! Proposal response ordering.
use axum::{Json, http::StatusCode};
use codetether_cognition_core::Proposal;
/// Order captured proposals newest first.
/// # Arguments
/// * `proposals` - Runtime proposal snapshot.
/// # Returns
/// JSON records ordered by descending creation time.
/// # Errors
/// This projection currently always succeeds.
/// # Examples
/// ```
/// assert!(codetether_cognition_api::list_proposals(Default::default()).unwrap().0.is_empty());
/// ```

pub fn list_proposals(
    proposals: std::collections::HashMap<String, Proposal>,
) -> Result<Json<Vec<Proposal>>, (StatusCode, String)> {
    let mut result: Vec<Proposal> = proposals.into_values().collect();
    result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(Json(result))
}
