//! Render structured items into the local text-and-images prompt shape.

use super::{InputItem, Prepared, image};
use anyhow::{Result, bail};

pub(super) async fn items(items: Vec<InputItem>) -> Result<Prepared> {
    let mut text = Vec::new();
    let mut images = Vec::new();
    for item in items {
        match item {
            InputItem::Text { text: value } => text.push(value),
            InputItem::Image { image_url } => {
                images.push(image::data_url(&image_url)?);
                text.push("[Image attached]".into());
            }
            InputItem::LocalImage { path } => {
                images.push(image::local(&path).await?);
                text.push(format!("[Image attached: {}]", path.display()));
            }
            InputItem::Skill { name, path } => {
                text.push(format!("Use skill `{name}` from `{}`.", path.display()));
            }
            InputItem::Mention { name, path } => text.push(format!("[{name}]({path})")),
            InputItem::Audio { audio_url } => unsupported(&audio_url)?,
            InputItem::LocalAudio { path } => unsupported(&path.display().to_string())?,
        }
    }
    Ok(Prepared {
        message: text.join("\n"),
        images,
    })
}

fn unsupported(source: &str) -> Result<()> {
    bail!("Audio input `{source}` is not supported by the local provider interface")
}
