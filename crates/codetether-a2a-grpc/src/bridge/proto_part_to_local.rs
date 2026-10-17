//! proto part to local conversion.
use super::*;

pub(super) fn proto_part_to_local(part: &proto::Part) -> Option<local::Part> {
    match &part.part {
        Some(proto::part::Part::Text(text)) => Some(local::Part::Text { text: text.clone() }),
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
