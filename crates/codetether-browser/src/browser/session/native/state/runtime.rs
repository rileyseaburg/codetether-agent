use super::NativePage;
use crate::browser::BrowserError;

pub(super) struct NativeRuntime {
    pub pages: Vec<NativePage>,
    pub current: usize,
}

impl NativeRuntime {
    pub fn new() -> Self {
        Self {
            pages: vec![NativePage::new()],
            current: 0,
        }
    }

    pub fn current(&self) -> Result<&NativePage, BrowserError> {
        self.pages
            .get(self.current)
            .ok_or(BrowserError::SessionNotStarted)
    }

    pub fn current_mut(&mut self) -> Result<&mut NativePage, BrowserError> {
        self.pages
            .get_mut(self.current)
            .ok_or(BrowserError::SessionNotStarted)
    }

    pub fn select(&mut self, index: usize) -> Result<(), BrowserError> {
        if index >= self.pages.len() {
            return Err(BrowserError::TabNotFound(index));
        }
        self.current = index;
        Ok(())
    }

    pub fn close(&mut self, index: usize) -> Result<(), BrowserError> {
        if index >= self.pages.len() {
            return Err(BrowserError::TabNotFound(index));
        }
        self.pages.remove(index);
        self.current = self
            .current
            .saturating_sub((index <= self.current) as usize);
        self.current = self.current.min(self.pages.len().saturating_sub(1));
        Ok(())
    }
}
