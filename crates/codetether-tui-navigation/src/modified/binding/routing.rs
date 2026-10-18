//! Frontend routing effects for modifier shortcuts.
/// Implementation detail of [crate::declare_frontend_modified_keys].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_modified_routing {
    ($key:path; $alt:path, $approval:path, $ctrl_c:path, $fallback:path) => {
        mod routing {
            use super::Native;
            use $crate::modified::Routing;
            use $key as KeyEvent;
            impl Routing for Native<'_> {
                type Error = anyhow::Error;
                fn alt_scroll(&mut self, key: KeyEvent) -> Option<anyhow::Result<bool>> {
                    $alt(self.app, key)
                }
                fn approval(&mut self, character: char) -> bool {
                    $approval(self.app, character, self.cwd)
                }
                fn ctrl_c(&mut self) -> bool {
                    $ctrl_c(self.app, self.runtime)
                }
                fn fallback(&mut self, key: KeyEvent, ctrl: bool) -> Option<anyhow::Result<bool>> {
                    $fallback(self.app, self.cwd, key, ctrl)
                }
            }
        }
    };
}
