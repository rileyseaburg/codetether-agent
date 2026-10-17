//! Credential broker wire types, HTTP transport and Git query formatting.
//!
//! The host owns CLI streams and helper-process invocation. Credential debug
//! output retains password redaction; broker status handling is explicit.
mod gh_query;
mod material;
mod parse;
mod query;
mod request;
mod response;
pub use gh_query::render_gh_credential_query;
pub use material::GitCredentialMaterial;
pub use parse::parse_query;
pub use query::GitCredentialQuery;
pub use request::request_git_credentials;
