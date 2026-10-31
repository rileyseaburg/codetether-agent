//! Repair persisted native PPM captures before sending them to vision providers.
use super::*;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[test]
fn mislabeled_ppm_data_url_becomes_real_png() {
    let ppm = b"P6\n2 1\n255\n\xff\x00\x00\x00\xff\x00";
    let url = format!("data:image/png;base64,{}", STANDARD.encode(ppm));
    let repaired = fit_data_url(&url).unwrap();
    let bytes = STANDARD
        .decode(repaired.strip_prefix("data:image/png;base64,").unwrap())
        .unwrap();
    assert_eq!(image::guess_format(&bytes).unwrap(), ImageFormat::Png);
    let decoded = image::load_from_memory(&bytes).unwrap().to_rgb8();
    assert_eq!(decoded.dimensions(), (2, 1));
    assert_eq!(decoded.into_raw(), [255, 0, 0, 0, 255, 0]);
    assert!(fit_data_url(&repaired).is_none());
}

#[test]
fn oversized_ppm_is_also_downscaled() {
    let mut ppm = b"P6\n1 2500\n255\n".to_vec();
    ppm.extend(vec![128; 2500 * 3]);
    let fitted = fit(&ppm).unwrap();
    assert_eq!(fitted.mime, "image/jpeg");
    let decoded = image::load_from_memory(&fitted.bytes).unwrap();
    assert_eq!(decoded.width(), 1);
    assert_eq!(decoded.height(), MAX_EDGE);
}
