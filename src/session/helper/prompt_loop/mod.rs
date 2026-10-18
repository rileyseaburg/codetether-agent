//! Shared agentic loop used by streaming and non-streaming prompts.

mod completion;
mod finish;
mod goal_recovery;
mod input;
mod lifecycle;
mod model;
use codetether_prompt_loop as progress;
mod response;
mod setup;
mod setup_support;
mod state;
mod tools;

pub(crate) use lifecycle::run;
pub(crate) use setup::initialize;
pub(crate) use state::Runner;
