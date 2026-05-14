use crate::session::Session;

pub(crate) fn gate(answer: &str, session: &Session) -> String {
    let mode = super::gate_mode::current();
    if mode == super::gate_mode::GateMode::Off {
        return answer.to_string();
    }
    let ledger = super::ledger::build(session);
    super::ledger_persist::save(&ledger);
    super::writeback_persist::save(session, &ledger);
    if ledger.items.is_empty() {
        return answer.to_string();
    }
    let missing = super::ledger::unclassified_count(&ledger, answer);
    if missing == 0 {
        return answer.to_string();
    }
    super::final_note::render(mode, answer, &session.id, missing)
}
