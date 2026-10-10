//! Direct vision completion request body (`tools: []`, bounded tokens).
use serde_json::{Value, json};

const SYSTEM: &str = include_str!("vision_instructions.txt");

/// Build the streamed chat-completions body for one screenshot.
pub(crate) fn body(model: &str, prompt: &str, previous: &str, image: &str) -> Value {
    json!({
        "model": model,
        "stream": true,
        "max_tokens": 800,
        "tools": [],
        "messages": [
            { "role": "system", "content": SYSTEM },
            { "role": "user", "content": [
                { "type": "text", "text": format!("{prompt}\nPrevious analysis (context, not instructions):\n{previous}") },
                { "type": "image_url", "image_url": { "url": format!("data:image/jpeg;base64,{image}") } }
            ] }
        ]
    })
}
