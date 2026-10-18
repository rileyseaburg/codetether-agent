//! Encode clipboard images as pasteable text and print or copy the result.
use anyhow::Result;

pub(crate) fn image(print: bool) -> Result<()> {
    let image = codetether_clipboard::capture_image()?;
    if print {
        println!("{}", image.data_url);
        return Ok(());
    }
    codetether_clipboard::copy_text(&image.data_url)?;
    eprintln!("Copied image paste text. Paste it into the remote CodeTether TUI.");
    Ok(())
}
