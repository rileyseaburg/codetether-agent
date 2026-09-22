use super::*;

#[test]
fn compiled_flag_matches_feature() {
    crate::tls::ensure_rustls_crypto_provider();
    assert_eq!(fips_status().compiled, cfg!(feature = "fips"));
}

#[test]
fn enforced_requires_every_layer() {
    let partial = FipsStatus {
        compiled: true,
        module_active: true,
        provider_approved: false,
    };
    assert!(!partial.enforced());
    let full = FipsStatus {
        provider_approved: true,
        ..partial
    };
    assert!(full.enforced());
}

#[cfg(not(feature = "fips"))]
#[test]
fn default_build_is_not_fips() {
    crate::tls::ensure_rustls_crypto_provider();
    let status = fips_status();
    assert!(!status.module_active);
    assert!(!status.enforced());
}

#[cfg(feature = "fips")]
#[test]
fn fips_build_is_enforced() {
    crate::tls::ensure_rustls_crypto_provider();
    assert!(require_fips().expect("FIPS active").enforced());
}
