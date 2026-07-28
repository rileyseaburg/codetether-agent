//! IndexedDB record model.

/// One deterministic IndexedDB-like key/value record.
///
/// Records are sorted by origin, database, object store, key, then value when
/// returned from listing APIs.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::IndexedDbRecord;
///
/// let record = IndexedDbRecord {
///     origin: "https://example.test".into(),
///     database: "app".into(),
///     object_store: "settings".into(),
///     key: "theme".into(),
///     value: "dark".into(),
/// };
/// assert_eq!(record.key, "theme");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexedDbRecord {
    /// Origin key such as `https://example.test`.
    pub origin: String,
    /// Database name.
    pub database: String,
    /// Object-store name within the database.
    pub object_store: String,
    /// String record key.
    pub key: String,
    /// String record value.
    pub value: String,
}

impl IndexedDbRecord {
    pub(crate) fn new(
        origin: impl Into<String>,
        database: impl Into<String>,
        object_store: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            origin: origin.into(),
            database: database.into(),
            object_store: object_store.into(),
            key: key.into(),
            value: value.into(),
        }
    }
}
