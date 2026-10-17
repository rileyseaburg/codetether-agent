//! repository for OKR tool dispatch.
use super::*;

pub(super) async fn get_repo() -> Result<&'static Arc<OkrRepository>> {
    OKR_REPO
        .get_or_try_init(|| async {
            let repo = OkrRepository::from_config().await?;
            Ok::<_, anyhow::Error>(Arc::new(repo))
        })
        .await
}
