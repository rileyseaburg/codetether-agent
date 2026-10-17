//! Ordered Google model catalog.
use crate::*;
mod gemini_2;
mod gemini_25;
mod gemini_3;
mod gemini_31;
mod gemini_image;
pub(crate) fn models() -> Vec<ModelInfo> {
    vec![
        gemini_31::model_0(),
        gemini_31::model_1(),
        gemini_3::model_2(),
        gemini_3::model_3(),
        gemini_image::model_4(),
        gemini_25::model_5(),
        gemini_25::model_6(),
        gemini_2::model_7(),
    ]
}
