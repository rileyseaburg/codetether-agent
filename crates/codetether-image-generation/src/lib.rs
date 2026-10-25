//! OpenAI and Gemini Web image generation and editing tool.
//!
//! [`ImageGenerationTool`] turns a rewritten prompt and optional image
//! references into an image that is saved and returned to the model. Set
//! `provider` to `gemini-web` for native Gemini generation and `refine_from`
//! to a previous result's state path for conversational improvements.

mod args;
mod artifact;
mod auth;
mod backend;
mod client;
mod credentials;
mod execute;
mod gemini;
mod recent;
mod reference_file;
mod references;
mod request_body;
mod response;
mod result;
mod schema;
mod tool_impl;
mod vault_choice;
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
mod reference_tests;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod result_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod vault_choice_tests;
#[cfg(test)]
mod vault_credentials_tests;
