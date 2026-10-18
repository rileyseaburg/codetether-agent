//! Frontend layers operations for ordered key-event dispatch.
/// Implementation detail of [crate::declare_frontend_key_dispatch].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_key_layers {
    ($question:path, $editor:path, $scroll:path, $okr:path) => {
        mod layers {
            use super::super::KeyEvent;
            use super::Native;
            use $crate::events::Layers;
            impl Layers for Native<'_> {
                fn question(&mut self, key: KeyEvent) -> bool {
                    $question(self.app, key)
                }
                async fn editor(&mut self, key: KeyEvent) -> bool {
                    $editor(self.app, self.slot, self.runtime, key).await
                }
                fn approval_scroll(&mut self, key: KeyEvent) -> bool {
                    $scroll(self.app, key)
                }
                async fn okr(&mut self, key: KeyEvent) -> bool {
                    $okr(self.app, key).await
                }
            }
        }
    };
}
