//! Provider/tool execution and prompt adaptation for individual stories.
//!
//! Application adapters retain live resources and call shared Ralph services.
//! Methods preserve their original visibility within the Ralph loop owner.
use super::*;
pub(super) mod host;
mod model_direct;
mod model_static;
mod prompts;
