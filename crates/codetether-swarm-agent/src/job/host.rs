//! Application workspace and instruction policy for prepared local jobs.
use codetether_swarm_core::prompt_policy::SystemPromptInput;
/// Native policy used by the shared job-to-agent adapter.
/// # Implementors
/// Applications select their existing workspace registry factory and prompt policy.
pub trait JobHost: crate::Host {
    /// Factory returning the same registry type used by the sub-agent loop.
    type Workspace: codetether_workspace_tools::composition::Factory<Registry = Self::Registry>;
    /// Assemble the application's runtime-aware system instructions.
    fn system_prompt(input: SystemPromptInput<'_>) -> String;
}
