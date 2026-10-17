//! Serialized GitHub pull-request mutation payload.
use super::*;

#[derive(Debug, Serialize)]
pub(super) struct UpdatePullRequestRequest<'a> {
    pub(super) title: &'a str,
    pub(super) body: &'a str,
}
