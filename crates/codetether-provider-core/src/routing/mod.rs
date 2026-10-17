//! Provider/model routing over an existing provider map.
//!
//! [parse_model_string] preserves nested model namespaces. [resolve_model]
//! resolves aliases and clones provider handles without changing the map.
use crate::Provider;
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
mod parse;
mod resolve;
pub use parse::parse_model_string;
pub use resolve::resolve_model;
#[cfg(test)]
mod tests;
