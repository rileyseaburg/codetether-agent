//! Original objective/run fixture for story-to-key-result regression coverage.

use codetether_okr::{KeyResult, Okr, OkrRun};
use uuid::Uuid;
pub(super) fn create() -> (Okr, OkrRun) {
    let okr_id = Uuid::new_v4();
    let mut okr = Okr::new("Test OKR", "Test description");
    okr.id = okr_id;

    let kr1 = KeyResult::new(okr_id, "Stories complete", 100.0, "%");
    let kr2 = KeyResult::new(okr_id, "No errors", 0.0, "count");
    okr.add_key_result(kr1);
    okr.add_key_result(kr2);

    let run = OkrRun::new(okr_id, "Test Run");

    (okr, run)
}
