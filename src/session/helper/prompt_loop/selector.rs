//! Parsing of provider/model selectors stored on sessions.

use crate::provider::parse_model_string;
use crate::session::Session;

/// Splits the session selector into an optional provider and model identifier.
pub(super) fn parse(session: &Session, providers: &[&str]) -> (Option<String>, String) {
    let Some(value) = &session.metadata.model else {
        return (None, String::new());
    };
    let (provider, model) = parse_model_string(value);
    let provider = provider.map(|name| match name {
        "zhipuai" | "z-ai" => "zai",
        other => other,
    });
    if let Some(provider) = provider {
        (Some(provider.to_string()), model.to_string())
    } else if providers.contains(&model) {
        (Some(model.to_string()), String::new())
    } else {
        (None, model.to_string())
    }
}
