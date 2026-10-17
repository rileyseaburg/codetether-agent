//! Wire types for Moltbook registration, profiles, posts, and API envelopes.
//!
//! These serde types preserve the remote payload structure and optional fields.
use serde::{Deserialize, Serialize};
mod register_response;
pub use register_response::RegisterResponse;
mod registered_agent;
pub use registered_agent::RegisteredAgent;
mod agent_profile;
pub use agent_profile::AgentProfile;
mod claim_status;
pub use claim_status::ClaimStatus;
mod post;
pub use post::Post;
mod post_author;
pub use post_author::PostAuthor;
mod feed_response;
pub use feed_response::FeedResponse;
mod api_response;
pub use api_response::ApiResponse;
