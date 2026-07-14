//! Shared agentic loop used by streaming and non-streaming prompts.

mod completion;
mod finish;
mod input;
mod lifecycle;
mod model;
mod progress;
mod response;
mod selector;
mod setup;
mod setup_model;
mod setup_support;
mod state;
mod tools;

pub(crate) use lifecycle::run;
pub(crate) use setup::initialize;
pub(crate) use state::Runner;
