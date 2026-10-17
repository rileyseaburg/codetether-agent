//! Version selection and the current non-training stub contract.
mod support;
use codetether_distill::trainer::{TrainConfig, train_lora};
use support::Directory;

#[test]
fn version_selection_includes_files_and_leaves_output_untouched() {
    let dir = Directory::new();
    std::fs::create_dir(dir.0.join("v1")).unwrap();
    std::fs::write(dir.0.join("v4"), "existing").unwrap();
    std::fs::create_dir(dir.0.join("vbad")).unwrap();
    let config = TrainConfig {
        output_dir: dir.0.to_str().unwrap().to_owned(),
        ..Default::default()
    };
    let result = train_lora(&config, &dir.0.join("missing-records")).unwrap();
    assert_eq!(result.version, 5);
    assert_eq!(result.adapter_path, format!("{}/v5", config.output_dir));
    assert_eq!(
        (result.train_loss, result.val_loss, result.records_used),
        (0.0, 0.0, 0)
    );
    assert!(!dir.0.join("v5").exists());
}

#[test]
fn missing_output_starts_at_one_without_creating_it() {
    let dir = Directory::new();
    let output = dir.0.join("missing-output");
    let config = TrainConfig {
        output_dir: output.to_str().unwrap().to_owned(),
        ..Default::default()
    };
    assert_eq!(
        train_lora(&config, &dir.0.join("missing-records"))
            .unwrap()
            .version,
        1
    );
    assert!(!output.exists());
    let defaults = TrainConfig::default();
    assert_eq!(
        (defaults.lora_rank, defaults.epochs, defaults.batch_size),
        (16, 3, 4)
    );
    assert_eq!(defaults.learning_rate, 2e-4);
}
