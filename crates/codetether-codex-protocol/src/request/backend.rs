//! Backend identity used to select Responses request fields.

/// Backend whose Responses request fields should be emitted.
///
/// Variants select public OpenAI or the opt-in ChatGPT Codex endpoint.
/// # Examples
/// ```
/// use codetether_codex_protocol::request::ResponsesWsBackend;
/// let accepts_output_limit = match ResponsesWsBackend::OpenAi {
///     ResponsesWsBackend::OpenAi => true,
///     ResponsesWsBackend::ChatGptCodex => false,
/// };
/// assert!(accepts_output_limit);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsesWsBackend {
    /// Public OpenAI Responses service.
    OpenAi,
    /// ChatGPT Codex service with its restricted request fields.
    ChatGptCodex,
}
