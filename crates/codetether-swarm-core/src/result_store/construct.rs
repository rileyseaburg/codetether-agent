use super::*;
impl ResultStore {
    /// Create a new result store
    pub fn new() -> Self {
        let (notification_tx, _) = broadcast::channel(1000);
        Self {
            results: RwLock::new(HashMap::new()),
            notification_tx,
            subscriptions: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new result store wrapped in an Arc
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }
}

impl Default for ResultStore {
    fn default() -> Self {
        Self::new()
    }
}
