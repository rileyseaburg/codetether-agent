//! Bind the shared root parser to this application's package metadata.
#[derive(Debug)]
/// Application package identity for shared root command metadata.
pub struct Application;
impl codetether_cli_command::root::ApplicationMetadata for Application {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");
}
/// Root CodeTether arguments with application-specific help and version text.
pub type Cli = codetether_cli_command::root::Cli<Application>;
#[cfg(test)]
#[path = "root_args_tests.rs"]
mod tests;
