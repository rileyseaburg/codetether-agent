use super::{args::ImagegenArgs, references, schema};
use std::{collections::VecDeque, path::PathBuf, sync::Mutex};

fn args(paths: Option<Vec<PathBuf>>, count: Option<usize>) -> ImagegenArgs {
    ImagegenArgs {
        prompt: "paint a moonlit lake".into(),
        referenced_image_paths: paths,
        num_last_images_to_include: count,
    }
}

#[test]
fn schema_matches_upstream_selectors() {
    let value = schema::parameters();
    assert_eq!(value["properties"]["referenced_image_paths"]["maxItems"], 5);
    assert_eq!(
        value["properties"]["num_last_images_to_include"]["maximum"],
        5
    );
}

#[test]
fn rejects_conflicting_selectors() {
    assert_eq!(
        args(Some(vec!["image.png".into()]), Some(1))
            .validate()
            .expect_err("selectors must conflict")
            .to_string(),
        "provide only one image selector"
    );
}

#[tokio::test]
async fn recent_images_remain_chronological() {
    let cache = Mutex::new(VecDeque::from([
        "first".into(),
        "second".into(),
        "third".into(),
    ]));
    let images = references::resolve(&args(None, Some(2)), &cache)
        .await
        .unwrap();
    assert_eq!(images, ["second", "third"]);
}

#[tokio::test]
async fn recent_images_require_requested_count() {
    let cache = Mutex::new(VecDeque::from(["only".into()]));
    let error = references::resolve(&args(None, Some(2)), &cache)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("only 1 were available"));
}
