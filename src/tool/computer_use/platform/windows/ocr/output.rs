//! Serialize WinRT OCR results before returning to the async caller.

use super::geometry;
use serde_json::{Value, json};
use windows::Media::Ocr::OcrResult;

pub(super) fn lines(result: &OcrResult) -> anyhow::Result<Vec<Value>> {
    let mut lines = Vec::new();
    for line in result.Lines()? {
        let mut words = Vec::new();
        let mut bounds = None;
        for word in line.Words()? {
            let rect = word.BoundingRect()?;
            bounds = Some(geometry::union(bounds, rect));
            words.push(json!({"text": word.Text()?.to_string(),
                "bounding_box": geometry::bounds(rect)}));
        }
        lines.push(json!({"text": line.Text()?.to_string(), "words": words,
            "bounding_box": bounds.map(geometry::bounds)}));
    }
    Ok(lines)
}
