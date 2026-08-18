//! Multi-shape assistant content decoding for OpenAI-compatible responses.

use serde::Deserialize;

/// Assistant content, which servers emit as a string, a part, or a part list.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum OpenAIChatContent {
    Text(String),
    Parts(Vec<OpenAIChatContentPart>),
    Part(OpenAIChatContentPart),
}

/// One structured content part.
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIChatContentPart {
    #[serde(rename = "type")]
    kind: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    content: Option<String>,
}

impl OpenAIChatContent {
    /// Flatten any content shape into plain text.
    pub(crate) fn to_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Parts(parts) => parts
                .iter()
                .filter_map(OpenAIChatContentPart::text_fragment)
                .collect::<Vec<_>>()
                .join("\n"),
            Self::Part(part) => part.text_fragment().unwrap_or_default(),
        }
    }
}

impl OpenAIChatContentPart {
    /// Return this part's text when it carries a textual payload.
    fn text_fragment(&self) -> Option<String> {
        if let Some(kind) = self.kind.as_deref()
            && !kind.eq_ignore_ascii_case("text")
            && !kind.eq_ignore_ascii_case("output_text")
        {
            return None;
        }

        self.text
            .as_deref()
            .or(self.content.as_deref())
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(ToString::to_string)
    }
}
