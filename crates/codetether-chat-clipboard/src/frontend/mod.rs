//! Frontend module bindings for clipboard paste and copy shortcuts.
mod host;
/// Declare the frontend clipboard host and keyboard action entry points.
///
/// The caller supplies its paste path and scroll sentinel; shared clipboard
/// code retains message selection, image fallback and error reporting.
/// # Examples
/// See the frontend event-handler module for concrete type and callback paths.
#[macro_export]
macro_rules! declare_frontend_chat_clipboard {
    ($app:path, $message:path, $image:path;
     $bottom:path, $paste:path, $transcript:path $(,)?) => {
        $crate::__frontend_chat_clipboard_host!($app, $message, $image; $bottom, $paste);
        mod clipboard {
            use $app as App;
            pub(super) fn handle_clipboard_paste(app: &mut App) {
                $crate::handle_clipboard_paste(&mut super::chat_clipboard_host::Native(app));
            }
        }
        mod copy_reply {
            use $app as App;
            pub(super) fn handle_copy_reply(app: &mut App) {
                $crate::handle_copy_reply(&mut super::chat_clipboard_host::Native(app));
            }
        }
        mod copy_transcript {
            use $app as App;
            pub(crate) use $transcript as build_transcript;
            pub(super) fn handle_copy_transcript(app: &mut App) {
                $crate::handle_copy_transcript(&mut super::chat_clipboard_host::Native(app));
            }
        }
    };
}
