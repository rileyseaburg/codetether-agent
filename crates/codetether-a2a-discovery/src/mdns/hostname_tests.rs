//! Regression cases for DNS labels.

use super::sanitize_hostname;

#[test]
fn normalizes_dns_labels() {
    assert_eq!(sanitize_hostname("my.Host_name"), "my-host-name");
    assert_eq!(sanitize_hostname("---bob---"), "bob");
    assert_eq!(sanitize_hostname("..."), "agent");
}

#[test]
fn caps_labels_at_sixty_three_bytes() {
    let output = sanitize_hostname(&"a".repeat(100));
    assert_eq!(output.len(), 63);
}
