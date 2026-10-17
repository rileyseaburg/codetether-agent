//! construct for Kubernetes tool execution.
use super::*;
/// Executable Kubernetes adapter over the shared resource manager.
///
/// # Examples
/// ```rust
/// let _tool = codetether_k8s_tool::K8sTool::new();
/// ```

pub struct K8sTool {
    manager: Arc<OnceCell<K8sManager>>,
}

impl K8sTool {
    /// Construct the tool with lazy manager initialization.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_k8s_tool::K8sTool::new();
    /// ```
    pub fn new() -> Self {
        Self {
            manager: Arc::new(OnceCell::new()),
        }
    }

    /// Create a K8sTool with a pre-initialized manager.
    /// # Arguments
    /// * `manager` - Already initialized manager shared by this tool.
    /// # Returns
    /// A tool that reuses the supplied manager.
    /// # Examples
    /// ```rust,no_run
    /// # fn example(manager: codetether_k8s::K8sManager) {
    /// let _tool = codetether_k8s_tool::K8sTool::with_manager(manager);
    /// # }
    /// ```
    pub fn with_manager(manager: K8sManager) -> Self {
        let cell = OnceCell::new();
        let _ = cell.set(manager);
        Self {
            manager: Arc::new(cell),
        }
    }

    pub(crate) async fn get_manager(&self) -> &K8sManager {
        self.manager
            .get_or_init(|| async { K8sManager::new().await })
            .await
    }
}

impl Default for K8sTool {
    fn default() -> Self {
        Self::new()
    }
}
