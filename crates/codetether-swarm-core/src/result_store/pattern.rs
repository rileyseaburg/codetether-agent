/// Subscription pattern for result notifications
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// use codetether_swarm_core::result_store::SubscriptionPattern;
/// let pattern = SubscriptionPattern::Exact("result".into());
/// match pattern {
///     SubscriptionPattern::Exact(key) => assert_eq!(key, "result"),
///     SubscriptionPattern::Prefix(_) | SubscriptionPattern::Tag(_) |
///     SubscriptionPattern::Producer(_) | SubscriptionPattern::All => unreachable!(),
/// }
/// ```
pub enum SubscriptionPattern {
    /// Exact key match
    Exact(String),
    /// Prefix match (key starts with)
    Prefix(String),
    /// Tag match (result has any of these tags)
    Tag(Vec<String>),
    /// Producer match (results from specific subtask)
    Producer(String),
    /// All results
    All,
}
