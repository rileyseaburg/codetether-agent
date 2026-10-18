//! Application session title access for native task-title preseeding.
#[cfg(test)]
pub(super) use codetether_native_worker::task_title::preseed_task_title;
impl codetether_native_worker::task_title::Title for crate::session::Session {
    fn task_title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    fn set_task_title(&mut self, title: String) {
        self.set_title(title);
    }
}
