//! Quiet-mode and execution services for interactive forage commands.
use codetether_forage_interactive::Host;
use std::marker::PhantomData;
/// Interactive forage backend using application runtime H.
/// # Examples
/// ```
/// let _: Option<codetether_native_tui::runs::Forage<()>> = None;
/// ```
pub struct Forage<H>(PhantomData<fn() -> H>);
impl<H: codetether_forage_runtime::Host> Host for Forage<H> {
    fn set_quiet(quiet: bool) {
        codetether_forage_reporting::console::set_quiet(quiet);
    }
    async fn execute(
        args: codetether_cli_options::ForageArgs,
    ) -> anyhow::Result<codetether_forage_core::ForageRunSummary> {
        codetether_forage_runtime::execute_with_summary::<H>(args).await
    }
}
