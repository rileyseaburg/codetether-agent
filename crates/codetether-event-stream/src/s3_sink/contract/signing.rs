//! signing contract.
//! Deterministic signing and serialized configuration contracts.
use super::*;

use reqwest::Method;

#[tokio::test]
async fn signing_and_endpoint_contract() {
    let sink = S3Sink::from_config(config(None, false)).await.unwrap();
    assert_eq!(
        sink.endpoint_url(),
        "https://bucket.s3.us-west-2.amazonaws.com"
    );
    let signature = sink.generate_aws_header(
        &Method::PUT,
        "/events/run.jsonl",
        "",
        "payload",
        "20261017T120000Z",
    );
    assert_eq!(
        signature,
        "AWS4-HMAC-SHA256 Credential=fixture-access/20261017/us-west-2/s3/aws4_request, SignedHeaders=content-type;x-amz-content-sha256;x-amz-date, Signature=98ae929b573305edfd1824a09e7989670f396676b438440ca83d688b8fd3dc61"
    );
    assert!(signature.starts_with(
        "AWS4-HMAC-SHA256 Credential=fixture-access/20261017/us-west-2/s3/aws4_request,"
    ));
    let sink = S3Sink::from_config(config(Some("http://localhost/root/".into()), false))
        .await
        .unwrap();
    assert_eq!(sink.endpoint_url(), "http://localhost/root/bucket");
}
