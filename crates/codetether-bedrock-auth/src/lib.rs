//! Bedrock credentials, live bearer state, SigV4 requests, and bearer tokens.
//!
//! Load [AwsCredentials], select [BedrockAuth], and use [sigv4::SigningClient]
//! for signed HTTP dispatch. [token_gen] creates presigned bearer tokens.
//! Secret persistence and refresh policy belong to the calling application.
mod bearer;
mod credentials;
mod mode;
pub mod sigv4;
pub mod token_gen;
pub use credentials::AwsCredentials;
pub use mode::BedrockAuth;
