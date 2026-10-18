//! Belief query filtering and confidence ordering.
use crate::BeliefFilter;
use axum::{Json, http::StatusCode};
use codetether_cognition_core::beliefs::Belief;
/// Filter a captured belief map and order by descending confidence.
/// # Arguments
/// * `beliefs` - Runtime snapshot to project.
/// * `filter` - Optional status substring and exact persona filters.
/// # Returns
/// JSON records in descending confidence order.
/// # Errors
/// This projection currently always succeeds.
/// # Examples
/// ```
/// let filter = codetether_cognition_api::BeliefFilter { status: None, persona: None };
/// assert!(codetether_cognition_api::list_beliefs(Default::default(), filter).unwrap().0.is_empty());
/// ```

pub fn list_beliefs(
    beliefs: std::collections::HashMap<String, Belief>,
    filter: BeliefFilter,
) -> Result<Json<Vec<Belief>>, (StatusCode, String)> {
    let mut result: Vec<Belief> = beliefs.into_values().collect();

    if let Some(status) = &filter.status {
        result.retain(|b| {
            let s = serde_json::to_string(&b.status).unwrap_or_default();
            s.contains(status)
        });
    }
    if let Some(persona) = &filter.persona {
        result.retain(|b| &b.asserted_by == persona);
    }

    result.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(Json(result))
}
