//! Model-authored swarm plan parsing, validation, staging and prompt construction.
//!
//! Build a decomposition prompt, parse the response with [parse_decomposition],
//! then [validate] dependency stages. [single] supplies the existing one-task fallback.
use crate::SubTask;
use anyhow::Result;
use std::collections::HashMap;
mod create;
pub mod decomposition_prompt;
mod dependencies;
mod json;
mod parse;
mod plan;
pub mod repair_prompt;
mod stages;
mod wire;
pub use parse::parse_decomposition;
pub use plan::{single, validate};

mod completion;
pub use completion::PlanCompletion;
mod generate;
pub use generate::generate_plan;
mod repair;
#[cfg(test)]
mod tests;
