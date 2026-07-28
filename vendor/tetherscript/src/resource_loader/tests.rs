use super::*;

#[test]
fn discovers_common_resources() {
    let html = r#"<link rel="stylesheet" href="a.css"><script src="b.js"></script><img src="c.png"><iframe src="f.html">"#;
    let r = discover_resources(&html, "https://e.test/p");
    assert!(r
        .iter()
        .any(|x| x.resource_type == ResourceType::Stylesheet));
    assert!(r.iter().any(|x| x.resource_type == ResourceType::Script));
    assert!(r.iter().any(|x| x.resource_type == ResourceType::Image));
}

#[test]
fn scheduler_limits_origin_and_orders() {
    let mut s = LoadScheduler::new(1);
    s.enqueue(ResourceRequest::new(
        "https://a/i.png",
        ResourceType::Image,
        ResourcePriority::Image,
        "img",
    ));
    s.enqueue(ResourceRequest::new(
        "https://a/s.css",
        ResourceType::Stylesheet,
        ResourcePriority::Css,
        "link",
    ));
    // CSS has higher priority so it comes out first
    let first = s.next_startable().unwrap();
    assert_eq!(first.resource_type, ResourceType::Stylesheet);
    // Origin "https://a" now has 1 active (per-origin limit = 1)
    // Image request is still in queue but blocked
    s.finish(&first.url);
    // Now the slot is free, image can start
    let second = s.next_startable().unwrap();
    assert_eq!(second.resource_type, ResourceType::Image);
}

#[test]
fn graph_detects_cycle() {
    let mut g = DependencyGraph::new();
    g.add_js_module("a.js", "b.js");
    g.add_js_module("b.js", "a.js");
    assert!(g.has_cycle());
}
