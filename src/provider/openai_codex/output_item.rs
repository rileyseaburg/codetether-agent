//! Conversion of completed Responses output items into retry checkpoints.

use crate::provider::{ContentPart, codex_reasoning};
use serde_json::Value;

pub(super) fn checkpoint(item: &Value) -> Option<ContentPart> {
    match item.get("type")?.as_str()? {
        "message" => message(item),
        "function_call" => tool(item),
        "reasoning" => codex_reasoning::checkpoint(item),
        _ => None,
    }
}

fn message(item: &Value) -> Option<ContentPart> {
    let text = item.get("content")?.as_array()?.iter().filter_map(|part| {
        (part.get("type")?.as_str()? == "output_text")
            .then(|| part.get("text")?.as_str())
            .flatten()
    });
    Some(ContentPart::Text {
        text: text.collect::<Vec<_>>().join(""),
    })
}

fn tool(item: &Value) -> Option<ContentPart> {
    Some(ContentPart::ToolCall {
        id: item
            .get("call_id")
            .or_else(|| item.get("id"))?
            .as_str()?
            .into(),
        name: item.get("name")?.as_str()?.into(),
        arguments: item.get("arguments")?.as_str()?.into(),
        thought_signature: None,
    })
}

#[cfg(test)]
#[path = "output_item/tests.rs"]
mod tests;
