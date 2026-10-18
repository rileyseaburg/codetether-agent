//! Chat composer textbox, cursor, attachment labels and autocomplete display.
//!
//! Hosts provide a borrowed [ComposerView] to [render_input]. Editing and
//! command selection remain host responsibilities.
mod attachment;
mod cursor;
mod input_area;
mod input_title;
mod suggestion_hint;
mod suggestion_style;
mod suggestions;
mod view;
pub use attachment::attachment_suffix;
pub use cursor::place_cursor;
pub use input_area::render_input;
pub use input_title::build as build_title;
pub use suggestion_style::highlight_style;
pub use suggestions::render_suggestions;
pub use view::ComposerView;
