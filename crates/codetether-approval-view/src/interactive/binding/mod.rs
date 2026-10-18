//! Frontend pending-approval key, copy, edit and scrolling bindings.
mod host;
/// Declare approval keyboard actions over existing preview/editor state.
///
/// Shared policy retains decision, clipboard and scrolling behavior. The
/// caller supplies its existing approval command dispatcher and editor types.
/// # Examples
/// See the frontend approval-key module for concrete type selection.
#[macro_export]
macro_rules! declare_frontend_approval_keys {
    ($app:path, $mode:path, $buffer:path, $session:path, $key:path, $code:path;
     $command:path $(,)?) => {
        use ::std::path::Path;
        use $app as App;
        use $code as KeyCode;
        use $key as KeyEvent;
        $crate::__frontend_approval_keys_host!($mode, $buffer, $session, $command);
        pub(super) fn handle(app: &mut App, character: char, cwd: &Path) -> bool {
            $crate::interactive::handle(&mut host::Native(app), character, cwd)
        }
        pub(super) fn copy_preview(app: &mut App) -> bool {
            $crate::interactive::copy_preview(&mut host::Native(app))
        }
        pub(super) fn edit(app: &mut App, cwd: &Path) -> bool {
            $crate::interactive::open(&mut host::Native(app), cwd)
        }
        pub(super) fn scroll(app: &mut App, key: KeyEvent) -> bool {
            $crate::interactive::scroll(&mut host::Native(app), key)
        }
        pub(super) fn wheel(app: &mut App, direction: KeyCode, amount: usize) -> bool {
            $crate::interactive::wheel(&mut host::Native(app), direction, amount)
        }
    };
}
