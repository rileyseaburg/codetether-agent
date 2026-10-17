//! Model rotation and preference regressions.
use super::*;
mod copilot_prefers_gpt_5_mini_when_available;
mod model;
mod openrouter_prefers_free_glm_models;
mod provider_alias_normalizes_zhipu_and_z_ai;
mod relay_rotation_cycles_models;
use model::model;
mod model_rotation_defaults_tests;
