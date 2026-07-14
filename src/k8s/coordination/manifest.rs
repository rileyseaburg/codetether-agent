//! Kubernetes Lease manifest construction and state extraction.

use chrono::{DateTime, Utc};
use k8s_openapi::api::coordination::v1::Lease;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{MicroTime, ObjectMeta};

use super::decision::LeaseState;
use super::resource::ResourceKey;

pub fn build(key: &ResourceKey, holder: &str, seconds: i32, now: DateTime<Utc>) -> Lease {
    let timestamp = k8s_openapi::jiff::Timestamp::from_second(now.timestamp()).expect("valid UTC");
    Lease {
        metadata: ObjectMeta {
            name: Some(key.lease_name()),
            namespace: Some(key.namespace.clone()),
            labels: Some([("app.kubernetes.io/managed-by".into(), "codetether".into())].into()),
            annotations: Some([("codetether.run/resource".into(), key.canonical())].into()),
            ..ObjectMeta::default()
        },
        spec: Some(k8s_openapi::api::coordination::v1::LeaseSpec {
            holder_identity: Some(holder.into()),
            lease_duration_seconds: Some(seconds),
            acquire_time: Some(MicroTime(timestamp)),
            renew_time: Some(MicroTime(timestamp)),
            lease_transitions: Some(0),
            ..Default::default()
        }),
    }
}

pub fn state(lease: &Lease) -> LeaseState {
    let spec = lease.spec.as_ref();
    let renewed_at = spec
        .and_then(|s| s.renew_time.as_ref())
        .and_then(|t| DateTime::from_timestamp(t.0.as_second(), 0));
    LeaseState {
        holder: spec.and_then(|s| s.holder_identity.clone()),
        renewed_at,
        duration_seconds: spec.and_then(|s| s.lease_duration_seconds).unwrap_or(1),
    }
}
