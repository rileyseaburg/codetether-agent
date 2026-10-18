use codetether_runtime_input as context;
codetether_runtime_input::impl_session_source!(crate::session::Session);
use codetether_prompt_policy as light_prompt;
mod prior_context;
use codetether_prompt_policy as prompt;
mod session_input;

pub use context::{enrich_tool_input_with_runtime_context, insert_field};
pub use light_prompt::prefers_light_system_prompt;
#[cfg(not(test))]
pub(crate) use prior_context::{
    allowed_for_session as prior_context_allowed_for_session,
    block_serialized_for_session as block_serialized_prior_context_for_session,
    tool_available_for_session as prior_context_tool_available_for_session,
};
#[cfg(test)]
pub(crate) use prior_context::{
    block as block_prior_context, block_serialized as block_serialized_prior_context,
    tool_available as prior_context_tool_available,
};
pub use prompt::{
    is_codesearch_no_match_output, is_interactive_tool, is_local_cuda_provider,
    local_cuda_light_system_prompt,
};
pub(crate) use session_input::enrich as enrich_tool_input_for_session;
pub(crate) use session_input::enrich_for_turn as enrich_tool_input_for_turn;
pub(crate) use session_input::enrich_with_model as enrich_tool_input_for_session_model;

#[cfg(test)]
include!("mod_test_imports.rs");
