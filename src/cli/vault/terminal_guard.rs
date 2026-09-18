//! Restore terminal echo and raw state on every prompt exit.
pub(super) struct Restore;
pub(super) fn enter() -> anyhow::Result<Restore> {
    crossterm::terminal::enable_raw_mode()?;
    Ok(Restore)
}
impl Drop for Restore {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
