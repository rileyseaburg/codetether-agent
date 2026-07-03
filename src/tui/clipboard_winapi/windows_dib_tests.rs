//! Tests for DIB → BMP file-header reconstruction.

#[cfg(test)]
mod tests {
    use super::super::windows_dib::dib_to_bmp;

    /// A 24-bit BMP (no palette, no bitfield masks) round-trips: stripping the
    /// 14-byte file header yields a DIB, and `dib_to_bmp` rebuilds a decodable
    /// BMP whose pixels match the original.
    #[test]
    fn dib_to_bmp_rebuilds_decodable_bmp() {
        let mut original = Vec::new();
        let img = image::RgbImage::from_fn(4, 3, |x, y| {
            image::Rgb([(x * 40) as u8, (y * 50) as u8, 60])
        });
        image::DynamicImage::ImageRgb8(img.clone())
            .write_to(
                &mut std::io::Cursor::new(&mut original),
                image::ImageFormat::Bmp,
            )
            .unwrap();

        // Clipboard CF_DIB == BMP file minus its 14-byte BITMAPFILEHEADER.
        let dib = &original[14..];
        let rebuilt = dib_to_bmp(dib).expect("dib_to_bmp should succeed");

        let decoded =
            image::load_from_memory_with_format(&rebuilt, image::ImageFormat::Bmp).unwrap();
        assert_eq!(decoded.to_rgb8(), img);
    }

    #[test]
    fn dib_to_bmp_rejects_truncated_header() {
        assert!(dib_to_bmp(&[0u8; 4]).is_none());
    }
}
