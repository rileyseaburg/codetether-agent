//! Public verifier type with implementation owned by native goal services.
codetether_native_goal_tools::declare_verifier!(
    pub struct LlmVerifier;,
    crate::tool::registry_lifecycle::Native
);
