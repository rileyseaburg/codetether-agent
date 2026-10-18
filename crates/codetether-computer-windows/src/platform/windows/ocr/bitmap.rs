//! Decode at original size, rejecting oversized images before bitmap allocation.

use anyhow::Context;
use windows::{
    Graphics::Imaging::{
        BitmapAlphaMode, BitmapDecoder, BitmapPixelFormat, BitmapTransform, ColorManagementMode,
        ExifOrientationMode, SoftwareBitmap,
    },
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream},
};

pub(super) fn decode(bytes: &[u8], maximum: u32) -> anyhow::Result<SoftwareBitmap> {
    let stream = InMemoryRandomAccessStream::new()?;
    let writer = DataWriter::CreateDataWriter(&stream)?;
    writer.WriteBytes(bytes)?;
    let stored = writer.StoreAsync()?.join()?;
    anyhow::ensure!(
        stored as usize == bytes.len(),
        "Incomplete OCR image stream write"
    );
    writer.DetachStream()?;
    stream.Seek(0)?;
    let decoder = BitmapDecoder::CreateAsync(&stream)?.join().context(
        "Windows could not decode the OCR image; use a supported image such as PNG or JPEG",
    )?;
    let (width, height) = (decoder.PixelWidth()?, decoder.PixelHeight()?);
    anyhow::ensure!(
        width > 0 && height > 0,
        "OCR image dimensions must be positive"
    );
    anyhow::ensure!(
        width <= maximum && height <= maximum,
        "OCR image {width}x{height} exceeds OcrEngine.MaxImageDimension={maximum}; explicitly crop or resize the source and retry (no automatic scaling is performed)"
    );
    // Conversion changes pixel format only: no crop, scale, or EXIF rotation.
    Ok(decoder
        .GetSoftwareBitmapTransformedAsync(
            BitmapPixelFormat::Bgra8,
            BitmapAlphaMode::Ignore,
            &BitmapTransform::new()?,
            ExifOrientationMode::IgnoreExifOrientation,
            ColorManagementMode::DoNotColorManage,
        )?
        .join()?)
}
