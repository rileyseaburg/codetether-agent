//! Structured input variants compatible with Codex collaboration calls.

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
/// Structured content accepted by child-agent submissions.
///
/// Variants carry text, remote/local images, unsupported audio, skill references,
/// or mentions. Audio is decoded here and rejected during preparation.
///
/// # Examples
/// ```
/// use codetether_collaboration_input::InputItem;
/// let item=InputItem::Text {text:"Hello".into()};
/// if let InputItem::Text {text}=item {assert_eq!(text,"Hello");}
/// ```
pub enum InputItem {
    /// Plain text.
    Text { text: String },
    /// A base64 image data URL or unchanged HTTP(S) reference.
    Image { image_url: String },
    /// An image file resolved against the supplied workspace.
    LocalImage { path: PathBuf },
    /// Audio URL; rejected by the current local provider interface.
    Audio { audio_url: String },
    /// Local audio path; rejected by the current local provider interface.
    LocalAudio { path: PathBuf },
    /// Skill name and path included in the prompt.
    Skill { name: String, path: PathBuf },
    /// Named reference rendered as a link.
    Mention { name: String, path: String },
}
