//! Public page APIs for deterministic JavaScript dialogs.

use crate::browser_agent::dialog::{dialog_decisions, dialog_records};
use crate::browser_agent::dialog::{DialogDecision, DialogKind, DialogRecord};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Record a dialog without executing page JavaScript.
    pub fn record_dialog(
        &mut self,
        kind: DialogKind,
        message: impl Into<String>,
        default_value: Option<String>,
    ) -> DialogRecord {
        let mut records = self.dialogs();
        let record = DialogRecord {
            sequence: records.len() as u64,
            kind,
            message: message.into(),
            default_value,
            accepted: None,
            response: None,
        };
        records.push(record.clone());
        dialog_records::write_records(&mut self.session.storage, &records);
        record
    }

    /// Return dialog records in deterministic insertion order.
    pub fn dialogs(&self) -> Vec<DialogRecord> {
        dialog_records::records(&self.session.storage)
    }

    /// Remove all recorded dialogs from this page.
    pub fn clear_dialogs(&mut self) {
        dialog_records::clear_records(&mut self.session.storage);
    }

    /// Accept the next JavaScript dialog with its default response.
    pub fn accept_next_dialog(&mut self) {
        self.queue_dialog_decision(DialogDecision::Accept);
    }

    /// Accept the next JavaScript prompt with the supplied response.
    pub fn accept_next_prompt(&mut self, value: impl Into<String>) {
        self.queue_dialog_decision(DialogDecision::Prompt(value.into()));
    }

    /// Dismiss the next JavaScript dialog.
    pub fn dismiss_next_dialog(&mut self) {
        self.queue_dialog_decision(DialogDecision::Dismiss);
    }

    /// Queue an explicit decision for the next JavaScript dialog.
    pub fn queue_dialog_decision(&mut self, decision: DialogDecision) {
        dialog_decisions::append_decision(&mut self.session.storage, decision);
    }
}
