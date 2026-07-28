//! JavaScript dispatch bridge for navigation lifecycle events.

use crate::browser_agent::page::BrowserPage;

pub(crate) enum JsNavigationEvent<'a> {
    BeforeUnload,
    Unload,
    HashChange { old_url: &'a str, new_url: &'a str },
    PopState,
}

pub(crate) fn dispatch(page: &mut BrowserPage, event: JsNavigationEvent<'_>) {
    let checkpoint = page.event_checkpoint();
    let action = action(&event).to_string();
    let state = page.session.browser_js_state();
    let Some(runtime) = page.runtime.as_mut() else {
        return;
    };
    runtime.apply_state(state);
    let result = match event {
        JsNavigationEvent::BeforeUnload => runtime.dispatch_beforeunload(),
        JsNavigationEvent::Unload => runtime.dispatch_unload(),
        JsNavigationEvent::HashChange { old_url, new_url } => {
            runtime.dispatch_hashchange(old_url, new_url)
        }
        JsNavigationEvent::PopState => runtime.dispatch_popstate(),
    };
    let _ = page.apply_runtime_result(checkpoint, &action, result);
    let _ = page.collect_agent_bridges();
    page.sync_context_state_from_session();
}

fn action(event: &JsNavigationEvent<'_>) -> &'static str {
    match event {
        JsNavigationEvent::BeforeUnload => "navigation.beforeunload",
        JsNavigationEvent::Unload => "navigation.unload",
        JsNavigationEvent::HashChange { .. } => "navigation.hashchange",
        JsNavigationEvent::PopState => "navigation.popstate",
    }
}
