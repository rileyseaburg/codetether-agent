use super::{Continuation, ImageResponse, path::at};
use serde_json::Value;
use std::collections::HashSet;

pub(super) fn snapshot(inner: &Value) -> Option<ImageResponse> {
    let candidates = at(inner, &[4])?.as_array()?;
    let mut response = ImageResponse {
        urls: Vec::new(),
        continuation: None,
    };
    let mut seen = HashSet::new();
    let mut image_candidates = 0;
    for candidate in candidates {
        let Some(images) = at(candidate, &[12, 7, 0]).and_then(Value::as_array) else {
            continue;
        };
        if images.is_empty() {
            continue;
        }
        image_candidates += 1;
        response.continuation = continuation(inner, candidate);
        for image in images {
            if let Some(url) = at(image, &[0, 3, 3])
                .and_then(Value::as_str)
                .filter(|url| seen.insert(*url))
            {
                response.urls.push(url.to_owned());
            }
        }
    }
    if image_candidates != 1 {
        response.continuation = None;
    }
    Some(response)
}

fn continuation(inner: &Value, candidate: &Value) -> Option<Continuation> {
    let state = Continuation {
        conversation_id: at(inner, &[1, 0])?.as_str()?.to_owned(),
        response_id: at(inner, &[1, 1])?.as_str()?.to_owned(),
        choice_id: at(candidate, &[0])?.as_str()?.to_owned(),
    };
    state.validate().ok()?;
    Some(state)
}
