//! Native packed Bonsai inference in Candle. No subprocess, server or full FP16 weights.
include!("modules_0.rs");
include!("modules_1.rs");
include!("modules_2.rs");
include!("modules_3.rs");
use contract::validate;
use index::Index;
pub(super) use model::Model;
pub(super) fn try_load(
    config: &super::ThinkerConfig,
) -> anyhow::Result<Option<super::CandleThinker>> {
    if detect::matches(config)? {
        native_new::load(config).map(Some)
    } else {
        Ok(None)
    }
}

#[cfg(all(test, feature = "candle-cuda"))]
mod cuda_rotation_tests;
#[cfg(all(test, feature = "candle-cuda"))]
mod cuda_tests;

mod contract_inverse;

#[cfg(all(test, feature = "candle-cuda"))]
mod full_model_tests;
