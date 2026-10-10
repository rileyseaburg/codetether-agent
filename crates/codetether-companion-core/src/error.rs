/// Redacted lifecycle failures, independent of an HTTP framework.
///
/// Variants distinguish invalid input, authentication, limits, missing/ended
/// sessions, and internal configuration/clock/entropy failures. Never include
/// bearer credentials, pairing codes, or user prompts in error text.
///
/// ```
/// use codetether_companion_core::Error;
/// match Error::Ended { Error::Ended => assert_eq!(Error::Ended.status(), 410), _ => unreachable!() }
/// ```
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Invalid model, instructions, or interval.
    #[error("Choose a provider/model, instructions, and an interval from 15 to 300 seconds")]
    Input,
    /// Missing or malformed bearer header.
    #[error("Authentication required")]
    AuthenticationRequired,
    /// Well-formed but incorrect bearer token.
    #[error("Authentication rejected")]
    AuthenticationRejected,
    /// A supplied browser origin is not the configured origin.
    #[error("Cross-origin requests are not allowed")]
    Origin,
    /// Pairing code does not identify an eligible session.
    #[error("Pairing code invalid or expired")]
    Pairing,
    /// Global pairing attempt budget exceeded.
    #[error("Too many pairing attempts; wait one minute")]
    Attempts,
    /// Four live sessions already exist.
    #[error("Stop an existing screen session first")]
    Capacity,
    /// Session was never registered or has been swept.
    #[error("Screen session not found")]
    NotFound,
    /// Session is expired or revoked but not yet swept.
    #[error("Screen session ended")]
    Ended,
    /// Invalid internal configuration or unrepresentable server time.
    #[error("Invalid companion configuration or server clock")]
    Configuration,
    /// Operating system failed to supply cryptographic randomness.
    #[error("Secure randomness unavailable")]
    Entropy,
}
impl Error {
    /// HTTP-equivalent status for a future transport adapter.
    pub fn status(&self) -> u16 {
        match self {
            Self::Input => 400,
            Self::AuthenticationRequired | Self::AuthenticationRejected | Self::Pairing => 401,
            Self::Origin => 403,
            Self::NotFound => 404,
            Self::Ended => 410,
            Self::Attempts | Self::Capacity => 429,
            Self::Configuration | Self::Entropy => 500,
        }
    }
}
