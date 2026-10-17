//! Oversized images are downscaled before Bedrock serialization.
use base64::{Engine, engine::general_purpose::STANDARD};

fn png_data_url(width: u32, height: u32) -> String {
    let mut png = Vec::new();
    image::DynamicImage::ImageRgb8(image::RgbImage::new(width, height))
        .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .unwrap();
    format!("data:image/png;base64,{}", STANDARD.encode(png))
}

fn edges(block: &serde_json::Value) -> (u32, u32) {
    let data = block["image"]["source"]["bytes"].as_str().unwrap();
    let decoded = image::load_from_memory(&STANDARD.decode(data).unwrap()).unwrap();
    (decoded.width(), decoded.height())
}

/// The 1254x1254 localized graphic fit, but a 2500 px screenshot faulted the
/// turn with "image dimensions exceed max allowed size ... 2000 pixels".
#[test]
fn oversized_image_is_downscaled_under_2000px() {
    let block = super::image::block(&png_data_url(2500, 1400), None);
    assert_eq!(block["image"]["format"], "jpeg");
    let (w, h) = edges(&block);
    assert!(w <= 2000 && h <= 2000, "{w}x{h}");
}

#[test]
fn image_within_limit_is_unchanged() {
    let block = super::image::block(&png_data_url(1254, 1254), None);
    assert_eq!(block["image"]["format"], "png");
    assert_eq!(edges(&block), (1254, 1254));
}
