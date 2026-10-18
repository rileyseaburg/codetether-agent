use codetether_provider_core::ToolDefinition;
/// Borrowed model settings for one completion attempt.
///
/// # Examples
/// ```
/// let target = codetether_context_request::attempt::Target {
///     system_prompt: "Help the user", provider: "example", model: "model",
///     tools: &[], temperature: None,
/// };
/// assert!(target.tools.is_empty());
/// ```
pub struct Target<'a> {
    /// System instructions placed before derived history.
    pub system_prompt: &'a str,
    /// Provider identity used for reasoning compatibility.
    pub provider: &'a str,
    /// Model identifier sent to the provider.
    pub model: &'a str,
    /// Advertised tool definitions.
    pub tools: &'a [ToolDefinition],
    /// Optional sampling temperature.
    pub temperature: Option<f32>,
}
