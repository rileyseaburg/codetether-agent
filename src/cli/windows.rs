//! Application parser coverage for the shared Windows diagnostics command.
pub use codetether_windows_cli::{WindowsArgs, WindowsCommand, run};

#[cfg(test)]
#[path = "windows_tests.rs"]
mod tests;
