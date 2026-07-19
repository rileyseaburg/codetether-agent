//! Prompt-lifetime renewal and deterministic release of mux leases.

pub(in crate::session::helper) struct LeaseTurn {
    owner: Option<String>,
    renewal: Option<tokio::task::JoinHandle<()>>,
}

impl LeaseTurn {
    pub(in crate::session::helper) fn open(owner: &str) -> Self {
        if !crate::mux::coordination::active() {
            return Self {
                owner: None,
                renewal: None,
            };
        }
        let owner = owner.to_string();
        let renewal_owner = owner.clone();
        let renewal = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                if let Err(error) = crate::mux::coordination::renew(&renewal_owner).await {
                    tracing::warn!(owner = %renewal_owner, error = %error, "Mux lease renewal failed");
                }
            }
        });
        Self {
            owner: Some(owner),
            renewal: Some(renewal),
        }
    }

    pub(in crate::session::helper) async fn close(self) {
        if let Some(renewal) = self.renewal {
            renewal.abort();
        }
        let Some(owner) = self.owner else { return };
        if let Err(error) = crate::mux::coordination::release(&owner).await {
            tracing::warn!(owner, error = %error, "Mux lease release failed");
        }
    }
}
