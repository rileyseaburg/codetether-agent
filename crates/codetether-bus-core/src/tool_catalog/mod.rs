//! Shared advertised tool schemas and JSON function specifications.
use serde::{Deserialize, Serialize};
mod schema;
pub use schema::ToolSchema;
mod functions;
pub use functions::to_function_specs;
mod definitions;
pub use definitions::from_definitions;
