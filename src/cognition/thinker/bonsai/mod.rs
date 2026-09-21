//! Compatibility bridge for existing thinker callers; native ownership lives in provider::bonsai.
mod native_new;
pub(super) use crate::provider::bonsai::native::Model;
pub(super) fn try_load(
    config: &super::ThinkerConfig,
) -> anyhow::Result<Option<super::CandleThinker>> {
    if crate::provider::bonsai::native::matches(
        config.candle_model_path.as_deref(),
        config.candle_arch.as_deref(),
    )? {
        native_new::load(config).map(Some)
    } else {
        Ok(None)
    }
}

#[cfg(all(test, feature = "candle-cuda"))]
mod full_model_tests;
