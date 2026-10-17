//! Read supported input, output, and inference modalities.
use super::*;
pub(super) fn read(m: &Value) -> (Vec<&str>, Vec<&str>, Vec<&str>) {
    let output_modalities: Vec<&str> = m
        .get("outputModalities")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let input_modalities: Vec<&str> = m
        .get("inputModalities")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let inference_types: Vec<&str> = m
        .get("inferenceTypesSupported")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();
    (output_modalities, input_modalities, inference_types)
}
