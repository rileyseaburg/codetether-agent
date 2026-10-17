//! Native filename and text search.
//!
//! [search_files] matches filename globs and returns sorted paths. [grep]
//! searches file contents and renders path/line/text records. Both accept
//! borrowed JSON argument maps, allowing tool adapters to share their behavior.
mod filenames;
mod grep;
mod lines;
mod matcher;
mod paths;
pub use filenames::search_files;
pub use grep::grep;
#[cfg(test)]
mod tests;
