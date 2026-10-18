//! Provider settings selected by the settings panel.
/// Provider configuration setting to advance.
/// # Examples
/// ```
/// let setting = codetether_settings_policy::cycles::Cycle::CodexEffort;
/// ```
#[derive(Clone, Copy)]
pub enum Cycle {
    /// Bedrock service tier.
    BedrockTier,
    /// Bedrock thinking effort.
    BedrockEffort,
    /// Codex model-supported reasoning effort.
    CodexEffort,
    /// OpenRouter reasoning effort.
    OpenRouterEffort,
}
/// Provider configuration and supported-level access.
/// # Implementors
/// Native applications bind their existing thread-safe provider configuration.
pub trait Runtime {
    /// Read the current provider override.
    fn current(setting: Cycle) -> Option<String>;
    /// Set or clear the provider override.
    fn set(setting: Cycle, value: Option<String>);
    /// Return supported effort levels in their existing order.
    fn levels(setting: Cycle, model: &str) -> Vec<&'static str>;
}
