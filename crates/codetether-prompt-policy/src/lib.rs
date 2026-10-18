//! Prompt selection and tool classifications used by the agent loop.
//!
//! [prefers_light_system_prompt] selects the compact prompt for local CUDA
//! providers or explicit environment opt-in. [local_cuda_light_system_prompt]
//! supplies its text. Tool helpers classify interaction and empty search output.
mod compact;
mod interactive;
mod provider;
mod search_result;
mod selection;
pub use compact::local_cuda_light_system_prompt;
pub use interactive::is_interactive_tool;
pub use provider::is_local_cuda_provider;
pub use search_result::is_codesearch_no_match_output;
pub use selection::prefers_light_system_prompt;
#[cfg(test)]
mod tests;

/// Limits and continuation messages used by the native prompt loop.
pub mod loop_constants;
