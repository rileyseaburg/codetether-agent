use super::*;

#[derive(Clone, Copy)]
pub(super) enum ReadKind {
    Text,
    DataUrl,
    ArrayBuffer,
}

impl ReadKind {
    pub(super) fn result(self, input: &JsValue, data: &[u8]) -> JsValue {
        match self {
            Self::Text => JsValue::String(String::from_utf8_lossy(data).into()),
            Self::DataUrl => data_url_value(input, data),
            Self::ArrayBuffer => bytes::byte_array(data.iter().copied()),
        }
    }
}

fn data_url_value(input: &JsValue, data: &[u8]) -> JsValue {
    let mime = match blob::mime_type(input).as_str() {
        "" => "application/octet-stream".into(),
        value => value.to_string(),
    };
    JsValue::String(format!("data:{mime};base64,{}", base64_encode(data)))
}
