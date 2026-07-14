//! OpenAI image generation and editing tool.
//!
//! [`ImageGenerationTool`] turns a rewritten prompt and optional image
//! references into a generated PNG that is saved and returned to the model.

mod args;
mod artifact;
mod auth;
mod client;
mod credentials;
mod execute;
mod reference_file;
mod references;
mod request_body;
mod response;
mod result;
mod schema;
mod tool_impl;
mod vault_credentials;

pub use tool_impl::ImageGenerationTool;

const IMAGE_MODEL: &str = "gpt-image-2";
const MAX_EDIT_IMAGES: usize = 5;
const DESCRIPTION: &str = include_str!("imagegen_description.md");

#[cfg(test)]
mod auth_tests;
#[cfg(test)]
mod input_tests;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod vault_credentials_tests;
