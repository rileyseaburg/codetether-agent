//! Serialized GitHub pull-request mutation payload.
use super::*;

#[derive(Debug, Serialize)]
pub(super) struct CreatePullRequestRequest<'a> {
    pub(super) title: &'a str,
    pub(super) head: &'a str,
    pub(super) base: &'a str,
    pub(super) body: &'a str,
    pub(super) draft: bool,
}
