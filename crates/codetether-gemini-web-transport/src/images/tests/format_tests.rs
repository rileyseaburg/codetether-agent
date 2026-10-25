use crate::images::format;

#[test]
fn jpeg_signatures_dimensions_and_limits_are_checked() {
    let bytes = vec![
        255, 216, 255, 192, 0, 11, 8, 0, 2, 0, 3, 1, 1, 17, 0, 255, 217,
    ];
    let image = format::image(bytes.clone(), "image/jpeg").unwrap();
    assert_eq!((image.width, image.height), (3, 2));
    assert_eq!(image.extension, "jpg");
    assert!(format::image(bytes.clone(), "image/png").is_err());
    assert!(format::image(bytes[..bytes.len() - 2].to_vec(), "image/jpeg").is_err());
    assert!(format::image(Vec::new(), "image/jpeg").is_err());
}

#[test]
fn png_ihdr_and_iend_are_required() {
    // Header-only structural fixture; pixel decoding is not the transport's job.
    let mut bytes = vec![0; 45];
    bytes[..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");
    bytes[8..12].copy_from_slice(&13u32.to_be_bytes());
    bytes[12..16].copy_from_slice(b"IHDR");
    bytes[16..20].copy_from_slice(&10u32.to_be_bytes());
    bytes[20..24].copy_from_slice(&20u32.to_be_bytes());
    bytes[33..].copy_from_slice(b"\x00\x00\x00\x00IEND\xaeB`\x82");
    let image = format::image(bytes.clone(), "image/png").unwrap();
    assert_eq!((image.width, image.height), (10, 20));
    bytes[16..20].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(format::image(bytes, "image/png").is_err());
}
