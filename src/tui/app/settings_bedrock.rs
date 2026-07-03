//! Bedrock service-tier cycling for the Settings panel.

use crate::provider::bedrock::runtime_config;
use crate::session::Session;
use crate::tui::app::state::App;

use super::persist;

pub fn bedrock_service_tier_label() -> &'static str {
    match runtime_config::service_tier().unwrap_or_default().as_str() {
        "standard" => "standard",
        "priority" => "priority",
        _ => "default",
    }
}

pub async fn cycle_bedrock_service_tier(app: &mut App, session: &mut Session) {
    let next = match bedrock_service_tier_label() {
        "default" => Some("standard".to_string()),
        "standard" => Some("priority".to_string()),
        _ => None,
    };
    runtime_config::set_service_tier(next);
    persist(
        app,
        session,
        format!("Bedrock service tier: {}", bedrock_service_tier_label()),
    )
    .await;
}
