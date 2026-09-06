//! Exercise the native OCR runtime on an in-memory image during setup checks.

use windows::{Graphics::Imaging::{BitmapAlphaMode, BitmapPixelFormat, SoftwareBitmap}, Media::Ocr::OcrEngine};

pub(super) fn recognize(engine: &OcrEngine) -> anyhow::Result<()> {
    let bitmap = SoftwareBitmap::CreateWithAlpha(
        BitmapPixelFormat::Bgra8, 16, 16, BitmapAlphaMode::Ignore,
    )?;
    let result = engine.RecognizeAsync(&bitmap)?.join()?;
    let _ = result.Text()?;
    Ok(())
}