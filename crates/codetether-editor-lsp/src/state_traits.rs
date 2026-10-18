//! Clone and default state without imposing manager trait bounds.
use crate::EditorLspState;
impl<M> Clone for EditorLspState<M> {
    fn clone(&self) -> Self {
        Self {
            manager: self.manager.clone(),
            hover: self.hover.clone(),
            area: self.area,
        }
    }
}
impl<M> Default for EditorLspState<M> {
    fn default() -> Self {
        Self {
            manager: None,
            hover: None,
            area: Default::default(),
        }
    }
}
