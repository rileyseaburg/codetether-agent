//! Parse credentials into the shared owner without copying the payload.
use super::{BoundTaskKeys, PerTaskProviderKeys, PhantomData};
use anyhow::Result;
impl<H, R> BoundTaskKeys<H, R> {
    /// Parse a populated provider-key claim.
    /// # Arguments
    /// `value` contains the original tenant credential claim.
    /// # Returns
    /// An owned credential set, or `None` for an absent claim.
    /// # Errors
    /// Propagates the shared credential parser's errors.
    /// # Examples
    /// ```
    /// let keys = codetether_provider_registry::BoundTaskKeys::<(), ()>::from_value(
    ///     &serde_json::Value::Null,
    /// )?;
    /// assert!(keys.is_none());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_value(value: &serde_json::Value) -> Result<Option<Self>> {
        PerTaskProviderKeys::from_value(value).map(|keys| {
            keys.map(|inner| Self {
                inner,
                binding: PhantomData,
            })
        })
    }
}
