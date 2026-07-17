//! Build-agent prompt constants.
//!
//! This module contains the raw build prompt template and the appended
//! non-interactive build-mode guardrail.
//!
//! # Examples
//!
//! ```ignore
//! assert!(BUILD_MODE_GUARDRAIL.contains("build mode"));
//! ```

/// Raw build-agent system prompt template.
///
/// # Examples
///
/// ```ignore
/// assert!(BUILD_SYSTEM_PROMPT.contains("{cwd}"));
/// ```
#[allow(dead_code)]
pub const BUILD_SYSTEM_PROMPT: &str = include_str!("build_base.md");

/// Additional non-interactive guidance appended to build prompts.
///
/// # Examples
///
/// ```ignore
/// assert!(BUILD_MODE_GUARDRAIL.contains("do not ask the user"));
/// ```
pub const BUILD_MODE_GUARDRAIL: &str = "\n\n## Build Mode Guardrail\n\
In build mode, do not ask the user for permission to continue (for example: \
\"should I go ahead?\", \"want me to proceed?\"). \
Execute the next concrete implementation step directly unless a hard blocker exists \
(missing requirements, destructive risk, or unavailable dependencies). \
When tools are available and the request is implementation/debugging work, \
start by using tools instead of giving a plan-only response. \
Never respond with \"if you want, I can...\" in build mode.";
