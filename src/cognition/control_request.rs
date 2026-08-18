//! Start/stop control payloads for perpetual cognition.

use serde::{Deserialize, Serialize};

use super::CreatePersonaRequest;

/// Start-control payload for perpetual cognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCognitionRequest {
    pub loop_interval_ms: Option<u64>,
    pub seed_persona: Option<CreatePersonaRequest>,
}

/// Stop-control payload for perpetual cognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopCognitionRequest {
    pub reason: Option<String>,
}
