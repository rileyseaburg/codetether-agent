//! Persist satisfaction decisions before permitting any goal resumption.

use crate::tui::app::{session_runtime::SessionSlot, state::App};
use anyhow::Result;
use crossterm::event::KeyEvent;

mod host;

pub(super) async fn handle(
    app: &mut App,
    slot: &SessionSlot,
    key: KeyEvent,
) -> Result<Option<bool>> {
    codetether_answer_review_view::interactive::handle(&mut host::Native { app, slot }, key).await
}

#[cfg(test)]
#[path = "answer_review_key_tests.rs"]
mod tests;
