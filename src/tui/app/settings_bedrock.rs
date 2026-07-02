//! Bedrock service-tier cycling for the Settings panel.

use crate::session::Session;
use crate::tui::app::state::App;

use super::persist;

pub fn bedrock_service_tier_label() -> &'static str {
    match std::env::var("CODETETHER_BEDROCK_SERVICE_TIER")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "standard" => "standard",
        "priority" => "priority",
        _ => "default",
    }
}

pub async fn cycle_bedrock_service_tier(app: &mut App, session: &mut Session) {
    let next = match bedrock_service_tier_label() {
        "default" => Some("standard"),
        "standard" => Some("priority"),
        _ => None,
    };
    if let Some(value) = next {
        unsafe { std::env::set_var("CODETETHER_BEDROCK_SERVICE_TIER", value) }
    } else {
        unsafe { std::env::remove_var("CODETETHER_BEDROCK_SERVICE_TIER") }
    }
    persist(
        app,
        session,
        format!("Bedrock service tier: {}", bedrock_service_tier_label()),
    )
    .await;
}
