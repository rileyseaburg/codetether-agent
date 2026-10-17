//! local artifact to proto conversion.
use super::*;

pub(super) fn local_artifact_to_proto(art: &local::Artifact) -> proto::Artifact {
    proto::Artifact {
        artifact_id: art.artifact_id.clone(),
        name: art.name.clone().unwrap_or_default(),
        description: art.description.clone().unwrap_or_default(),
        parts: art.parts.iter().map(local_part_to_proto).collect(),
        metadata: None,
        extensions: art.extensions.clone(),
    }
}
