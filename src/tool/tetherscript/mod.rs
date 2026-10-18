//! TetherScript tool binding, registration and application integration tests.
pub mod convert;
mod host;
use codetether_tetherscript_tool::input;
/// Application backend used by plugin computer grants.
pub use host::ApplicationHost;
/// Plugin tool using the application's computer backend and policy.
///
/// # Examples
/// ```rust
/// let _tool=codetether_agent::tool::tetherscript::TetherScriptPluginTool::new();
/// ```
pub type TetherScriptPluginTool =
    codetether_tetherscript_tool::TetherScriptPluginTool<ApplicationHost>;
#[cfg(test)]
#[path = "runner/process_arity_tests.rs"]
mod process_arity_tests;
#[cfg(test)]
#[path = "runner/process_migration_tests.rs"]
mod process_migration_tests;
#[cfg(test)]
mod tests;
