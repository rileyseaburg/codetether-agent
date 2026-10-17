//! Valid local kubeconfig selection, isolated from the parent environment.
use super::*;
#[test]
fn local_kubeconfig_initialization() {
    const SENTINEL: &str = "CODETETHER_TEST_K8S_LOCAL_CONFIG";
    if std::env::var_os(SENTINEL).is_some() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let manager = runtime.block_on(K8sManager::new());
        assert!(manager.is_available());
        assert!(format!("{manager:?}").contains("fixture-local"));
        return;
    }
    let path = std::env::temp_dir().join(format!("k8s-config-{}.yaml", uuid::Uuid::new_v4()));
    std::fs::write(&path, "apiVersion: v1\nkind: Config\ncurrent-context: fixture\nclusters:\n- name: fixture\n  cluster:\n    server: http://127.0.0.1:1\ncontexts:\n- name: fixture\n  context:\n    cluster: fixture\n    user: fixture\nusers:\n- name: fixture\n  user:\n    token: fixture-token\n").unwrap();
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "contract::initialization::local_kubeconfig_initialization",
        ])
        .env(SENTINEL, "1")
        .env("KUBECONFIG", &path)
        .env("CODETETHER_K8S_NAMESPACE", "fixture-local")
        .env_remove("KUBERNETES_SERVICE_HOST")
        .env_remove("KUBERNETES_SERVICE_PORT")
        .output()
        .unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
