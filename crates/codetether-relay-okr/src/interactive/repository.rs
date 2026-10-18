//! Optional repository initialization for interactive objective approval.
use codetether_okr::OkrRepository;
use std::sync::Arc;
/// Initialize the repository only when the caller has none.
/// # Arguments
/// * `repo` - Current optional repository.
/// # Returns
/// Leaves existing repositories untouched; unavailable initialization stays absent.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let mut repo = None;
/// codetether_relay_okr::interactive::ensure_repository(&mut repo).await;
/// # }
/// ```
pub async fn ensure_repository(repo: &mut Option<Arc<OkrRepository>>) {
    if repo.is_none() {
        if let Ok(new_repo) = OkrRepository::from_config().await {
            *repo = Some(Arc::new(new_repo));
        }
    }
}
