//! Regression for the harness's embedded TetherScript HTTPS feature wiring.
//!
//! No network requests, credentials, or temporary files are used. The host must
//! provide a usable platform CA store, as required by native verified HTTPS.

#![cfg(feature = "tetherscript")]

#[test]
fn embedded_tetherscript_constructs_verified_tls_connector() {
    tetherscript::tls::TlsConnector::new()
        .expect("embedded TetherScript needs openssl-tls and native CA roots");
}
