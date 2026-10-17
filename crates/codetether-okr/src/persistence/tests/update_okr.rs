//! Repository test_update_okr regression.

use super::*;

#[tokio::test]
async fn test_update_okr() {
    let repo = temp_repo();

    let mut okr = Okr::new("Test", "Desc");
    let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
    okr.add_key_result(kr);

    let created = repo.create_okr(okr).await.unwrap();

    let mut to_update = created.clone();
    to_update.title = "Updated Title".to_string();
    to_update.status = OkrStatus::Active;

    let updated = repo.update_okr(to_update).await.unwrap();
    assert_eq!(updated.title, "Updated Title");
    assert_eq!(updated.status, OkrStatus::Active);
}
