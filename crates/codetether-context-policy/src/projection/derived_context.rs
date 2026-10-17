//! Provider-ready derived context and source provenance.
use crate::pages::ResidencyLevel;
use codetether_provider_core::Message;

/// The per-step LLM context, derived from an append-only chat history.
///
/// This is the object the prompt loop should hand to the provider — *not*
/// application session messages directly.
///
/// # Fields
///
/// * `messages` — The message list to include in the completion request.
/// * `origin_len` — Length of the source history at the moment of derivation.
/// * `compressed` — Whether compression fired during this derivation.
/// * `dropped_ranges` — Best-effort ranges elided from the working context.
/// * `provenance` — Pipeline steps that produced this context.
/// * `resolutions` — Per-message residency levels in the derived context.
///
/// # Examples
///
/// ```rust
/// use codetether_context_policy::projection::helpers::DerivedContext;
///
/// let derived = DerivedContext {
///     messages: Vec::new(),
///     origin_len: 0,
///     compressed: false,
///     dropped_ranges: Vec::new(),
///     provenance: Vec::new(),
///     resolutions: Vec::new(),
/// };
/// assert_eq!(derived.origin_len, 0);
/// assert!(!derived.compressed);
/// ```
#[derive(Debug, Clone)]
pub struct DerivedContext {
    /// Messages to send to the provider this turn.
    pub messages: Vec<Message>,
    /// `session.messages.len()` at the moment of derivation.
    pub origin_len: usize,
    /// `true` when any compression / truncation pass rewrote the clone.
    pub compressed: bool,
    /// Best-effort elided ranges from the source history.
    pub dropped_ranges: Vec<(usize, usize)>,
    /// Names of the pipeline stages that shaped this context.
    pub provenance: Vec<String>,
    /// Per-message residency level in the derived context.
    pub resolutions: Vec<ResidencyLevel>,
}
