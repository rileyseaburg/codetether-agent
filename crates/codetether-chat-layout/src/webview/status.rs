/// Return status-bar rows for the available width.
/// # Arguments
/// * `width` - Terminal width in columns.
/// # Returns
/// One row at 140 columns or wider, otherwise two.
/// # Examples
/// ```
/// assert_eq!(codetether_chat_layout::webview::status_height(100), 2);
/// ```
pub fn status_height(width: u16) -> u16 {
    if width >= 140 { 1 } else { 2 }
}
