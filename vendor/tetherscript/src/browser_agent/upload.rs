//! Page method for deterministic file-input uploads.

#[path = "upload_script.rs"]
mod upload_script;
#[path = "upload_validate.rs"]
mod upload_validate;

use super::files::FilePayload;
use crate::browser_agent::action::ActionReport;
use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve;
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Set deterministic file metadata on an `input[type=file]`.
    ///
    /// The action persists metadata in `data-agent-file-*` attributes, updates
    /// the input value to a deterministic fake path, and dispatches `input` and
    /// `change` events through the page's persistent JavaScript runtime.
    ///
    /// # Errors
    ///
    /// Returns an error if the locator does not resolve, resolves to a
    /// non-file input, targets a disabled input, or supplies multiple files to
    /// an input without the `multiple` attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::{BrowserPage, FilePayload, Locator};
    ///
    /// let mut page = BrowserPage::from_html("mem://upload", "<input id='u' type='file'>");
    /// let files = vec![FilePayload::new("note.txt", "text/plain", b"hello".to_vec())];
    /// let report = page.set_input_files(&Locator::css("#u"), files).unwrap();
    ///
    /// assert_eq!(report.action, "set_input_files");
    /// assert!(page.session.html.contains("data-agent-file-count=\"1\""));
    /// ```
    pub fn set_input_files(
        &mut self,
        locator: &Locator,
        files: Vec<FilePayload>,
    ) -> Result<ActionReport, String> {
        let resolved = resolve::resolve(&self.session, self.viewport_width, locator)?;
        upload_validate::file_input(locator, &resolved.dom.element, &files)?;
        self.eval_js(&upload_script::set_files(&resolved.dom.path, &files))?;
        self.session.trace.push(TraceEvent::new(
            "set_input_files",
            format!("{:?}", locator.kind),
        ));
        Ok(ActionReport::new(
            "set_input_files",
            format!("{:?}", locator.kind),
            resolved.bounds,
            ActionabilityReport::new(false),
        ))
    }
}
