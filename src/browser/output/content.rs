use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TextContent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HtmlContent {
    pub html: String,
}
