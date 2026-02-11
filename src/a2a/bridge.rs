//! Bridge between hand-written serde types (`a2a::types`) and
//! tonic/prost-generated types (`a2a::proto`).
//!
//! This module provides `From`/conversion functions in both directions so
//! the gRPC layer can speak proto types on the wire while the rest of the
//! codebase works with the ergonomic serde types.

#![allow(dead_code)]

use crate::a2a::proto;
use crate::a2a::types as local;

// ═══════════════════════════════════════════════════════════════════════
// Proto → Local
// ═══════════════════════════════════════════════════════════════════════

/// Convert a proto `Message` to a local `Message`.
pub fn proto_message_to_local(msg: &proto::Message) -> local::Message {
    local::Message {
        message_id: msg.message_id.clone(),
        role: match msg.role() {
            proto::Role::User => local::MessageRole::User,
            _ => local::MessageRole::Agent,
        },
        parts: msg.content.iter().filter_map(proto_part_to_local).collect(),
        context_id: if msg.context_id.is_empty() {
            None
        } else {
            Some(msg.context_id.clone())
        },
        task_id: if msg.task_id.is_empty() {
            None
        } else {
            Some(msg.task_id.clone())
        },
        metadata: Default::default(),
        extensions: msg.extensions.clone(),
    }
}

fn proto_part_to_local(part: &proto::Part) -> Option<local::Part> {
    match &part.part {
        Some(proto::part::Part::Text(text)) => Some(local::Part::Text {
            text: text.clone(),
        }),
        Some(proto::part::Part::File(file)) => {
            let (bytes, uri) = match &file.file {
                Some(proto::file_part::File::FileWithUri(u)) => (None, Some(u.clone())),
                Some(proto::file_part::File::FileWithBytes(b)) => {
                    use base64::Engine;
                    (
                        Some(base64::engine::general_purpose::STANDARD.encode(b)),
                        None,
                    )
                }
                None => (None, None),
            };
            Some(local::Part::File {
                file: local::FileContent {
                    bytes,
                    uri,
                    mime_type: if file.mime_type.is_empty() {
                        None
                    } else {
                        Some(file.mime_type.clone())
                    },
                    name: if file.name.is_empty() {
                        None
                    } else {
                        Some(file.name.clone())
                    },
                },
            })
        }
        Some(proto::part::Part::Data(data_part)) => {
            let val = data_part
                .data
                .as_ref()
                .map(prost_struct_to_json)
                .unwrap_or(serde_json::Value::Null);
            Some(local::Part::Data { data: val })
        }
        None => None,
    }
}

fn proto_task_state_to_local(state: proto::TaskState) -> local::TaskState {
    match state {
        proto::TaskState::Submitted => local::TaskState::Submitted,
        proto::TaskState::Working => local::TaskState::Working,
        proto::TaskState::Completed => local::TaskState::Completed,
        proto::TaskState::Failed => local::TaskState::Failed,
        proto::TaskState::Cancelled => local::TaskState::Cancelled,
        proto::TaskState::InputRequired => local::TaskState::InputRequired,
        proto::TaskState::Rejected => local::TaskState::Rejected,
        proto::TaskState::AuthRequired => local::TaskState::AuthRequired,
        proto::TaskState::Unspecified => local::TaskState::Submitted,
    }
}

/// Convert a proto `Task` to a local `Task`.
pub fn proto_task_to_local(task: &proto::Task) -> local::Task {
    let status = task
        .status
        .as_ref()
        .map(|s| local::TaskStatus {
            state: proto_task_state_to_local(s.state()),
            message: s.update.as_ref().map(proto_message_to_local),
            timestamp: s.timestamp.as_ref().map(|t| {
                chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            }),
        })
        .unwrap_or(local::TaskStatus {
            state: local::TaskState::Submitted,
            message: None,
            timestamp: None,
        });

    local::Task {
        id: task.id.clone(),
        context_id: if task.context_id.is_empty() {
            None
        } else {
            Some(task.context_id.clone())
        },
        status,
        artifacts: task.artifacts.iter().map(proto_artifact_to_local).collect(),
        history: task.history.iter().map(proto_message_to_local).collect(),
        metadata: Default::default(),
    }
}

