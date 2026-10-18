//! Shared runtime values and application resume-configuration conversion.
pub(super) use codetether_collaboration_tools::RuntimeContext;
pub(super) fn resume_config(
    context: &RuntimeContext,
) -> crate::tool::agent::residency::ResumeConfig {
    crate::tool::agent::residency::ResumeConfig::new(
        context.model.clone(),
        context.workspace.clone(),
        context.prior_context_allowed,
    )
}
