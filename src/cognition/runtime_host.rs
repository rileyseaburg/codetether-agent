//! Bind the shared runtime to native thinker services and executable tools.
pub type Native = codetether_cognition_runtime::application::ApplicationHost<
    super::thinker::Services,
    crate::tool::ToolRegistry,
    { cfg!(test) },
>;
