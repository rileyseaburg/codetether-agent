//! File attachments formatted as bounded plain-text snippets for a composer.
//!
//! build_file_share_snippet reads a file and returns text plus truncation and
//! binary flags. display_path_for_workspace supplies a workspace-relative label.
pub mod interactive;
mod path;
mod size;
mod snippet;
mod text;
pub use path::display_path_for_workspace;
pub use snippet::build_file_share_snippet;
/// Default upper bound on retained UTF-8 file content in a shared snippet.
pub const FILE_SHARE_MAX_BYTES: usize = 64 * 1024;
#[cfg(test)]
mod tests;
