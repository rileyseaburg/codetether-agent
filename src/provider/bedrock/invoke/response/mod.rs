//! Native Anthropic Messages response types and parsing.

mod content;
mod parse;
mod types;

pub(in crate::provider::bedrock) use parse::parse_anthropic_messages_response;
