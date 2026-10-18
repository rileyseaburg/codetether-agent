//! Native goal storage and audit services with the application verifier.
pub(super) type Native = codetether_native_goal_tools::Host<super::verify::LlmVerifier>;
