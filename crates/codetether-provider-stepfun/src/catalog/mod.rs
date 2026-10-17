//! Ordered StepFun model catalog.
use crate::*;
mod model_0;
mod model_1;
mod model_2;
mod model_3;
mod model_4;
pub(crate) fn models() -> Vec<ModelInfo> {
    vec![
        model_0::info(),
        model_1::info(),
        model_2::info(),
        model_3::info(),
        model_4::info(),
    ]
}
