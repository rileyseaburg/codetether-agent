//! Request validation and per-request sampling, separate from loaded model state.
use crate::provider::CompletionRequest;
use anyhow::{Result, ensure};
pub(super) struct GenerationRequest {
    pub prompt: String,
    pub temperature: f64,
    pub top_p: Option<f64>,
    pub max_tokens: usize,
    pub stop: Vec<String>,
}
impl GenerationRequest {
    pub fn parse(request: &CompletionRequest) -> Result<Self> {
        ensure!(
            request.model == super::MODEL,
            "Unknown Bonsai model: {}",
            request.model
        );
        let max_tokens = request.max_tokens.unwrap_or(256);
        ensure!(
            (1..=1024).contains(&max_tokens),
            "Bonsai max_tokens must be 1..=1024"
        );
        let temperature = f64::from(request.temperature.unwrap_or(0.0));
        ensure!(
            temperature.is_finite() && (0.0..=2.0).contains(&temperature),
            "Invalid temperature"
        );
        let top_p = request.top_p.map(f64::from);
        ensure!(
            top_p.is_none_or(|p| p.is_finite() && p > 0.0 && p <= 1.0),
            "Invalid top_p"
        );
        ensure!(
            request
                .stop
                .iter()
                .all(|s| !s.is_empty() && s.len() <= 1024),
            "Invalid stop string"
        );
        Ok(Self {
            prompt: super::protocol::render(request)?,
            temperature,
            top_p,
            max_tokens,
            stop: request.stop.clone(),
        })
    }
}
