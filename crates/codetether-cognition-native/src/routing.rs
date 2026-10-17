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

#[path = "routing/config.rs"]
mod config;

#[path = "routing/prioritize.rs"]
mod prioritize;
#[path = "routing/prompt.rs"]
mod prompt;
#[path = "routing/prompt_tools.rs"]
mod prompt_tools;

#[path = "routing/parse.rs"]
mod parse;
#[path = "routing/parse_blocks.rs"]
mod parse_blocks;
#[path = "routing/parsed_call.rs"]
mod parsed_call;

#[path = "routing/direct.rs"]
mod direct;
#[path = "routing/identity_prompt.rs"]
mod identity_prompt;
#[path = "routing/infer.rs"]
mod infer;
#[path = "routing/inspect.rs"]
mod inspect;
#[path = "routing/reformat.rs"]
mod reformat;
#[path = "routing/reformat_infer.rs"]
mod reformat_infer;
#[path = "routing/rewrite.rs"]
mod rewrite;
#[path = "routing/router.rs"]
mod router;
#[path = "routing/router_build.rs"]
mod router_build;
#[path = "routing/router_config.rs"]
mod router_config;

#[cfg(test)]
#[path = "routing/config_tests.rs"]
mod config_tests;
#[cfg(test)]
#[path = "routing/parse_malformed_tests.rs"]
mod parse_malformed_tests;
#[cfg(test)]
#[path = "routing/parse_tests.rs"]
mod parse_tests;
#[cfg(test)]
#[path = "routing/prompt_tests.rs"]
mod prompt_tests;

pub use config::ToolRouterConfig;
pub use router::ToolCallRouter;
