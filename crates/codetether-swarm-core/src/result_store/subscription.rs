use super::*;
impl ResultStore {
    /// Subscribe to result notifications
    pub fn subscribe(&self) -> broadcast::Receiver<ResultNotification> {
        self.notification_tx.subscribe()
    }

    /// Register a subscription pattern for a subtask
    pub async fn register_subscription(
        &self,
        subtask_id: impl Into<String>,
        pattern: SubscriptionPattern,
    ) {
        let subtask_id = subtask_id.into();
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.entry(subtask_id).or_default().push(pattern);
    }

    /// Unregister all subscriptions for a subtask
    pub async fn unregister_subscriptions(&self, subtask_id: &str) {
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.remove(subtask_id);
    }

    /// Check if a result matches a subscription pattern
    pub fn matches_pattern(result: &SharedResult, pattern: &SubscriptionPattern) -> bool {
        match pattern {
            SubscriptionPattern::Exact(key) => result.key == *key,
            SubscriptionPattern::Prefix(prefix) => result.key.starts_with(prefix),
            SubscriptionPattern::Tag(tags) => tags.iter().any(|t| result.tags.contains(t)),
            SubscriptionPattern::Producer(producer) => result.producer_id == *producer,
            SubscriptionPattern::All => true,
        }
    }
}
