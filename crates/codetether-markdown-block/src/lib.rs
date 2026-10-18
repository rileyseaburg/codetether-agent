//! Classify and style Markdown block prefixes for terminal messages.
//!
//! [classify] recognizes headings, rules, quotes, lists and task checkboxes.
//! [Block] carries fully styled content or a prefix and remaining inline text,
//! leaving inline emphasis processing to the caller.
mod detect;
mod header_style;
mod model;
pub use detect::classify;
pub use model::Block;
#[cfg(test)]
mod tests;
