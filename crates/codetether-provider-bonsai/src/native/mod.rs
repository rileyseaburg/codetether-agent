//! Direct Bonsai inference: packed GGUF weights and native CUDA kernels via Candle tensors.
mod model_state;
include!("modules_0.rs");
include!("modules_1.rs");
include!("modules_2.rs");
include!("modules_3.rs");
mod contract_inverse;
mod detect;
mod loaded;
use contract::validate;
pub use detect::matches;
use index::Index;
pub use loaded::{Loaded, open};
pub use model::Model;
#[cfg(all(test, feature = "candle-cuda"))]
mod cuda_rotation_tests;
#[cfg(all(test, feature = "candle-cuda"))]
mod cuda_tests;
