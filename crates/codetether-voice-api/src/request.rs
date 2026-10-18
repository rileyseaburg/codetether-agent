//! Voice-session REST request contract.
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CreateVoiceSessionRequest {
    pub voice: Option<String>,
    pub mode: Option<String>,
    pub codebase_id: Option<String>,
    pub user_id: Option<String>,
}
