//! Transport-only image preservation for legacy Chat-compatible adapters.
//! `convert` decorates single-message serializers; `convert_many` also supports
//! adapters emitting multiple tool replies. Stored conversation roles never change.
use super::{ContentPart, Message, Role};
use serde_json::Value;

mod parts;
mod replies;
#[cfg(test)]
mod tests;
mod tools;

pub(super) fn convert(messages: &[Message], serialize: impl Fn(&Message) -> Value) -> Vec<Value> {
    convert_many(messages, |message| vec![serialize(message)])
}

pub(super) fn convert_many(
    messages: &[Message],
    serialize: impl Fn(&Message) -> Vec<Value>,
) -> Vec<Value> {
    let mut output = Vec::new();
    let mut pending = Vec::new();
    for message in messages {
        if message.role != Role::Tool {
            output.append(&mut pending);
        }
        let has_images = message
            .content
            .iter()
            .any(|part| matches!(part, ContentPart::Image { .. }));
        let mut converted = if message.role == Role::Tool {
            replies::convert(message, &serialize)
        } else {
            serialize(message)
        };
        match (has_images, message.role, converted.first_mut()) {
            (true, Role::User, Some(value)) => {
                value["content"] = Value::Array(parts::user(&message.content));
            }
            (true, Role::Tool, _) => pending.extend(tools::companions(message)),
            _ => {}
        }
        output.extend(converted);
    }
    output.extend(pending);
    output
}
