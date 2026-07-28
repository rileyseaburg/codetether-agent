//! # Canvas Inspection
//!
//! This module exposes deterministic canvas command logs captured by the
//! JavaScript host. Use [`BrowserPage::canvas_surface`] to inspect one canvas.

#[path = "canvas_model.rs"]
mod canvas_model;
#[path = "canvas_page.rs"]
mod canvas_page;
#[path = "canvas_parse.rs"]
mod canvas_parse;
#[path = "canvas_webgl_commands.rs"]
mod canvas_webgl_commands;
#[path = "canvas_webgl_model.rs"]
mod canvas_webgl_model;
#[path = "canvas_webgl_page.rs"]
mod canvas_webgl_page;
#[path = "canvas_webgl_parse.rs"]
mod canvas_webgl_parse;

#[cfg(test)]
#[path = "canvas_tests.rs"]
mod canvas_tests;
#[cfg(test)]
#[path = "canvas_webgl_tests.rs"]
mod canvas_webgl_tests;

pub use canvas_model::{CanvasCommand, CanvasSurface};
pub use canvas_webgl_model::{WebGlCommand, WebGlContextSnapshot};
