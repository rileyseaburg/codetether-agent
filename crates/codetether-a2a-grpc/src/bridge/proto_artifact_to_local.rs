//! proto artifact to local conversion.
use super::*;

pub(super) fn proto_artifact_to_local(art: &proto::Artifact) -> local::Artifact {
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
