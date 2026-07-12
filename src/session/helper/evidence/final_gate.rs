use crate::session::Session;

pub(crate) fn gate(answer: &str, session: &Session) -> String {
    let mode = super::gate_mode::current();
    if mode == super::gate_mode::GateMode::Off {
        return answer.to_string();
    }
    let answer = super::verification_gate::apply(mode, answer);
    let ledger = super::ledger::build(session);
    super::ledger_persist::save(&ledger);
    super::writeback_persist::save(session, &ledger);
    super::palace_sync::save(&ledger);
    if ledger.items.is_empty() {
        return answer;
    }
    let missing = super::ledger::unclassified_count(&ledger, &answer);
    if missing == 0 {
        return answer;
    }
    super::final_note::render(mode, &answer, &session.id, missing)
}
