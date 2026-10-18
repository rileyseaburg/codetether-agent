//! Application enrichment binding and native tool-call regression coverage.
mod host;
pub(super) use codetether_native_worker::tool_call::execute_tool_call;
#[cfg(test)]
mod denied_tests;
#[cfg(test)]
mod image_fixture;
#[cfg(test)]
mod image_tests;
