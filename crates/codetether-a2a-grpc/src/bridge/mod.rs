//! Conversion between protobuf wire types and shared A2A contracts.
//!
//! Message, task, artifact, card and JSON conversions preserve the wire
//! representations used by the gRPC transport. Public functions convert in
//! either direction; internal helpers handle parts and enum variants.
use crate::proto;
use codetether_a2a_core as local;
mod proto_message_to_local;
pub use proto_message_to_local::proto_message_to_local;
mod proto_part_to_local;
use proto_part_to_local::proto_part_to_local;
mod proto_task_state_to_local;
use proto_task_state_to_local::proto_task_state_to_local;
mod proto_task_to_local;
pub use proto_task_to_local::proto_task_to_local;
mod proto_artifact_to_local;
use proto_artifact_to_local::proto_artifact_to_local;
mod local_task_to_proto;
pub use local_task_to_proto::local_task_to_proto;
mod local_task_status_to_proto;
pub use local_task_status_to_proto::local_task_status_to_proto;
mod local_task_state_to_proto;
use local_task_state_to_proto::local_task_state_to_proto;
mod local_message_to_proto;
pub use local_message_to_proto::local_message_to_proto;
mod local_part_to_proto;
use local_part_to_proto::local_part_to_proto;
mod local_artifact_to_proto;
use local_artifact_to_proto::local_artifact_to_proto;
mod card_capabilities;
mod card_interfaces;
mod card_signatures;
mod card_skills;
mod local_card_to_proto;
pub use local_card_to_proto::local_card_to_proto;
mod prost_struct_to_json;
pub use prost_struct_to_json::prost_struct_to_json;
mod prost_value_to_json;
use prost_value_to_json::prost_value_to_json;
mod json_to_prost_struct;
pub use json_to_prost_struct::json_to_prost_struct;
mod json_to_prost_value;
use json_to_prost_value::json_to_prost_value;
