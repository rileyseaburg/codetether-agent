//! Start/stop control payloads for perpetual cognition.

use serde::{Deserialize, Serialize};

use super::CreatePersonaRequest;

/// Start-control payload for perpetual cognition.
///
/// # Examples
/// ```
/// use codetether_cognition_core::control::StartCognitionRequest;
/// let _record = StartCognitionRequest { loop_interval_ms: Some(1000), seed_persona: None };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCognitionRequest {
    pub loop_interval_ms: Option<u64>,
    pub seed_persona: Option<CreatePersonaRequest>,
}

/// Stop-control payload for perpetual cognition.
///
/// # Examples
/// ```
/// use codetether_cognition_core::control::StopCognitionRequest;
/// let _record = StopCognitionRequest { reason: Some("operator request".into()) };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopCognitionRequest {
    pub reason: Option<String>,
}
