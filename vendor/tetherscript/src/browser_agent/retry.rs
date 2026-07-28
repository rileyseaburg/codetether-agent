//! Bounded deterministic retry loop for page waits and actions.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;

pub(crate) fn run<T>(
    page: &mut BrowserPage,
    action: &str,
    locator: &Locator,
    mut attempt: impl FnMut(&mut BrowserPage) -> Result<T, String>,
) -> Result<T, String> {
    let options = page.wait_options;
    let mut last_error = String::from("no attempt ran");
    for tick in 0..=options.timeout_ticks {
        match attempt(page) {
            Ok(value) => return Ok(value),
            Err(error) => last_error = error,
        }
        if tick < options.timeout_ticks {
            settle(page, action, locator, tick, &last_error)?;
        }
    }
    Err(format!(
        "{action} timed out after {} ticks for locator {:?}; last error: {last_error}",
        options.timeout_ticks, locator.kind
    ))
}

fn settle(
    page: &mut BrowserPage,
    action: &str,
    locator: &Locator,
    tick: usize,
    last_error: &str,
) -> Result<(), String> {
    page.run_scripts().map_err(|error| {
        format!(
            "{action} failed while settling locator {:?} after tick {tick}; last error: {last_error}; settle error: {error}",
            locator.kind
        )
    })
}
