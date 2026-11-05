//! Pinned Monarch OAuth endpoints and focused discovery operations.
mod fetch;
mod http;
mod models;
mod validate;
pub(crate) use fetch::discover;
pub(crate) use http::{client, json};
#[cfg(test)]
mod tests;

pub(crate) const ISSUER: &str = "https://api.monarch.com";
pub(crate) const RESOURCE: &str = "https://api.monarch.com/mcp";
pub(crate) const AUTHORIZE: &str = "https://api.monarch.com/oauth/authorize/";
pub(crate) const TOKEN: &str = "https://api.monarch.com/oauth/token/";
pub(crate) const REGISTER: &str = "https://api.monarch.com/oauth/register";
pub(crate) const REVOKE: &str = "https://api.monarch.com/oauth/revoke/";
