//! Explicit mention parsing and focused-agent prompt decoration.
//!
//! [parse_mention] recognizes a leading recipient and nonempty message.
//! [route_prompt] uses that recipient before a focused agent, passing commands
//! and empty input through unchanged when no explicit mention is present.
mod parse;
mod route;
pub use parse::parse_mention;
pub use route::route_prompt;
#[cfg(test)]
mod tests;
