//! Native and textual provider tool-call parsing without session ownership.
//!
//! [parse_tool_calls] separates valid calls from malformed argument payloads.
//! [normalize_textual_tool_calls] recovers markup or bare JSON only when native
//! calls are absent, filtering recovered names against the registered tools.
mod arguments;
mod bare_json;
mod call_parts;
mod markup;
mod markup_regex;
mod native;
mod normalize;
mod text;
pub use arguments::parse_arguments;
pub use bare_json::{extract_bare_json_tool_call, remaining_keys_as_args};
pub use markup::extract_markup_tool_calls;
pub use markup_regex::tool_call_markup_re;
pub use native::parse_tool_calls;
pub use normalize::normalize_textual_tool_calls;
#[cfg(test)]
mod tests;
