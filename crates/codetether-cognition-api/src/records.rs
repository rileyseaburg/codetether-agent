//! Optional runtime records mapped to their existing HTTP responses.
use axum::{Json, http::StatusCode};
use codetether_cognition_core::beliefs::Belief;
use codetether_cognition_core::{MemorySnapshot, PersonaRuntimeState};
/// Return the latest snapshot or the existing 404 response.
/// # Arguments
/// * `snapshot` - Latest runtime snapshot.
/// # Returns
/// JSON snapshot when present.
/// # Errors
/// Returns 404 when no snapshot exists.
/// # Examples
/// ```
/// assert!(codetether_cognition_api::latest_snapshot(None).is_err());
/// ```
pub fn latest_snapshot(
    snapshot: Option<MemorySnapshot>,
) -> Result<Json<MemorySnapshot>, (StatusCode, String)> {
    match snapshot {
        Some(snapshot) => Ok(Json(snapshot)),
        None => Err((StatusCode::NOT_FOUND, "No snapshots available".to_string())),
    }
}
/// Return a belief or the existing ID-specific 404 response.
/// # Arguments
/// * `belief` - Runtime lookup result.
/// * `id` - Requested belief ID.
/// # Returns
/// JSON belief when present.
/// # Errors
/// Returns 404 for an absent belief.
/// # Examples
/// ```
/// assert!(codetether_cognition_api::belief(None, "missing").is_err());
/// ```
pub fn belief(belief: Option<Belief>, id: &str) -> Result<Json<Belief>, (StatusCode, String)> {
    match belief {
        Some(belief) => Ok(Json(belief)),
        None => Err((StatusCode::NOT_FOUND, format!("Belief not found: {}", id))),
    }
}
/// Return a persona or the existing ID-specific 404 response.
/// # Arguments
/// * `persona` - Runtime lookup result.
/// * `id` - Requested persona ID.
/// # Returns
/// JSON persona when present.
/// # Errors
/// Returns 404 for an absent persona.
/// # Examples
/// ```
/// assert!(codetether_cognition_api::persona(None, "missing").is_err());
/// ```
pub fn persona(
    persona: Option<PersonaRuntimeState>,
    id: &str,
) -> Result<Json<PersonaRuntimeState>, (StatusCode, String)> {
    persona
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Persona not found: {}", id)))
}
