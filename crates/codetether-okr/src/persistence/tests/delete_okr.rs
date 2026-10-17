//! Repository test_delete_okr regression.

use super::*;

#[tokio::test]
async fn test_delete_okr() {
    let repo = temp_repo();

    let mut okr = Okr::new("Test", "Desc");
    let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
    okr.add_key_result(kr);

    let created = repo.create_okr(okr).await.unwrap();
    let deleted = repo.delete_okr(created.id).await.unwrap();
    assert!(deleted);

    let fetched = repo.get_okr(created.id).await.unwrap();
    assert!(fetched.is_none());
}
