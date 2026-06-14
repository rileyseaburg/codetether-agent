//! `codetether auth bedrock` — mint a short-term Bedrock API key from
//! AWS credentials, including SSO/IdP profiles (`aws sso login`).
//!
//! AWS publishes this mechanism as the "Bedrock API key": a SigV4
//! presigned `CallWithBearerToken` URL usable as a plain bearer token
//! (`AWS_BEARER_TOKEN_BEDROCK`). We mirror the official generator.

mod args;
mod aws_cli;
mod creds;
mod execute;
mod exported;
mod login;
mod mode;
mod output;
mod select;
mod sso;
mod sso_cache;
mod sso_cache_files;
mod sso_default;
mod sso_parse;
mod sso_profile_meta;
#[cfg(test)]
mod sso_tests;
mod sso_vault_extra;
mod token;
mod validate;
mod vault_expiry;
mod vault_extra;
mod vault_save;

pub use args::BedrockAuthArgs;
pub use execute::execute;
pub use login::LoginMode;
