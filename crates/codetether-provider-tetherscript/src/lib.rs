//! TetherScript-backed provider orchestration with per-call plugin instances.
//!
//! [TetherScriptProvider] stores source and credentials; blocking workers load
//! fresh plugins because runtime values are not Send. [usai] supplies the
//! existing embedded USAI script and endpoint defaults.
mod authority;
mod call;
mod complete;
mod complete_parse;
mod convert;
mod model_list;
#[cfg(test)]
mod model_record;
mod provider_impl;
mod runner;
pub mod usai;

pub use runner::TetherScriptProvider;
