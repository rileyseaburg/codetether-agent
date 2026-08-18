//! FunctionGemma-powered hybrid tool-call router.
//!
//! Sits between the primary LLM response and the tool-extraction step in the
//! session agentic loop. When the primary LLM returns text-only output that
//! *describes* tool calls without using structured `ContentPart::ToolCall`
//! entries, the router passes the text plus available tool definitions through a
//! local FunctionGemma model (via Candle) and emits properly formatted
//! `ContentPart::ToolCall` entries.
//!
//! **Feature-gated**: this module only compiles when both the `functiongemma`
//! and `candle` features are enabled, since it depends on the in-process Candle
//! runtime. Binary size is unaffected in default builds.
//!
//! # Architecture
//!
//! `config` and `config_env` hold the contract; `prompt*` builds the model
//! input; `parse*` decodes its output; `router*`, `reformat*`, `infer`, and
//! `rewrite` own the routing decision and response rewriting.

#[path = "tool_router/config.rs"]
mod config;
#[path = "tool_router/config_env.rs"]
mod config_env;

#[path = "tool_router/prioritize.rs"]
mod prioritize;
#[path = "tool_router/prompt.rs"]
mod prompt;
#[path = "tool_router/prompt_tools.rs"]
mod prompt_tools;

#[path = "tool_router/parse.rs"]
mod parse;
#[path = "tool_router/parse_blocks.rs"]
mod parse_blocks;
#[path = "tool_router/parsed_call.rs"]
mod parsed_call;

#[path = "tool_router/direct.rs"]
mod direct;
#[path = "tool_router/infer.rs"]
mod infer;
#[path = "tool_router/inspect.rs"]
mod inspect;
#[path = "tool_router/reformat.rs"]
mod reformat;
#[path = "tool_router/reformat_infer.rs"]
mod reformat_infer;
#[path = "tool_router/rewrite.rs"]
mod rewrite;
#[path = "tool_router/router.rs"]
mod router;
#[path = "tool_router/router_build.rs"]
mod router_build;

#[cfg(test)]
#[path = "tool_router/config_tests.rs"]
mod config_tests;
#[cfg(test)]
#[path = "tool_router/parse_malformed_tests.rs"]
mod parse_malformed_tests;
#[cfg(test)]
#[path = "tool_router/parse_tests.rs"]
mod parse_tests;
#[cfg(test)]
#[path = "tool_router/prompt_tests.rs"]
mod prompt_tests;

pub use config::ToolRouterConfig;
pub use router::ToolCallRouter;
