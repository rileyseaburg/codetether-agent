//! Native thinker and tool specialization of shared loop handles.
pub(super) type LoopCtx = codetether_cognition_core::runtime::LoopContext<
    super::ThinkerClient,
    crate::tool::ToolRegistry,
>;
