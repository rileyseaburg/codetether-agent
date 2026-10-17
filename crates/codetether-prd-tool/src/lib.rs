//! Generate, save and validate structured PRDs for Ralph execution.
//!
//! [PrdTool] exposes analysis, generation, validation and persistence through
//! the shared tool contract. Data and persistence contracts live in Ralph core.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_ralph_core::{Prd, QualityChecks, UserStory, VerificationStep};
use codetether_tool_core::{Tool, ToolResult};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashSet;
use std::path::PathBuf;
mod analyze;
mod construct;
mod convert;
mod cycle;
mod cycle_dfs;
mod execute;
mod generate;
mod params;
mod quality;
mod quality_input;
mod save;
mod schema;
mod schema_stories;
mod story_input;
mod tool;
mod validate;
mod validate_dependencies;
mod validate_story;
mod validation;
mod validation_response;
mod validation_result;
mod verification_example;
use cycle::detect_cycle;
use params::Params;
use quality_input::QualityChecksInput;
use story_input::StoryInput;
use validation::validate_prd;
use validation_result::ValidationResult;
/// Stateless PRD generation and validation tool.
///
/// # Examples
/// ```rust
/// let _tool = codetether_prd_tool::PrdTool::new();
/// ```
pub struct PrdTool;
