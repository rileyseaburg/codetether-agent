//! Linux observation regressions; fixture utilities never need a real desktop.

use super::{
    backend::{self, Backend},
    command, image_file,
    session::Session,
    snapshot, status,
};
use crate::tool::computer_use::input::ComputerUseInput;
use std::{
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

mod backends;
mod capture;
mod commands;
mod environment;
mod failures;
mod images;
mod live;
mod pixel_budget;
mod routing;

fn request(action: &str) -> ComputerUseInput {
    serde_json::from_value(serde_json::json!({"action": action})).unwrap()
}

fn script(directory: &Path, name: &str, body: &str) -> PathBuf {
    let path = directory.join(name);
    std::fs::write(&path, format!("#!/bin/sh\nset -eu\n{body}\n")).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
    path
}

fn png() -> Vec<u8> {
    let image = image::RgbImage::from_pixel(80, 40, image::Rgb([20, 40, 60]));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}
