//! Provider configuration cycles and their persistence/status sequencing.
//!
//! Implement [`Runtime`] for provider configuration, then call [`cycle`] with
//! the existing session and view persistence adapters.
mod bedrock;
mod reasoning;
mod runtime;
use crate::persistence::{Session, View, save};
pub use runtime::{Cycle, Runtime};
/// Cycle one provider setting and save the session with its status text.
/// # Arguments
/// * `view` - Settings status output.
/// * `session` - Existing session storage.
/// * `model` - Current session model for supported-level lookup.
/// * `setting` - Provider setting to advance.
/// # Returns
/// Completes after saving, or after reporting an unsupported Codex model.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_settings_policy::cycles::Runtime, V: codetether_settings_policy::persistence::View, S: codetether_settings_policy::persistence::Session>(view: &mut V, session: &S) {
/// codetether_settings_policy::cycles::cycle::<R, V, S>(view, session, "", codetether_settings_policy::cycles::Cycle::BedrockTier).await;
/// # }
/// ```
pub async fn cycle<R: Runtime, V: View, S: Session>(
    view: &mut V,
    session: &S,
    model: &str,
    setting: Cycle,
) {
    let message = match setting {
        Cycle::BedrockTier => bedrock::tier::<R>(),
        Cycle::BedrockEffort => bedrock::effort::<R>(),
        Cycle::CodexEffort | Cycle::OpenRouterEffort => {
            match reasoning::advance::<R>(model, setting) {
                Some(message) => message,
                None => {
                    view.set_status(
                        "Select an OpenAI Codex model before changing its effort".to_string(),
                    );
                    return;
                }
            }
        }
    };
    save(view, session, message).await;
}
