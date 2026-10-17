//! Signing-key derivation and Authorization header construction.
use super::*;
impl SigningClient<'_> {
    pub(super) fn authorization(
        &self,
        creds: &AwsCredentials,
        datestamp: &str,
        service: &str,
        amz_date: &str,
        canonical_request: &str,
        signed_headers: &str,
    ) -> String {
        let credential_scope = format!("{datestamp}/{}/{service}/aws4_request", self.region);

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{amz_date}\n{credential_scope}\n{}",
            sha256_hex(canonical_request.as_bytes())
        );

        let k_date = hmac_sha256(
            format!("AWS4{}", creds.secret_access_key).as_bytes(),
            datestamp.as_bytes(),
        );
        let k_region = hmac_sha256(&k_date, self.region.as_bytes());
        let k_service = hmac_sha256(&k_region, service.as_bytes());
        let k_signing = hmac_sha256(&k_service, b"aws4_request");

        let signature = hex::encode(hmac_sha256(&k_signing, string_to_sign.as_bytes()));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{credential_scope}, SignedHeaders={signed_headers}, Signature={signature}",
            creds.access_key_id
        );
        authorization
    }
}
