//! Authenticated Gemini Web image generation and conversational refinement.
//!
//! [`ImageClient`] owns a scoped Netscape cookie jar and a private renewal cache.
//! Its native HTTP requests return [`ImageGeneration`] containing checked JPEG
//! or PNG bytes. No cookie values or signed image URLs are included in errors.
//! See [`ImageClient::new`] and [`ImageClient::generate`] for usage.
mod authenticate;
mod cache;
mod cache_io;
mod client;
mod cookie_expiry;
mod cookies;
mod download;
mod format;
mod generate;
mod http;
mod jpeg;
mod migrate_cache;
mod netscape;
mod network_error;
mod node_cache;
mod page;
mod policy;
mod protocol_error;
mod refresh;
mod renewal_throttle;
mod rotation;
mod rotation_response;
mod tokens;
mod types;
pub use client::ImageClient;
pub use types::{GeneratedImage, ImageGeneration};
const ORIGIN: &str = "https://gemini.google.com";
const ROTATE: &str = "https://accounts.google.com/RotateCookies";
const LIMIT: usize = 32 * 1024 * 1024;
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36";

mod cache_permissions;
mod cookie_record;
mod cookie_scope;
mod rotation_fresh;

#[cfg(test)]
mod tests;
