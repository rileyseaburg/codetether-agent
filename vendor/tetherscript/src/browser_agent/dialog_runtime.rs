//! Runtime bridge between page JavaScript and dialog records.

use crate::browser_agent::dialog::{dialog_decisions, dialog_parse, dialog_records, dialog_script};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    pub(crate) fn install_dialog_bridge(&mut self) -> Result<(), String> {
        let decisions = dialog_decisions::decisions(&self.session.storage);
        let script = dialog_script::install(&decisions);
        self.runtime_mut()?.eval(&script)?;
        Ok(())
    }

    pub(crate) fn collect_dialogs(&mut self) -> Result<(), String> {
        let value = self.runtime_mut()?.eval(dialog_script::drain())?.value;
        let (consumed, captured) = dialog_parse::captured_dialogs(value);
        let mut records = self.dialogs();
        let start = records.len() as u64;
        records.extend(captured.into_iter().enumerate().map(|(index, mut record)| {
            record.sequence = start + index as u64;
            record
        }));
        dialog_records::write_records(&mut self.session.storage, &records);
        dialog_decisions::discard_decisions(&mut self.session.storage, consumed);
        Ok(())
    }
}
