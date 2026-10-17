//! Repository test_query_okrs_by_status regression.

use super::*;

#[tokio::test]
async fn test_query_okrs_by_status() {
    let repo = temp_repo();

    let mut okr1 = Okr::new("Active OKR", "Desc");
    let kr1 = KeyResult::new(okr1.id, "KR1", 100.0, "%");
    okr1.add_key_result(kr1);
    okr1.status = OkrStatus::Active;

    let mut okr2 = Okr::new("Draft OKR", "Desc");
    let kr2 = KeyResult::new(okr2.id, "KR2", 100.0, "%");
    okr2.add_key_result(kr2);
    okr2.status = OkrStatus::Draft;

    repo.create_okr(okr1).await.unwrap();
    repo.create_okr(okr2).await.unwrap();

    let active = repo.query_okrs_by_status(OkrStatus::Active).await.unwrap();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].title, "Active OKR");
}
