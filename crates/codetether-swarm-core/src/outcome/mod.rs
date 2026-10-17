//! Shared execution outcomes and produced artifacts.
//!
//! [SwarmResult] aggregates per-task results and metrics; [SwarmArtifact] carries
//! produced content or a path with source and MIME metadata.
use crate::{SubTaskResult, SwarmStats};
use serde::{Deserialize, Serialize};
mod result;
pub use result::SwarmResult;
mod artifact;
pub use artifact::SwarmArtifact;
