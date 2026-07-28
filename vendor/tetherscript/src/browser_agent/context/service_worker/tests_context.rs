use crate::browser_agent::{BrowserContext, BrowserPage, ServiceWorkerState};

#[test]
fn registrations_activate_by_origin_and_scope() {
    let mut context = BrowserContext::new();
    let registration = context.service_worker_register("https://app.test/a", "/app/", "/sw.js");

    assert_eq!(registration.state, ServiceWorkerState::Installing);
    assert!(context.service_worker_activate("https://app.test", "/app/"));
    let registrations = context.service_worker_registrations("https://app.test/page");
    assert_eq!(registrations[0].state, ServiceWorkerState::Active);
}

#[test]
fn pages_share_service_workers_but_contexts_do_not() {
    let mut context = BrowserContext::new();
    let page = context.new_page(BrowserPage::from_html("https://app.test/", ""));
    context
        .page_mut(page)
        .unwrap()
        .service_worker_register("/", "/sw.js")
        .unwrap();

    let registrations = context.service_worker_registrations("https://app.test/");
    assert_eq!(registrations.len(), 1);
    assert!(BrowserContext::new()
        .service_worker_registrations("https://app.test/")
        .is_empty());
}
