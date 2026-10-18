//! Shared browser-control CLI with native alias coverage.
pub use codetether_browser_cli::{BrowserCtlArgs, BrowserCtlCommand, OfflineCommand, execute};
#[cfg(test)]
mod tests;
