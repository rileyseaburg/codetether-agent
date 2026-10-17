//! Harvest filesystem error contract.
mod support;
use codetether_distill::harvest::{HarvestFilter, harvest_from_dir};
use support::Directory;
#[test]
fn missing_directory_is_empty_but_invalid_input_propagates() {
    let dir = Directory::new();
    let filter = HarvestFilter::default();
    assert!(harvest_from_dir(&dir.0, &filter).unwrap().is_empty());
    std::fs::write(dir.0.join("sft"), "not a directory").unwrap();
    assert!(harvest_from_dir(&dir.0, &filter).is_err());
    std::fs::remove_file(dir.0.join("sft")).unwrap();
    std::fs::create_dir(dir.0.join("sft")).unwrap();
    std::fs::write(dir.0.join("sft/bad.jsonl"), [0xff]).unwrap();
    assert!(harvest_from_dir(&dir.0, &filter).is_err());
}
