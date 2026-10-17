//! Native Anthropic Messages request and response conversion for InvokeModel.
//!
//! [body] builds requests; [invoke_msgconvert] and [invoke_convert] adapt
//! messages and tool definitions; [response] decodes completion results.
pub mod body;
mod invoke_blocks;
pub mod invoke_convert;
pub mod invoke_msgconvert;
mod native_audit;
mod native_tools;
pub mod response;
#[cfg(test)]
mod tests;
