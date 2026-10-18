//! Settings panel data, row composition and terminal rendering.
//!
//! Supply [SettingsView] from the application, then call [settings_lines] for
//! text rows or [render_settings] for the panel with a host status-footer callback.
mod access_mode_line;
mod data;
mod lines;
mod provider_lines;
mod render;
mod row_style;
mod setting_line;
mod setting_value_span;
mod value_line;
pub use data::SettingsView;
pub use lines::settings_lines;
pub use render::render_settings;
