//! Cache ownership for a worker's discovered provider models.
use super::{Host, Models};
use anyhow::Result;
use tokio::sync::OnceCell;

/// A worker catalog initialized by the first successful discovery.
///
/// # Examples
/// ```
/// let _catalog = codetether_model_discovery::worker::Catalog::new();
/// ```
#[derive(Default)]
pub struct Catalog {
    cache: OnceCell<Models>,
}
impl Catalog {
    /// Create an empty catalog suitable for process-static storage.
    pub const fn new() -> Self {
        Self {
            cache: OnceCell::const_new(),
        }
    }

    /// Load or clone the cached worker inventory.
    ///
    /// # Arguments
    /// The `H` type supplies application registry loading operations.
    /// # Returns
    /// A cloned inventory, including an empty inventory when discovery succeeds.
    /// # Errors
    /// Returns the fallback registry error if no primary registry is available.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_model_discovery::worker::Host>() -> anyhow::Result<()> {
    /// let catalog = codetether_model_discovery::worker::Catalog::new();
    /// let _models = catalog.load::<H>().await?;
    /// # Ok(()) }
    /// ```
    pub async fn load<H: Host>(&self) -> Result<Models> {
        self.cache
            .get_or_try_init(super::load::uncached::<H>)
            .await
            .cloned()
    }
}
