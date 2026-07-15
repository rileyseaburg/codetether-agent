mod context;
mod path;
mod prior_context;
mod prompt;
mod provenance;
mod session_input;

#[cfg(test)]
mod context_tests;

pub use context::{enrich_tool_input_with_runtime_context, insert_field};
#[cfg(test)]
pub(crate) use prior_context::{
    allowed as prior_context_allowed, block as block_prior_context,
    block_serialized as block_serialized_prior_context,
    tool_available as prior_context_tool_available,
};
pub(crate) use prior_context::{
    allowed_for_session as prior_context_allowed_for_session,
    block_runtime_context as block_prior_context_from_runtime,
    block_serialized_for_session as block_serialized_prior_context_for_session,
    remove_tools as remove_prior_context_tools,
    tool_available_for_session as prior_context_tool_available_for_session,
};
pub use prompt::{
    is_codesearch_no_match_output, is_interactive_tool, is_local_cuda_provider,
    local_cuda_light_system_prompt,
};
pub(crate) use session_input::enrich as enrich_tool_input_for_session;
pub(crate) use session_input::enrich_with_model as enrich_tool_input_for_session_model;
