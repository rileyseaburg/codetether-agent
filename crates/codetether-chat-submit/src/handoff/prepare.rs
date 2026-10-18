use super::Preparation;
use std::path::Path;
/// Prepare and transfer an idle session using current UI execution settings.
/// # Arguments
/// Host owns the session slot; cwd supplies the optional worktree root; prompt
/// and images are moved into the constructed execution request.
/// # Returns
/// Completes after rejection or handoff, restoring ownership on transfer failure.
/// # Examples
/// ```ignore
/// prepare(&mut native, cwd, prompt, images).await;
/// ```
pub async fn prepare<H: Preparation>(
    host: &mut H,
    cwd: &Path,
    prompt: &str,
    pending_images: Vec<H::Image>,
) {
    let Some(original_dir) = host.original_directory() else {
        host.status("Session is already running");
        return;
    };
    let use_worktree = host.use_worktree();
    host.configure(use_worktree);
    let worktree_root = use_worktree.then(|| cwd.to_path_buf());
    let Some(session) = host.take_session() else {
        return;
    };
    let request = host.request(
        session,
        prompt.to_string(),
        pending_images,
        original_dir,
        worktree_root,
    );
    super::submit(host, request).await;
}
