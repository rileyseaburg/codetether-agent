//! Prevent silently mixing OpenAI file edits with Gemini conversation edits.
use crate::{args::ImagegenArgs, backend::ImageBackend};
use anyhow::{Result, bail};
use std::path::PathBuf;

pub(crate) fn selectors(args: &ImagegenArgs, paths: &[PathBuf]) -> Result<()> {
    match args.provider {
        ImageBackend::GeminiWeb => {
            if !paths.is_empty() || args.num_last_images_to_include.is_some() {
                bail!(
                    "Gemini Web edits use `refine_from` from a prior image result; arbitrary image uploads are not supported"
                );
            }
        }
        ImageBackend::Openai => {
            if args.refine_from.is_some() {
                bail!("`refine_from` requires provider `gemini-web`");
            }
        }
    }
    Ok(())
}
