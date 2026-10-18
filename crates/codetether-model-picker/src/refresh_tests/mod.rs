//! Provider discovery and Codex variant coverage for model refresh.
use crate::ModelPickerState;
use std::sync::Arc;
mod support;
use support::{Registry as ProviderRegistry, StaticProvider, model};
mod codex_variants;
mod provider_models;
