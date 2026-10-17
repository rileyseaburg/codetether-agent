//! GitHub PR requests using caller-supplied authentication and content.
//! [authenticated_client] constructs headers, then [create_or_update] selects
//! the existing branch PR or creates one from a [PullRequestSpec].
use crate::context::RepoContext;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
mod authenticated_client;
mod create_payload;
mod create_pr;
mod find_existing;
mod response;
mod spec;
mod update_payload;
mod update_pr;
mod upsert;
pub use authenticated_client::authenticated_client;
use create_payload::CreatePullRequestRequest;
use create_pr::create_pr;
use find_existing::find_existing;
pub use response::PullRequestResponse;
pub use spec::PullRequestSpec;
use update_payload::UpdatePullRequestRequest;
use update_pr::update_pr;
pub use upsert::create_or_update;
