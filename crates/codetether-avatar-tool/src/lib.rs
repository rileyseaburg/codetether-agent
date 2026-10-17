//! Avatar video generation and model management through the voice service.
//!
//! [AvatarTool] exposes container lifecycle, generation, and model operations.
//! Action modules own requests; separate modules validate inputs and format results.
//!
//! ```rust
//! let _tool = codetether_avatar_tool::AvatarTool::new();
//! ```
mod client;
mod config;
use config::voice_api_url;
mod execute;
mod generate;
mod generate_args;
mod generate_audio;
mod generate_form;
mod generate_from_episode;
mod generate_from_episode_args;
mod generate_from_episode_form;
mod generate_from_episode_output;
mod generate_from_episode_result;
mod generate_result;
mod list_models;
mod list_models_result;
mod params;
mod required;
mod schema;
mod schema_fields_0;
mod schema_fields_1;
mod schema_fields_2;
mod start;
mod status;
mod status_result;
mod stop;
mod tool;
mod upload_model;
mod upload_model_args;
mod upload_model_form;
mod upload_model_result;
/// Generate avatar videos and manage models through the configured voice API.
///
/// # Examples
/// ```rust
/// let _tool = codetether_avatar_tool::AvatarTool::new();
/// ```
pub struct AvatarTool {
    client: reqwest::Client,
}
