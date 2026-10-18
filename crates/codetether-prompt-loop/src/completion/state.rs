//! State retained across retries of one provider step.
use codetether_context_policy::{
    derivation::DerivePolicy, projection::helpers::DerivedContext, relevance::Bucket,
};
use codetether_provider_core::Message;

/// Context and counters retained while retrying one provider step.
///
/// # Examples
/// ```
/// # use codetether_context_policy::{projection::helpers::DerivedContext, relevance::bucket_for_messages};
/// let attempt = codetether_prompt_loop::completion::Attempt {
///     derived: DerivedContext { messages: vec![], origin_len: 0,
///         compressed: false, dropped_ranges: vec![], provenance: vec![], resolutions: vec![] },
///     proactive: None, bucket: bucket_for_messages(&[]), policy: Default::default(),
///     count: 0, upstream_retries: 0,
/// };
/// assert_eq!(attempt.upstream_retries, 0);
/// ```
pub struct Attempt {
    /// Derived message history sent to the provider.
    pub derived: DerivedContext,
    /// Optional proactive language-server context message.
    pub proactive: Option<Message>,
    /// Delegation bucket used for routing feedback.
    pub bucket: Bucket,
    /// Baseline context derivation policy for the step.
    pub policy: DerivePolicy,
    /// Provider attempts made with the current target.
    pub count: usize,
    /// Transient upstream failures observed during the step.
    pub upstream_retries: u8,
}
