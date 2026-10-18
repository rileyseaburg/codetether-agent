//! Preserve application-version identity for the shared Copilot wrapper.
/// Application identity selected by the public Copilot provider alias.
pub struct App;
impl codetether_provider_copilot::application::Identity for App {
    fn user_agent() -> String {
        format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"))
    }
}
