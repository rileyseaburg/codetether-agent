//! Data retained after a worker task claim succeeds.
use codetether_provenance::ClaimProvenance;
/// Successful claim attribution and optional provider credentials.
///
/// # Examples
/// ```rust
/// use codetether_a2a_http::worker::claim::ClaimedTaskData;
/// let claimed = ClaimedTaskData { claim_provenance: Default::default(), provider_keys: None };
/// assert!(claimed.provider_keys.is_none());
/// ```

pub struct ClaimedTaskData {
    /// Execution attribution returned by the claim.
    pub claim_provenance: ClaimProvenance,
    /// Optional provider credentials returned by the control plane.
    pub provider_keys: Option<serde_json::Value>,
}
