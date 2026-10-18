//! Application package identity supplied to the shared root parser.
/// Package identity used by root command help and version output.
/// # Implementors
/// Applications provide their own package version and description so the CLI
/// retains the identity of the executable that embeds the shared schema.
pub trait ApplicationMetadata: std::fmt::Debug {
    /// Application package version displayed by `--version`.
    const VERSION: &'static str;
    /// Application description displayed in root help.
    const ABOUT: &'static str;
}