fn proto_artifact_to_local(art: &proto::Artifact) -> local::Artifact {
    local::Artifact {
        artifact_id: art.artifact_id.clone(),
        parts: art.parts.iter().filter_map(proto_part_to_local).collect(),
        name: if art.name.is_empty() {
            None
        } else {
            Some(art.name.clone())
        },
        description: if art.description.is_empty() {
            None
        } else {
            Some(art.description.clone())
        },
        metadata: Default::default(),
        extensions: art.extensions.clone(),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Local → Proto
// ═══════════════════════════════════════════════════════════════════════

/// Convert a local `Task` to a proto `Task`.
pub fn local_task_to_proto(task: &local::Task) -> proto::Task {
    proto::Task {
        id: task.id.clone(),
        context_id: task.context_id.clone().unwrap_or_default(),
        status: Some(local_task_status_to_proto(&task.status)),
        artifacts: task.artifacts.iter().map(local_artifact_to_proto).collect(),
        history: task.history.iter().map(local_message_to_proto).collect(),
        metadata: None,
    }
}

pub fn local_task_status_to_proto(status: &local::TaskStatus) -> proto::TaskStatus {
    proto::TaskStatus {
        state: local_task_state_to_proto(status.state).into(),
        update: status.message.as_ref().map(local_message_to_proto),
        timestamp: status.timestamp.as_ref().and_then(|ts| {
            chrono::DateTime::parse_from_rfc3339(ts)
                .ok()
                .map(|dt| prost_types::Timestamp {
                    seconds: dt.timestamp(),
                    nanos: dt.timestamp_subsec_nanos() as i32,
                })
        }),
    }
}

fn local_task_state_to_proto(state: local::TaskState) -> proto::TaskState {
    match state {
        local::TaskState::Submitted => proto::TaskState::Submitted,
        local::TaskState::Working => proto::TaskState::Working,
        local::TaskState::Completed => proto::TaskState::Completed,
        local::TaskState::Failed => proto::TaskState::Failed,
        local::TaskState::Cancelled => proto::TaskState::Cancelled,
        local::TaskState::InputRequired => proto::TaskState::InputRequired,
        local::TaskState::Rejected => proto::TaskState::Rejected,
        local::TaskState::AuthRequired => proto::TaskState::AuthRequired,
    }
}

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

fn local_part_to_proto(part: &local::Part) -> proto::Part {
    match part {
        local::Part::Text { text } => proto::Part {
            part: Some(proto::part::Part::Text(text.clone())),
            metadata: None,
        },
        local::Part::File { file } => proto::Part {
            part: Some(proto::part::Part::File(proto::FilePart {
                file: file
                    .uri
                    .as_ref()
                    .map(|u| proto::file_part::File::FileWithUri(u.clone()))
                    .or_else(|| {
                        file.bytes.as_ref().and_then(|b| {
                            use base64::Engine;
                            base64::engine::general_purpose::STANDARD
                                .decode(b)
                                .ok()
                                .map(proto::file_part::File::FileWithBytes)
                        })
                    }),
                mime_type: file.mime_type.clone().unwrap_or_default(),
                name: file.name.clone().unwrap_or_default(),
            })),
            metadata: None,
        },
        local::Part::Data { data } => proto::Part {
            part: Some(proto::part::Part::Data(proto::DataPart {
                data: Some(json_to_prost_struct(data)),
            })),
            metadata: None,
        },
    }
}

fn local_artifact_to_proto(art: &local::Artifact) -> proto::Artifact {
    proto::Artifact {
        artifact_id: art.artifact_id.clone(),
        name: art.name.clone().unwrap_or_default(),
        description: art.description.clone().unwrap_or_default(),
        parts: art.parts.iter().map(local_part_to_proto).collect(),
        metadata: None,
        extensions: art.extensions.clone(),
    }
}

/// Convert a local `AgentCard` to a proto `AgentCard`.
pub fn local_card_to_proto(card: &local::AgentCard) -> proto::AgentCard {
    proto::AgentCard {
        protocol_version: card.protocol_version.clone(),
        name: card.name.clone(),
        description: card.description.clone(),
        url: card.url.clone(),
        preferred_transport: card.preferred_transport.clone().unwrap_or_default(),
        additional_interfaces: card
            .additional_interfaces
            .iter()
            .map(|i| proto::AgentInterface {
                url: i.url.clone(),
                transport: i.transport.clone(),
            })
            .collect(),
        provider: card.provider.as_ref().map(|p| proto::AgentProvider {
            url: p.url.clone(),
            organization: p.organization.clone(),
        }),
        version: card.version.clone(),
        documentation_url: card.documentation_url.clone().unwrap_or_default(),
        capabilities: Some(proto::AgentCapabilities {
            streaming: card.capabilities.streaming,
            push_notifications: card.capabilities.push_notifications,
            extensions: card
                .capabilities
                .extensions
                .iter()
                .map(|e| proto::AgentExtension {
                    uri: e.uri.clone(),
                    description: e.description.clone().unwrap_or_default(),
                    required: e.required,
                    params: None,
                })
                .collect(),
        }),
        security_schemes: Default::default(), // complex mapping deferred
        security: vec![],
        default_input_modes: card.default_input_modes.clone(),
        default_output_modes: card.default_output_modes.clone(),
        skills: card
            .skills
            .iter()
            .map(|s| proto::AgentSkill {
                id: s.id.clone(),
                name: s.name.clone(),
                description: s.description.clone(),
                tags: s.tags.clone(),
                examples: s.examples.clone(),
                input_modes: s.input_modes.clone(),
                output_modes: s.output_modes.clone(),
                security: vec![],
            })
            .collect(),
        supports_authenticated_extended_card: card.supports_authenticated_extended_card,
        signatures: card
            .signatures
            .iter()
            .map(|s| proto::AgentCardSignature {
                protected: s.algorithm.clone().unwrap_or_default(),
                signature: s.signature.clone(),
                header: None,
            })
            .collect(),
        icon_url: card.icon_url.clone().unwrap_or_default(),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// prost_types::Struct ↔ serde_json::Value helpers
// ═══════════════════════════════════════════════════════════════════════

/// Convert a `prost_types::Struct` to a `serde_json::Value`.
pub fn prost_struct_to_json(s: &prost_types::Struct) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> = s
        .fields
        .iter()
        .map(|(k, v)| (k.clone(), prost_value_to_json(v)))
        .collect();
    serde_json::Value::Object(map)
}

fn prost_value_to_json(v: &prost_types::Value) -> serde_json::Value {
    match &v.kind {
        Some(prost_types::value::Kind::NullValue(_)) => serde_json::Value::Null,
        Some(prost_types::value::Kind::NumberValue(n)) => {
            serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap_or_else(|| 0.into()))
        }
        Some(prost_types::value::Kind::StringValue(s)) => serde_json::Value::String(s.clone()),
        Some(prost_types::value::Kind::BoolValue(b)) => serde_json::Value::Bool(*b),
        Some(prost_types::value::Kind::StructValue(s)) => prost_struct_to_json(s),
        Some(prost_types::value::Kind::ListValue(l)) => {
            serde_json::Value::Array(l.values.iter().map(prost_value_to_json).collect())
        }
        None => serde_json::Value::Null,
    }
}

/// Convert a `serde_json::Value` to a `prost_types::Struct`.
pub fn json_to_prost_struct(v: &serde_json::Value) -> prost_types::Struct {
    match v {
        serde_json::Value::Object(map) => prost_types::Struct {
            fields: map
                .iter()
                .map(|(k, v)| (k.clone(), json_to_prost_value(v)))
                .collect(),
        },
        _ => prost_types::Struct::default(),
    }
}

fn json_to_prost_value(v: &serde_json::Value) -> prost_types::Value {
    let kind = match v {
        serde_json::Value::Null => prost_types::value::Kind::NullValue(0),
        serde_json::Value::Bool(b) => prost_types::value::Kind::BoolValue(*b),
        serde_json::Value::Number(n) => {
            prost_types::value::Kind::NumberValue(n.as_f64().unwrap_or(0.0))
        }
        serde_json::Value::String(s) => prost_types::value::Kind::StringValue(s.clone()),
        serde_json::Value::Array(arr) => {
            prost_types::value::Kind::ListValue(prost_types::ListValue {
                values: arr.iter().map(json_to_prost_value).collect(),
            })
        }
        serde_json::Value::Object(map) => {
            prost_types::value::Kind::StructValue(prost_types::Struct {
                fields: map
                    .iter()
                    .map(|(k, v)| (k.clone(), json_to_prost_value(v)))
                    .collect(),
            })
        }
    };
    prost_types::Value { kind: Some(kind) }
}
