//! local part to proto conversion.
use super::*;

pub(super) fn local_part_to_proto(part: &local::Part) -> proto::Part {
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
