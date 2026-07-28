//! Retry runner for assertions that observe page state.

use crate::browser_agent::page::BrowserPage;

pub(crate) fn run<T>(
    page: &mut BrowserPage,
    action: &str,
    mut attempt: impl FnMut(&mut BrowserPage) -> Result<T, String>,
) -> Result<T, String> {
    let options = page.wait_options;
    let mut last_error = String::from("no assertion attempt ran");
    for tick in 0..=options.timeout_ticks {
        match attempt(page) {
            Ok(value) => return Ok(value),
            Err(error) => last_error = error,
        }
        if tick < options.timeout_ticks {
            settle(page, action, tick, &last_error)?;
        }
    }
    Err(format!(
        "{action} timed out after {} ticks; {last_error}",
        options.timeout_ticks
    ))
}

fn settle(
    page: &mut BrowserPage,
    action: &str,
    tick: usize,
    last_error: &str,
) -> Result<(), String> {
    page.run_scripts().map_err(|error| {
        format!(
            "{action} failed while settling after tick {tick}; \
             last assertion: {last_error}; settle error: {error}"
        )
    })
}
