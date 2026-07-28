//! Clipboard object API bridge composition.

mod item;
mod nav;
mod state;
mod thenable;

pub(super) fn source() -> String {
    [thenable::SOURCE, item::SOURCE, state::SOURCE, nav::SOURCE].join("")
}
