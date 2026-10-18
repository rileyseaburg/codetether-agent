//! Start mention selection or insert a literal at-sign.
use crate::{Composer, starts_mention};

/// Open the picker, or insert a literal `@` when not at a word boundary.
///
/// Returns `true` when the mention picker was opened.
/// # Arguments
/// * `app` - Host composer, whose editor inserts the at-sign.
/// # Returns
/// Whether mention selection opened; a mid-word at-sign is inserted literally.
/// # Examples
/// ```no_run
/// # fn example(app: &mut impl codetether_symbol_mention::Composer) {
/// let opened = codetether_symbol_mention::open(app);
/// # }
/// ```
pub fn open(app: &mut impl Composer) -> bool {
    if !starts_mention(app.input(), app.cursor()) {
        app.insert_char('@');
        return false;
    }
    let start = app.cursor();
    app.insert_char('@');
    app.picker_mut().open_mention(start);
    app.set_status("Type a symbol name, then Enter to attach".to_string());
    true
}
