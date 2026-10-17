//! local message to proto conversion.
use super::*;

pub fn local_message_to_proto(msg: &local::Message) -> proto::Message {
    proto::Message {
        message_id: msg.message_id.clone(),
        context_id: msg.context_id.clone().unwrap_or_default(),
        task_id: msg.task_id.clone().unwrap_or_default(),
        role: match msg.role {
            local::MessageRole::User => proto::Role::User.into(),
            local::MessageRole::Agent => proto::Role::Agent.into(),
        },
        content: msg.parts.iter().map(local_part_to_proto).collect(),
        metadata: None,
        extensions: msg.extensions.clone(),
    }
}
