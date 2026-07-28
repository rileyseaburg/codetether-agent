//! Tiny dependency-free browser JavaScript host bindings.
//!
//! This is a deterministic bridge between the in-tree HTML/CSS browser
//! primitives and the in-tree JavaScript interpreter. It intentionally uses only
//! `std` and the project's own modules. It exposes a small DOM surface:
//! `document`, `window`, `getElementById`, `querySelector`, `querySelectorAll`,
//! `textContent`, `innerText`, `innerHTML`, `children`, `setAttribute`, and
//! `getAttribute`, plus basic element creation and tree mutation APIs. Browser
//! compatibility globals also include `location`, `navigator`, deterministic
//! timers, and in-memory `localStorage`/`sessionStorage` Storage objects.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

use crate::browser::{self, Element, Node};
use crate::js::{self, JsEngine, JsValue, NativeFunction};
use crate::value::Value;

#[path = "browser_js_canvas.rs"]
mod canvas_host;
#[path = "browser_js_channels.rs"]
mod channels_host;
#[path = "browser_js_dom/class_list.rs"]
mod class_list_host;
#[path = "browser_js_compat.rs"]
mod compat_host;
#[path = "browser_js_cookies.rs"]
mod cookie_host;
#[path = "browser_js_cssom/mod.rs"]
mod cssom_host;
#[path = "browser_js_custom.rs"]
mod custom_host;
#[path = "browser_js_dom.rs"]
mod dom_compat_host;
#[path = "browser_js_event_path.rs"]
mod event_path_host;
#[path = "browser_js_fullscreen.rs"]
mod fullscreen_host;
#[path = "browser_js_inline_preflight.rs"]
mod inline_preflight;
#[path = "browser_js_lifecycle.rs"]
mod lifecycle_host;
#[path = "browser_js_dom/live_form.rs"]
mod live_form_host;
#[path = "browser_js_media/mod.rs"]
mod media_host;
#[path = "browser_js_metadata.rs"]
mod metadata_host;
#[path = "browser_js_network_cookies.rs"]
mod network_cookie_host;
#[path = "browser_js_performance.rs"]
mod performance_host;
#[path = "browser_js_realtime_model.rs"]
mod realtime_model;
#[allow(unused_imports)]
pub(crate) use realtime_model::{BrowserJsRealtimeEvent, BrowserJsRealtimeEventKind};
#[path = "browser_js_script_budget.rs"]
mod script_budget;
#[path = "browser_js_selection/mod.rs"]
mod selection_host;
#[path = "browser_js_timers.rs"]
mod timers_host;
#[path = "browser_js_trace.rs"]
mod trace_host;
#[path = "browser_js_viewport/mod.rs"]
mod viewport_host;
#[path = "browser_js_window/mod.rs"]
mod window_host;
use timers_host::TimerQueue;

const DOM_API_VERSION: &str = "tetherscript-dom-0.3";
const MAX_TIMER_DRAIN: usize = 10_000;

thread_local! {
    static EVENT_REGISTRY: RefCell<HashMap<String, EventEntry>> = RefCell::new(HashMap::new());
    static DOM_HANDLE_REGISTRY: RefCell<HashMap<String, DomHandle>> = RefCell::new(HashMap::new());
    static NEXT_DOM_HANDLE_ID: RefCell<u64> = const { RefCell::new(1) };
    static FOCUSED_ELEMENT: RefCell<Option<String>> = const { RefCell::new(None) };
    static INPUT_SELECTIONS: RefCell<HashMap<String, (usize, usize)>> = RefCell::new(HashMap::new());
    static NETWORK_EVENTS: RefCell<Vec<BrowserJsNetworkEvent>> = const { RefCell::new(Vec::new()) };
    static LAYOUT_CSS: RefCell<String> = const { RefCell::new(String::new()) };
    static MUTATION_OBSERVERS: RefCell<HashMap<u64, MutationObserverState>> = RefCell::new(HashMap::new());
    static NEXT_MUTATION_OBSERVER_ID: RefCell<u64> = const { RefCell::new(1) };
    static OBJECT_URLS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static NEXT_OBJECT_URL_ID: RefCell<u64> = const { RefCell::new(1) };
}

#[derive(Clone, Copy)]
enum InsertPosition {
    Append,
    Prepend,
}

// NOTE(riley): Index-path-based handles can shift when siblings are prepended/removed.
// A future iteration should give each node a stable ID or store Rc<RefCell<Node>> per node
// so that existing handles remain valid after mutations.
#[derive(Clone)]
struct DomHandle {
    root: Rc<RefCell<Node>>,
    path: Vec<usize>,
}

#[derive(Clone, Default)]
struct EventEntry {
    listeners: HashMap<String, Vec<RegisteredListener>>,
    handlers: HashMap<String, JsValue>,
}

#[derive(Clone)]
struct RegisteredListener {
    callback: JsValue,
    capture: bool,
    once: bool,
}

#[derive(Clone)]
struct ScheduledCallback {
    id: u32,
    callback: JsValue,
    args: Vec<JsValue>,
    this_value: JsValue,
}

#[derive(Default)]
struct StorageArea {
    entries: Vec<(String, String)>,
}

struct BrowserRuntime {
    window: JsValue,
    local_storage: Rc<RefCell<StorageArea>>,
    session_storage: Rc<RefCell<StorageArea>>,
    realtime: Rc<RefCell<RealtimeHost>>,
}

struct RealtimeHost {
    next_id: u64,
    next_sequence: u64,
    connections: Vec<RealtimeConnectionHandle>,
    outbound: Vec<BrowserJsRealtimeOutbound>,
    events: Vec<realtime_model::BrowserJsRealtimeEvent>,
}

struct RealtimeConnectionHandle {
    id: u64,
    kind: BrowserJsRealtimeKind,
    url: String,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
}

impl Default for RealtimeHost {
    fn default() -> Self {
        Self {
            next_id: 1,
            next_sequence: 0,
            connections: Vec::new(),
            outbound: Vec::new(),
            events: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct MutationObserverState {
    callback: JsValue,
    targets: Vec<MutationObserverTarget>,
    records: Vec<JsValue>,
    connected: bool,
}

#[derive(Clone)]
struct MutationObserverTarget {
    handle: DomHandle,
    options: MutationObserverOptions,
}

#[derive(Clone, Copy)]
struct MutationObserverOptions {
    child_list: bool,
    attributes: bool,
    character_data: bool,
    subtree: bool,
    attribute_old_value: bool,
    character_data_old_value: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BrowserJsState {
    pub url: String,
    pub cookies: Vec<(String, String)>,
    pub cookie_jar: Vec<crate::browser_cookie::Cookie>,
    pub set_cookies: Vec<String>,
    pub local_storage: Vec<(String, String)>,
    pub session_storage: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserJsNetworkEvent {
    pub method: String,
    pub url: String,
    pub status: Option<u16>,
    pub route_result: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BrowserJsRealtimeKind {
    WebSocket,
    EventSource,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserJsRealtimeConnection {
    pub(crate) id: u64,
    pub(crate) kind: BrowserJsRealtimeKind,
    pub(crate) url: String,
    pub(crate) ready_state: i64,
    pub(crate) retry_ms: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserJsRealtimeOutbound {
    pub(crate) connection_id: u64,
    pub(crate) url: String,
    pub(crate) data: String,
}

pub struct BrowserJsResult {
    pub document: browser::Document,
    pub value: JsValue,
    pub console: Vec<String>,
    pub css: String,
    pub viewport_width: i64,
    pub html: String,
    pub state: BrowserJsState,
    pub network: Vec<BrowserJsNetworkEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserJsRouteRequest {
    pub(crate) method: String,
    pub(crate) url: String,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BrowserJsRouteFulfillment {
    pub(crate) status: u16,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum BrowserJsRouteAction {
    PassThrough,
    Continue,
    Abort(String),
    Blocked(String),
    Fulfill(BrowserJsRouteFulfillment),
}

pub(crate) type BrowserJsRouteHandler =
    Rc<RefCell<dyn FnMut(BrowserJsRouteRequest) -> BrowserJsRouteAction>>;

type SharedBrowserJsRouteHandler = Rc<RefCell<Option<BrowserJsRouteHandler>>>;
type JsObjectRef = Rc<RefCell<HashMap<String, JsValue>>>;
type OptionalJsObjectRef = Rc<RefCell<Option<JsObjectRef>>>;
type RealtimeObjectLookup = Option<(String, JsObjectRef)>;
type StorageEventWindow = Rc<RefCell<Option<JsValue>>>;

struct DomGlobalInstall {
    css: String,
    local_storage_entries: Vec<(String, String)>,
    session_storage_entries: Vec<(String, String)>,
    url: String,
    route_handler: SharedBrowserJsRouteHandler,
}

/// Persistent JavaScript runtime for one deterministic browser page.
///
/// This keeps the JS heap, DOM handles, timers, event listeners, and storage
/// objects alive across evaluations. It is intended for agent page actions that
/// need browser-like continuity after inline scripts register listeners.
pub struct BrowserJsRuntime {
    engine: JsEngine,
    root: Rc<RefCell<Node>>,
    timers: Rc<RefCell<TimerQueue>>,
    runtime: BrowserRuntime,
    console_offset: usize,
    network_offset: usize,
    scripts_ran: bool,
    route_handler: SharedBrowserJsRouteHandler,
}

impl BrowserJsRuntime {
    /// Create a persistent runtime from HTML and an initial browser state.
    pub fn new(html: &str, state: BrowserJsState) -> Result<Self, String> {
        inline_preflight::reject(html)?;
        reset_browser_js_state();
        seed_browser_js_state(&state);
        let root = html_to_root(html);
        let mut engine = JsEngine::new();
        let timers = Rc::new(RefCell::new(TimerQueue::default()));
        let route_handler = Rc::new(RefCell::new(None));
        let css = browser::extract_embedded_css(&root_to_document(&root));
        let runtime = install_dom_globals(
            &mut engine,
            root.clone(),
            timers.clone(),
            DomGlobalInstall {
                css: css.clone(),
                local_storage_entries: state.local_storage,
                session_storage_entries: state.session_storage,
                url: state.url.clone(),
                route_handler: route_handler.clone(),
            },
        )?;
        Ok(Self {
            engine,
            root,
            timers,
            runtime,
            console_offset: 0,
            network_offset: 0,
            scripts_ran: false,
            route_handler,
        })
    }

    /// Return the current serialized page HTML.
    pub fn html(&self) -> String {
        inner_html(&self.root.borrow())
    }

    /// Apply host-side cookie and storage state without replacing the JS heap.
    pub fn apply_state(&mut self, state: BrowserJsState) {
        seed_browser_js_state(&state);
        self.runtime.local_storage.borrow_mut().entries = state.local_storage;
        self.runtime.session_storage.borrow_mut().entries = state.session_storage;
        set_runtime_location(&self.runtime.window, &state.url);
    }

    /// Install or clear the host request route handler used by fetch and XHR.
    pub(crate) fn set_route_handler(&mut self, handler: Option<BrowserJsRouteHandler>) {
        *self.route_handler.borrow_mut() = handler;
    }

    pub(crate) fn realtime_connections(&self) -> Vec<BrowserJsRealtimeConnection> {
        realtime_connections(&self.runtime.realtime)
    }

    pub(crate) fn realtime_outbound(&self) -> Vec<BrowserJsRealtimeOutbound> {
        self.runtime.realtime.borrow().outbound.clone()
    }

    pub(crate) fn realtime_events(&self) -> Vec<BrowserJsRealtimeEvent> {
        self.runtime.realtime.borrow().events.clone()
    }

    pub(crate) fn inject_websocket_message(
        &mut self,
        connection_id: u64,
        data: &str,
    ) -> Result<BrowserJsResult, String> {
        inject_realtime_message(
            &self.runtime.realtime,
            BrowserJsRealtimeKind::WebSocket,
            connection_id,
            data,
        )?;
        self.settle(JsValue::Undefined)
    }

    pub(crate) fn fail_websocket_connection(
        &mut self,
        connection_id: u64,
        reason: &str,
    ) -> Result<BrowserJsResult, String> {
        fail_realtime_connection(
            &self.runtime.realtime,
            BrowserJsRealtimeKind::WebSocket,
            connection_id,
            reason,
            None,
        )?;
        self.settle(JsValue::Undefined)
    }

    pub(crate) fn fail_event_source_connection(
        &mut self,
        connection_id: u64,
        reason: &str,
        retry_ms: Option<u64>,
    ) -> Result<BrowserJsResult, String> {
        fail_realtime_connection(
            &self.runtime.realtime,
            BrowserJsRealtimeKind::EventSource,
            connection_id,
            reason,
            retry_ms,
        )?;
        self.settle(JsValue::Undefined)
    }

    pub(crate) fn inject_event_source_message(
        &mut self,
        connection_id: u64,
        data: &str,
    ) -> Result<BrowserJsResult, String> {
        inject_realtime_message(
            &self.runtime.realtime,
            BrowserJsRealtimeKind::EventSource,
            connection_id,
            data,
        )?;
        self.settle(JsValue::Undefined)
    }

    /// Execute inline scripts once, then deliver load lifecycle events.
    pub fn run_scripts(&mut self) -> Result<BrowserJsResult, String> {
        if !self.scripts_ran {
            let scripts = collect_inline_scripts(&self.root.borrow());
            let mut last = JsValue::Undefined;
            for source in scripts {
                if !source.trim().is_empty() {
                    last = self.eval_script(&source)?;
                    self.drain_microtasks()?;
                }
            }
            self.drain_microtasks()?;
            dispatch_document_lifecycle(&self.root, "DOMContentLoaded")?;
            self.drain_microtasks()?;
            dispatch_window_lifecycle(&self.runtime.window, "load")?;
            self.scripts_ran = true;
            return self.settle(last);
        }
        self.settle(JsValue::Undefined)
    }

    /// Evaluate JavaScript inside the persistent page context.
    pub fn eval(&mut self, script: &str) -> Result<BrowserJsResult, String> {
        let value = self.eval_script(script)?;
        self.settle(value)
    }

    fn eval_script(&mut self, script: &str) -> Result<JsValue, String> {
        eval_browser_script_with_drain(
            &mut self.engine,
            script,
            self.timers.clone(),
            self.runtime.window.clone(),
        )
    }

    fn settle(&mut self, value: JsValue) -> Result<BrowserJsResult, String> {
        self.drain_microtasks()?;
        drain_timers(self.timers.clone(), self.runtime.window.clone())?;
        Ok(self.snapshot(value))
    }

    fn drain_microtasks(&self) -> Result<(), String> {
        drain_microtasks(self.timers.clone(), self.runtime.window.clone())
    }

    fn snapshot(&mut self, value: JsValue) -> BrowserJsResult {
        let console = self.engine.console_output();
        let console_delta = console
            .get(self.console_offset..)
            .map(|items| items.to_vec())
            .unwrap_or_default();
        self.console_offset = console.len();
        let network = NETWORK_EVENTS.with(|events| events.borrow().clone());
        let network_delta = network
            .get(self.network_offset..)
            .map(|items| items.to_vec())
            .unwrap_or_default();
        self.network_offset = network.len();
        browser_js_result_with_network(
            self.root.clone(),
            &self.runtime,
            value,
            console_delta,
            network_delta,
        )
    }
}

pub fn run_html_scripts(html: &str) -> Result<BrowserJsResult, String> {
    run_html_scripts_with_state(html, BrowserJsState::default())
}

pub fn run_html_scripts_at_url(html: &str, url: &str) -> Result<BrowserJsResult, String> {
    run_html_scripts_with_state(
        html,
        BrowserJsState {
            url: url.into(),
            ..BrowserJsState::default()
        },
    )
}

pub fn run_html_scripts_with_state(
    html: &str,
    state: BrowserJsState,
) -> Result<BrowserJsResult, String> {
    trace_host::mark("run_html_scripts start");
    inline_preflight::reject(html)?;
    trace_host::mark("inline preflight complete");
    reset_browser_js_state();
    seed_browser_js_state(&state);
    let root = html_to_root(html);
    trace_host::mark("html parsed");
    trace_host::mark("js engine init start");
    let mut engine = JsEngine::new();
    trace_host::mark("js engine init complete");
    let timers = Rc::new(RefCell::new(TimerQueue::default()));
    trace_host::mark("embedded css extraction start");
    let css = browser::extract_embedded_css(&root_to_document(&root));
    trace_host::mark("embedded css extraction complete");
    trace_host::mark("dom globals install start");
    let runtime = install_dom_globals(
        &mut engine,
        root.clone(),
        timers.clone(),
        DomGlobalInstall {
            css: css.clone(),
            local_storage_entries: state.local_storage,
            session_storage_entries: state.session_storage,
            url: state.url.clone(),
            route_handler: Rc::new(RefCell::new(None)),
        },
    )?;
    trace_host::mark("dom globals installed");
    let scripts = collect_inline_scripts(&root.borrow());
    let mut last = JsValue::Undefined;
    for source in scripts {
        if !source.trim().is_empty() {
            trace_host::mark("inline script eval start");
            last = eval_browser_script_with_drain(
                &mut engine,
                &source,
                timers.clone(),
                runtime.window.clone(),
            )?;
            trace_host::mark("inline script eval complete");
            drain_microtasks(timers.clone(), runtime.window.clone())?;
        }
    }
    drain_microtasks(timers.clone(), runtime.window.clone())?;
    dispatch_document_lifecycle(&root, "DOMContentLoaded")?;
    drain_microtasks(timers.clone(), runtime.window.clone())?;
    dispatch_window_lifecycle(&runtime.window, "load")?;
    drain_microtasks(timers.clone(), runtime.window.clone())?;
    drain_timers(timers, runtime.window.clone())?;
    Ok(browser_js_result(
        root,
        &runtime,
        last,
        engine.console_output(),
    ))
}

pub fn eval_with_dom(html: &str, script: &str) -> Result<BrowserJsResult, String> {
    eval_with_dom_state(html, script, BrowserJsState::default())
}

pub fn eval_with_dom_state(
    html: &str,
    script: &str,
    state: BrowserJsState,
) -> Result<BrowserJsResult, String> {
    trace_host::mark("eval_with_dom start");
    reset_browser_js_state();
    seed_browser_js_state(&state);
    let root = html_to_root(html);
    let mut engine = JsEngine::new();
    let timers = Rc::new(RefCell::new(TimerQueue::default()));
    let css = browser::extract_embedded_css(&root_to_document(&root));
    let runtime = install_dom_globals(
        &mut engine,
        root.clone(),
        timers.clone(),
        DomGlobalInstall {
            css: css.clone(),
            local_storage_entries: state.local_storage,
            session_storage_entries: state.session_storage,
            url: state.url.clone(),
            route_handler: Rc::new(RefCell::new(None)),
        },
    )?;
    trace_host::mark("dom globals installed");
    let value = eval_browser_script_with_drain(
        &mut engine,
        script,
        timers.clone(),
        runtime.window.clone(),
    )?;
    trace_host::mark("script eval complete");
    drain_microtasks(timers.clone(), runtime.window.clone())?;
    dispatch_document_lifecycle(&root, "DOMContentLoaded")?;
    drain_microtasks(timers.clone(), runtime.window.clone())?;
    dispatch_window_lifecycle(&runtime.window, "load")?;
    drain_microtasks(timers.clone(), runtime.window.clone())?;
    drain_timers(timers, runtime.window.clone())?;
    Ok(browser_js_result(
        root,
        &runtime,
        value,
        engine.console_output(),
    ))
}

fn reset_browser_js_state() {
    EVENT_REGISTRY.with(|r| r.borrow_mut().clear());
    DOM_HANDLE_REGISTRY.with(|r| r.borrow_mut().clear());
    NEXT_DOM_HANDLE_ID.with(|id| *id.borrow_mut() = 1);
    FOCUSED_ELEMENT.with(|focused| *focused.borrow_mut() = None);
    INPUT_SELECTIONS.with(|selections| selections.borrow_mut().clear());
    cookie_host::reset();
    network_cookie_host::reset();
    performance_host::reset();
    NETWORK_EVENTS.with(|events| events.borrow_mut().clear());
    canvas_host::reset_all();
    channels_host::reset_all();
    custom_host::reset_all();
    fullscreen_host::reset();
    media_host::reset_all();
    selection_host::reset();
    dom_compat_host::form_validation::reset();
    compat_host::promise::reset();
    MUTATION_OBSERVERS.with(|observers| observers.borrow_mut().clear());
    NEXT_MUTATION_OBSERVER_ID.with(|id| *id.borrow_mut() = 1);
    OBJECT_URLS.with(|urls| urls.borrow_mut().clear());
    NEXT_OBJECT_URL_ID.with(|id| *id.borrow_mut() = 1);
}

fn eval_browser_script(engine: &mut JsEngine, source: &str) -> Result<JsValue, String> {
    engine.eval_with_budget(source, script_budget::for_source(source))
}

fn eval_browser_script_with_drain(
    engine: &mut JsEngine,
    source: &str,
    timers: Rc<RefCell<TimerQueue>>,
    window: JsValue,
) -> Result<JsValue, String> {
    let hook_timers = timers.clone();
    let hook_window = window.clone();
    js::with_await_drain(
        Rc::new(move || {
            drain_microtasks_count(hook_timers.clone(), hook_window.clone())
                .map(|drained| drained > 0)
        }),
        || eval_browser_script(engine, source),
    )
}

fn seed_browser_js_state(state: &BrowserJsState) {
    cookie_host::seed(state.cookies.clone());
    network_cookie_host::seed(state.cookie_jar.clone(), state.url.clone());
}

fn set_runtime_location(window: &JsValue, url: &str) {
    if url.is_empty() {
        return;
    }
    let JsValue::Object(window) = window else {
        return;
    };
    let Some(JsValue::Object(location)) = window.borrow().get("location").cloned() else {
        return;
    };
    set_location_href(&location, url);
}

fn browser_js_result(
    root: Rc<RefCell<Node>>,
    runtime: &BrowserRuntime,
    value: JsValue,
    console: Vec<String>,
) -> BrowserJsResult {
    browser_js_result_with_network(
        root,
        runtime,
        value,
        console,
        NETWORK_EVENTS.with(|events| events.borrow().clone()),
    )
}

fn browser_js_result_with_network(
    root: Rc<RefCell<Node>>,
    runtime: &BrowserRuntime,
    value: JsValue,
    console: Vec<String>,
    network: Vec<BrowserJsNetworkEvent>,
) -> BrowserJsResult {
    let document = root_to_document(&root);
    let html = inner_html(&root.borrow());
    let css = LAYOUT_CSS.with(|source| source.borrow().clone());
    BrowserJsResult {
        document,
        value,
        console,
        css,
        viewport_width: 80,
        html,
        state: BrowserJsState {
            url: window_location_href(&runtime.window),
            cookies: cookie_host::visible_pairs(),
            cookie_jar: network_cookie_host::jar(),
            set_cookies: cookie_host::mutations(),
            local_storage: runtime.local_storage.borrow().entries.clone(),
            session_storage: runtime.session_storage.borrow().entries.clone(),
        },
        network,
    }
}

fn html_to_root(html: &str) -> Rc<RefCell<Node>> {
    let document = browser::parse_html(html);
    Rc::new(RefCell::new(Node::Element(Element {
        tag: "#document".into(),
        attrs: HashMap::new(),
        children: document.children,
    })))
}

fn install_dom_globals(
    engine: &mut JsEngine,
    root: Rc<RefCell<Node>>,
    timers: Rc<RefCell<TimerQueue>>,
    install: DomGlobalInstall,
) -> Result<BrowserRuntime, String> {
    let DomGlobalInstall {
        css,
        local_storage_entries,
        session_storage_entries,
        url,
        route_handler,
    } = install;
    // Store CSS in thread-local for layout computations
    LAYOUT_CSS.with(|c| *c.borrow_mut() = css.clone());

    trace_host::mark("document object install start");
    let document = node_object(DomHandle {
        root: root.clone(),
        path: Vec::new(),
    });
    trace_host::mark("document object install complete");
    fullscreen_host::register_document(&document);
    trace_host::mark("cssom document install start");
    cssom_host::install_document(&document, &root_to_document(&root), css.clone());
    trace_host::mark("cssom document install complete");
    trace_host::mark("viewport document install start");
    viewport_host::install_document(&document, root.clone());
    trace_host::mark("viewport document install complete");
    engine.set_global("document", document.clone());
    let mut window = HashMap::new();
    window.insert("document".into(), document.clone());
    let location_url = if url.is_empty() {
        "http://localhost/"
    } else {
        &url
    };
    let location_map = Rc::new(RefCell::new(parse_location(location_url)));
    install_location_methods(&location_map);
    let location = JsValue::Object(location_map.clone());
    let navigator = navigator_object();
    let storage_event_window = Rc::new(RefCell::new(None));
    let (local_storage, local_storage_area) = storage_object(
        "localStorage",
        local_storage_entries,
        Some(storage_event_window.clone()),
    );
    // sessionStorage is scoped to this single page context, so this host skips
    // cross-document storage events for it.
    let (session_storage, session_storage_area) =
        storage_object("sessionStorage", session_storage_entries, None);
    let cookie_store = cookie_host::store_object(&document);
    let history = history_object(location_map.clone());
    window.insert("location".into(), location.clone());
    window.insert("history".into(), history.clone());
    window.insert("navigator".into(), navigator.clone());
    window.insert("cookieStore".into(), cookie_store.clone());
    window.insert("localStorage".into(), local_storage.clone());
    window.insert("sessionStorage".into(), session_storage.clone());
    selection_host::install_window(
        &mut window,
        &DomHandle {
            root: root.clone(),
            path: Vec::new(),
        },
    );
    install_timer_bindings(&mut window, timers.clone());
    let realtime = Rc::new(RefCell::new(RealtimeHost::default()));
    let style_root = root.clone();
    window.insert(
        "getComputedStyle".into(),
        native("getComputedStyle", Some(1), move |args| {
            let path = dom_path_from_value(args.first().unwrap_or(&JsValue::Undefined));
            Ok(computed_style_object(&DomHandle {
                root: style_root.clone(),
                path,
            }))
        }),
    );
    let rect_root = root.clone();
    window.insert(
        "getBoundingClientRect".into(),
        native("getBoundingClientRect", Some(1), move |args| {
            let path = dom_path_from_value(args.first().unwrap_or(&JsValue::Undefined));
            Ok(rect_object(&element_rect(&DomHandle {
                root: rect_root.clone(),
                path,
            })))
        }),
    );
    let a11y_root = root.clone();
    window.insert(
        "getAccessibilityTree".into(),
        native("getAccessibilityTree", Some(0), move |_| {
            Ok(accessibility_tree_object(&root_to_document(&a11y_root)))
        }),
    );
    install_window_event_bindings(&mut window);
    window_host::install_event_handlers(&mut window);
    performance_host::install(&mut window, timers.clone());
    install_fetch_binding(
        &mut window,
        timers.clone(),
        route_handler.clone(),
        location_map.clone(),
    );
    install_xml_http_request(
        &mut window,
        timers.clone(),
        route_handler.clone(),
        location_map.clone(),
    );
    install_realtime_bindings(&mut window, timers.clone(), realtime.clone());
    install_mutation_observer(&mut window);
    install_intersection_observer(&mut window, root.clone());
    install_resize_observer(&mut window);
    install_web_api_bindings(&mut window);
    cssom_host::install_window(&mut window);
    viewport_host::install_window(&mut window);
    dom_compat_host::install_window(&mut window);
    channels_host::install(&mut window, timers.clone());
    compat_host::install(&mut window);
    custom_host::install(&mut window, root.clone());
    metadata_host::install(
        &mut window,
        &document,
        &navigator,
        location_map,
        route_handler.clone(),
    );
    let window = JsValue::Object(Rc::new(RefCell::new(window)));
    *storage_event_window.borrow_mut() = Some(window.clone());
    window_host::install_dispatchers(&window);
    if let JsValue::Object(obj) = &window {
        obj.borrow_mut().insert("window".into(), window.clone());
        obj.borrow_mut().insert("self".into(), window.clone());
        obj.borrow_mut().insert("globalThis".into(), window.clone());
        obj.borrow_mut().insert("top".into(), window.clone());
        obj.borrow_mut().insert("parent".into(), window.clone());
        obj.borrow_mut().insert("frames".into(), window.clone());
        obj.borrow_mut().insert("opener".into(), JsValue::Null);
        obj.borrow_mut()
            .insert("closed".into(), JsValue::Bool(false));
        obj.borrow_mut()
            .insert("name".into(), JsValue::String(String::new()));
        obj.borrow_mut()
            .insert("length".into(), JsValue::Number(0.0));
        let borrowed = obj.borrow();
        if let Some(set_timeout) = borrowed.get("setTimeout").cloned() {
            engine.set_global("setTimeout", set_timeout);
        }
        if let Some(clear_timeout) = borrowed.get("clearTimeout").cloned() {
            engine.set_global("clearTimeout", clear_timeout);
        }
        if let Some(set_interval) = borrowed.get("setInterval").cloned() {
            engine.set_global("setInterval", set_interval);
        }
        if let Some(clear_interval) = borrowed.get("clearInterval").cloned() {
            engine.set_global("clearInterval", clear_interval);
        }
        if let Some(queue_microtask) = borrowed.get("queueMicrotask").cloned() {
            engine.set_global("queueMicrotask", queue_microtask);
        }
        if let Some(request_animation_frame) = borrowed.get("requestAnimationFrame").cloned() {
            engine.set_global("requestAnimationFrame", request_animation_frame);
        }
        if let Some(cancel_animation_frame) = borrowed.get("cancelAnimationFrame").cloned() {
            engine.set_global("cancelAnimationFrame", cancel_animation_frame);
        }
    }
    if let JsValue::Object(obj) = &window {
        if let JsValue::Object(document_obj) = &document {
            document_obj
                .borrow_mut()
                .insert("defaultView".into(), window.clone());
        }
        for name in [
            "getComputedStyle",
            "getBoundingClientRect",
            "getAccessibilityTree",
            "fetch",
            "XMLHttpRequest",
            "WebSocket",
            "EventSource",
            "Event",
            "CustomEvent",
            "MouseEvent",
            "KeyboardEvent",
            "InputEvent",
            "SubmitEvent",
            "FocusEvent",
            "PointerEvent",
            "WheelEvent",
            "MutationObserver",
            "IntersectionObserver",
            "ResizeObserver",
            "ReportingObserver",
            "DOMMatrix",
            "DOMMatrixReadOnly",
            "URL",
            "URLPattern",
            "URLSearchParams",
            "AbortController",
            "Request",
            "Response",
            "Headers",
            "atob",
            "btoa",
            "Uint8Array",
            "Uint32Array",
            "Uint16Array",
            "Int32Array",
            "Float32Array",
            "ArrayBuffer",
            "SharedArrayBuffer",
            "TextEncoder",
            "TextDecoder",
            "Blob",
            "File",
            "FormData",
            "FileReader",
            "structuredClone",
            "crypto",
            "trustedTypes",
            "CSS",
            "CSSStyleSheet",
            "performance",
            "scheduler",
            "requestIdleCallback",
            "cancelIdleCallback",
            "PerformanceObserver",
            "customElements",
            "MessageChannel",
            "BroadcastChannel",
            "Worker",
            "registerWorkerScript",
            "DOMParser",
            "XMLSerializer",
            "Node",
            "Element",
            "HTMLElement",
            "Document",
            "DocumentFragment",
            "NodeFilter",
            "matchMedia",
            "screen",
            "innerWidth",
            "innerHeight",
            "devicePixelRatio",
            "scrollX",
            "scrollY",
            "pageXOffset",
            "pageYOffset",
        ] {
            if let Some(value) = obj.borrow().get(name).cloned() {
                engine.set_global(name, value);
            }
        }
    }
    engine.set_global("window", window.clone());
    engine.set_global("self", window.clone());
    engine.set_global("globalThis", window.clone());
    engine.set_global("top", window.clone());
    engine.set_global("parent", window.clone());
    engine.set_global("frames", window.clone());
    engine.set_global("opener", JsValue::Null);
    engine.set_global("closed", JsValue::Bool(false));
    engine.set_global("name", JsValue::String(String::new()));
    engine.set_global("length", JsValue::Number(0.0));
    engine.set_global("location", location);
    engine.set_global("history", history);
    engine.set_global("navigator", navigator);
    engine.set_global("cookieStore", cookie_store);
    engine.set_global("localStorage", local_storage);
    engine.set_global("sessionStorage", session_storage);
    window_host::bootstrap(engine)?;
    Ok(BrowserRuntime {
        window,
        local_storage: local_storage_area,
        session_storage: session_storage_area,
        realtime,
    })
}

fn install_timer_bindings(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    let set_queue = timers.clone();
    window.insert(
        "setTimeout".into(),
        native("setTimeout", None, move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            let callback_args = if args.len() > 2 {
                args[2..].to_vec()
            } else {
                Vec::new()
            };
            let mut queue = set_queue.borrow_mut();
            queue.next_id = queue.next_id.saturating_add(1).max(1);
            let id = queue.next_id;
            queue.schedule_timeout(
                timer_delay(args.get(1)),
                ScheduledCallback {
                    id,
                    callback,
                    args: callback_args,
                    this_value: JsValue::Undefined,
                },
            );
            Ok(JsValue::Number(id as f64))
        }),
    );

    let clear_queue = timers.clone();
    window.insert(
        "clearTimeout".into(),
        native("clearTimeout", None, move |args| {
            let id = args.first().map(timer_id).unwrap_or(0);
            clear_queue.borrow_mut().cancel_timer(id);
            Ok(JsValue::Undefined)
        }),
    );

    let interval_queue = timers.clone();
    window.insert(
        "setInterval".into(),
        native("setInterval", None, move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            let callback_args = if args.len() > 2 {
                args[2..].to_vec()
            } else {
                Vec::new()
            };
            let mut queue = interval_queue.borrow_mut();
            queue.next_id = queue.next_id.saturating_add(1).max(1);
            let id = queue.next_id;
            queue.interval_ids.insert(id);
            queue.schedule_timeout(
                timer_delay(args.get(1)),
                ScheduledCallback {
                    id,
                    callback,
                    args: callback_args,
                    this_value: JsValue::Undefined,
                },
            );
            Ok(JsValue::Number(id as f64))
        }),
    );

    let clear_interval_queue = timers.clone();
    window.insert(
        "clearInterval".into(),
        native("clearInterval", None, move |args| {
            let id = args.first().map(timer_id).unwrap_or(0);
            clear_interval_queue.borrow_mut().cancel_timer(id);
            Ok(JsValue::Undefined)
        }),
    );

    let microtask_queue = timers.clone();
    window.insert(
        "queueMicrotask".into(),
        native("queueMicrotask", Some(1), move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            microtask_queue
                .borrow_mut()
                .microtasks
                .push_back(ScheduledCallback {
                    id: 0,
                    callback,
                    args: Vec::new(),
                    this_value: JsValue::Undefined,
                });
            Ok(JsValue::Undefined)
        }),
    );

    let raf_queue = timers.clone();
    window.insert(
        "requestAnimationFrame".into(),
        native("requestAnimationFrame", Some(1), move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            let mut queue = raf_queue.borrow_mut();
            queue.next_id = queue.next_id.saturating_add(1).max(1);
            let id = queue.next_id;
            queue.animation_frames.push_back(ScheduledCallback {
                id,
                callback,
                args: vec![JsValue::Number(0.0)],
                this_value: JsValue::Undefined,
            });
            Ok(JsValue::Number(id as f64))
        }),
    );

    let cancel_raf_queue = timers;
    window.insert(
        "cancelAnimationFrame".into(),
        native("cancelAnimationFrame", Some(1), move |args| {
            let id = args.first().map(timer_id).unwrap_or(0);
            cancel_raf_queue
                .borrow_mut()
                .animation_frames
                .retain(|task| task.id != id);
            Ok(JsValue::Undefined)
        }),
    );
}

fn drain_timers(timers: Rc<RefCell<TimerQueue>>, window: JsValue) -> Result<(), String> {
    let mut drained = 0;
    loop {
        drain_microtasks(timers.clone(), window.clone())?;
        let frame_count = timers.borrow().animation_frames.len();
        for _ in 0..frame_count {
            let task = { timers.borrow_mut().animation_frames.pop_front() };
            let Some(task) = task else {
                break;
            };
            drained += 1;
            if drained > MAX_TIMER_DRAIN {
                return Err(format!(
                    "requestAnimationFrame: exceeded deterministic drain limit of {} callbacks",
                    MAX_TIMER_DRAIN
                ));
            }
            js::call_function_with_this(task.callback, task.this_value, &task.args)?;
            drain_microtasks(timers.clone(), window.clone())?;
        }

        let task = { timers.borrow_mut().pop_timer() };
        let Some(task) = task else {
            if performance_host::drain_idle_callbacks(timers.clone(), window.clone(), &mut drained)?
            {
                continue;
            }
            break;
        };
        let (task, delay_ms) = task;
        drained += 1;
        let is_interval = { timers.borrow().interval_ids.contains(&task.id) };
        if drained > MAX_TIMER_DRAIN {
            let timer_name = if is_interval {
                "setInterval"
            } else {
                "setTimeout"
            };
            return Err(format!(
                "{}: exceeded deterministic drain limit of {} callbacks",
                timer_name, MAX_TIMER_DRAIN
            ));
        }
        js::call_function_with_this(task.callback.clone(), window.clone(), &task.args)?;
        drain_microtasks(timers.clone(), window.clone())?;
        if is_interval && timers.borrow().interval_ids.contains(&task.id) {
            timers.borrow_mut().reschedule_interval(task, delay_ms);
        }
    }
    Ok(())
}

fn drain_microtasks(timers: Rc<RefCell<TimerQueue>>, window: JsValue) -> Result<(), String> {
    drain_microtasks_count(timers, window).map(|_| ())
}

fn drain_microtasks_count(
    timers: Rc<RefCell<TimerQueue>>,
    _window: JsValue,
) -> Result<usize, String> {
    let mut drained = 0;
    loop {
        loop {
            let task = { timers.borrow_mut().microtasks.pop_front() };
            let Some(task) = task else {
                break;
            };
            drained += 1;
            if drained > MAX_TIMER_DRAIN {
                return Err(format!(
                    "queueMicrotask: exceeded deterministic drain limit of {} callbacks",
                    MAX_TIMER_DRAIN
                ));
            }
            js::call_function_with_this(task.callback, task.this_value, &task.args)?;
        }
        let delivered = deliver_mutation_observers()?;
        drained += delivered;
        if drained > MAX_TIMER_DRAIN {
            return Err(format!(
                "MutationObserver: exceeded deterministic drain limit of {} callbacks",
                MAX_TIMER_DRAIN
            ));
        }
        if delivered == 0 && timers.borrow().microtasks.is_empty() {
            break;
        }
    }
    Ok(drained)
}

fn timer_id(value: &JsValue) -> u32 {
    match value {
        JsValue::Number(n) if n.is_finite() && *n > 0.0 => *n as u32,
        other => other.display().parse().unwrap_or(0),
    }
}

fn timer_delay(value: Option<&JsValue>) -> u64 {
    let delay = match value {
        Some(JsValue::Number(n)) => *n,
        Some(JsValue::String(s)) => s.trim().parse().unwrap_or(0.0),
        Some(JsValue::Bool(true)) => 1.0,
        Some(JsValue::Bool(false) | JsValue::Null | JsValue::Undefined) | None => 0.0,
        Some(_) => 0.0,
    };
    if delay.is_finite() && delay > 0.0 {
        delay.trunc() as u64
    } else {
        0
    }
}

fn install_window_event_bindings(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "addEventListener".into(),
        native("window.addEventListener", None, move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let (capture, once) = event_listener_options(args.get(2));
            EVENT_REGISTRY.with(|registry| {
                registry
                    .borrow_mut()
                    .entry("window".into())
                    .or_default()
                    .listeners
                    .entry(event_type)
                    .or_default()
                    .push(RegisteredListener {
                        callback: listener,
                        capture,
                        once,
                    });
            });
            Ok(JsValue::Undefined)
        }),
    );
    window.insert(
        "removeEventListener".into(),
        native("window.removeEventListener", None, move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let (capture, _) = event_listener_options(args.get(2));
            EVENT_REGISTRY.with(|registry| {
                if let Some(entry) = registry.borrow_mut().get_mut("window") {
                    if let Some(list) = entry.listeners.get_mut(&event_type) {
                        list.retain(|item| item.callback != listener || item.capture != capture);
                    }
                }
            });
            Ok(JsValue::Undefined)
        }),
    );
    window.insert(
        "dispatchEvent".into(),
        native("window.dispatchEvent", Some(1), move |args| {
            dispatch_window_event_object(args.first().cloned().unwrap_or(JsValue::Undefined))
        }),
    );
    for prop in [
        "onload",
        "onpopstate",
        "onbeforeunload",
        "onunload",
        "onhashchange",
        "onstorage",
    ] {
        window.insert(prop.into(), JsValue::Null);
        window.insert(
            format!("__set:{}", prop),
            native(&format!("set_window_{}", prop), Some(1), move |args| {
                let handler = args.first().cloned().unwrap_or(JsValue::Undefined);
                EVENT_REGISTRY.with(|registry| {
                    registry
                        .borrow_mut()
                        .entry("window".into())
                        .or_default()
                        .handlers
                        .insert(prop.into(), handler);
                });
                Ok(JsValue::Undefined)
            }),
        );
    }
}

fn dispatch_window_lifecycle(window: &JsValue, event_type: &str) -> Result<(), String> {
    dispatch_window_event_with_this(event_type, window.clone())
}

fn dispatch_window_event_object(event: JsValue) -> Result<JsValue, String> {
    let event_type = event_type(&event).unwrap_or_else(|| "event".into());
    let normalized = normalize_event(event, &event_type, JsValue::Undefined, JsValue::Undefined);
    dispatch_window_normalized(&event_type, normalized.clone(), JsValue::Undefined)?;
    Ok(JsValue::Bool(!event_flag(&normalized, "defaultPrevented")))
}

fn dispatch_window_event_with_this(event_type: &str, this_value: JsValue) -> Result<(), String> {
    let event = normalize_event(
        JsValue::String(event_type.into()),
        event_type,
        this_value.clone(),
        this_value.clone(),
    );
    dispatch_window_normalized(event_type, event, this_value)
}

fn dispatch_window_normalized(
    event_type: &str,
    event: JsValue,
    this_value: JsValue,
) -> Result<(), String> {
    let (listeners, handler) = EVENT_REGISTRY.with(|registry| {
        registry
            .borrow()
            .get("window")
            .map(|entry| {
                (
                    entry.listeners.get(event_type).cloned().unwrap_or_default(),
                    entry.handlers.get(&format!("on{}", event_type)).cloned(),
                )
            })
            .unwrap_or_default()
    });
    for listener in listeners {
        call_dom_listener(listener.callback.clone(), this_value.clone(), event.clone())?;
        if listener.once {
            remove_window_event_listener(event_type, &listener.callback, listener.capture);
        }
    }
    if let Some(handler) = handler {
        call_dom_listener(handler, this_value, event)?;
    }
    Ok(())
}

fn remove_window_event_listener(event_type: &str, listener: &JsValue, capture: bool) {
    EVENT_REGISTRY.with(|registry| {
        if let Some(entry) = registry.borrow_mut().get_mut("window") {
            if let Some(list) = entry.listeners.get_mut(event_type) {
                list.retain(|item| item.callback != *listener || item.capture != capture);
            }
        }
    });
}

fn dispatch_document_lifecycle(root: &Rc<RefCell<Node>>, event_type: &str) -> Result<(), String> {
    DomHandle {
        root: root.clone(),
        path: Vec::new(),
    }
    .dispatch_event(JsValue::String(event_type.into()))?;
    Ok(())
}

fn install_location_methods(obj: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let assign_obj = obj.clone();
    obj.borrow_mut().insert(
        "assign".into(),
        native("location.assign", Some(1), move |args| {
            let href = args.first().unwrap_or(&JsValue::Undefined).display();
            set_location_href(&assign_obj, &href);
            Ok(JsValue::Undefined)
        }),
    );
    let replace_obj = obj.clone();
    obj.borrow_mut().insert(
        "replace".into(),
        native("location.replace", Some(1), move |args| {
            let href = args.first().unwrap_or(&JsValue::Undefined).display();
            set_location_href(&replace_obj, &href);
            Ok(JsValue::Undefined)
        }),
    );
    obj.borrow_mut().insert(
        "reload".into(),
        native("location.reload", None, move |_| Ok(JsValue::Undefined)),
    );
}

fn history_object(location: Rc<RefCell<HashMap<String, JsValue>>>) -> JsValue {
    let stack = Rc::new(RefCell::new(vec![(
        location_href(&location),
        JsValue::Null,
    )]));
    let index = Rc::new(RefCell::new(0usize));
    let object = Rc::new(RefCell::new(HashMap::new()));
    object
        .borrow_mut()
        .insert("length".into(), JsValue::Number(1.0));
    object.borrow_mut().insert("state".into(), JsValue::Null);

    let push_location = location.clone();
    let push_stack = stack.clone();
    let push_index = index.clone();
    let push_object = object.clone();
    object.borrow_mut().insert(
        "pushState".into(),
        native("history.pushState", Some(2), move |args| {
            let state = history_state_arg(args)?;
            let url = args.get(2).map(JsValue::display).unwrap_or_default();
            if !url.is_empty() {
                set_location_href(&push_location, &url);
            }
            set_history_state(&push_object, &state)?;
            let mut stack = push_stack.borrow_mut();
            let mut index = push_index.borrow_mut();
            stack.truncate((*index).saturating_add(1));
            stack.push((location_href(&push_location), state));
            *index = stack.len().saturating_sub(1);
            set_history_length(&push_object, stack.len());
            Ok(JsValue::Undefined)
        }),
    );

    let replace_location = location.clone();
    let replace_stack = stack.clone();
    let replace_index = index.clone();
    let replace_object = object.clone();
    object.borrow_mut().insert(
        "replaceState".into(),
        native("history.replaceState", Some(2), move |args| {
            let state = history_state_arg(args)?;
            let url = args.get(2).map(JsValue::display).unwrap_or_default();
            if !url.is_empty() {
                set_location_href(&replace_location, &url);
            }
            set_history_state(&replace_object, &state)?;
            let index = *replace_index.borrow();
            if let Some(entry) = replace_stack.borrow_mut().get_mut(index) {
                *entry = (location_href(&replace_location), state);
            }
            Ok(JsValue::Undefined)
        }),
    );

    let back_location = location.clone();
    let back_stack = stack.clone();
    let back_index = index.clone();
    let back_object = object.clone();
    object.borrow_mut().insert(
        "back".into(),
        native("history.back", Some(0), move |_| {
            history_traverse(&back_location, &back_stack, &back_index, &back_object, -1)?;
            Ok(JsValue::Undefined)
        }),
    );

    let forward_location = location.clone();
    let forward_stack = stack.clone();
    let forward_index = index.clone();
    let forward_object = object.clone();
    object.borrow_mut().insert(
        "forward".into(),
        native("history.forward", Some(0), move |_| {
            history_traverse(
                &forward_location,
                &forward_stack,
                &forward_index,
                &forward_object,
                1,
            )?;
            Ok(JsValue::Undefined)
        }),
    );

    let go_location = location.clone();
    let go_stack = stack.clone();
    let go_index = index.clone();
    let go_object = object.clone();
    object.borrow_mut().insert(
        "go".into(),
        native("history.go", None, move |args| {
            history_traverse(
                &go_location,
                &go_stack,
                &go_index,
                &go_object,
                history_delta_arg(args.first()),
            )?;
            Ok(JsValue::Undefined)
        }),
    );

    JsValue::Object(object)
}

fn history_traverse(
    location: &Rc<RefCell<HashMap<String, JsValue>>>,
    stack: &Rc<RefCell<Vec<(String, JsValue)>>>,
    index: &Rc<RefCell<usize>>,
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    delta: isize,
) -> Result<(), String> {
    let target = *index.borrow() as isize + delta;
    if target < 0 || target >= stack.borrow().len() as isize || delta == 0 {
        return Ok(());
    }
    let old_href = location_href(location);
    *index.borrow_mut() = target as usize;
    let (href, state) = stack.borrow()[target as usize].clone();
    set_location_href(location, &href);
    set_history_state(object, &state)?;
    lifecycle_host::dispatch_popstate_event(compat_host::structured_clone(&state)?)?;
    if location_hash(&old_href) != location_hash(&href) {
        lifecycle_host::dispatch_hashchange_event(&old_href, &href)?;
    }
    Ok(())
}

fn history_delta_arg(value: Option<&JsValue>) -> isize {
    match value {
        Some(JsValue::Number(value)) => *value as isize,
        Some(value) => value.display().parse().unwrap_or(0),
        None => 0,
    }
}

fn set_history_length(object: &Rc<RefCell<HashMap<String, JsValue>>>, len: usize) {
    object
        .borrow_mut()
        .insert("length".into(), JsValue::Number(len as f64));
}

fn history_state_arg(args: &[JsValue]) -> Result<JsValue, String> {
    match args.first() {
        Some(value) => compat_host::structured_clone(value),
        None => Ok(JsValue::Null),
    }
}

fn set_history_state(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    state: &JsValue,
) -> Result<(), String> {
    object
        .borrow_mut()
        .insert("state".into(), compat_host::structured_clone(state)?);
    Ok(())
}

fn location_href(location: &Rc<RefCell<HashMap<String, JsValue>>>) -> String {
    location
        .borrow()
        .get("href")
        .map(JsValue::display)
        .unwrap_or_else(|| "http://localhost/".into())
}

fn location_hash(href: &str) -> &str {
    href.split_once('#').map_or("", |(_, hash)| hash)
}

fn set_location_href(location: &Rc<RefCell<HashMap<String, JsValue>>>, href: &str) {
    let mut parsed = parse_location(href);
    install_location_methods_on_map(&mut parsed, location.clone());
    *location.borrow_mut() = parsed;
}

fn install_location_methods_on_map(
    map: &mut HashMap<String, JsValue>,
    target: Rc<RefCell<HashMap<String, JsValue>>>,
) {
    let assign_target = target.clone();
    map.insert(
        "assign".into(),
        native("location.assign", Some(1), move |args| {
            set_location_href(
                &assign_target,
                &args.first().unwrap_or(&JsValue::Undefined).display(),
            );
            Ok(JsValue::Undefined)
        }),
    );
    let replace_target = target;
    map.insert(
        "replace".into(),
        native("location.replace", Some(1), move |args| {
            set_location_href(
                &replace_target,
                &args.first().unwrap_or(&JsValue::Undefined).display(),
            );
            Ok(JsValue::Undefined)
        }),
    );
    map.insert(
        "reload".into(),
        native("location.reload", None, move |_| Ok(JsValue::Undefined)),
    );
}

fn navigator_object() -> JsValue {
    let mut obj = HashMap::new();
    obj.insert(
        "userAgent".into(),
        JsValue::String("TetherScript/0.1 BrowserCompat".into()),
    );
    obj.insert("language".into(), JsValue::String("en-US".into()));
    obj.insert(
        "languages".into(),
        JsValue::Array(Rc::new(RefCell::new(vec![
            JsValue::String("en-US".into()),
            JsValue::String("en".into()),
        ]))),
    );
    obj.insert(
        "platform".into(),
        JsValue::String(std::env::consts::OS.into()),
    );
    obj.insert("cookieEnabled".into(), JsValue::Bool(false));
    obj.insert("onLine".into(), JsValue::Bool(true));
    obj.insert("hardwareConcurrency".into(), JsValue::Number(4.0));
    obj.insert("deviceMemory".into(), JsValue::Number(8.0));
    obj.insert("vendor".into(), JsValue::String("TetherScript".into()));
    obj.insert("product".into(), JsValue::String("Gecko".into()));
    obj.insert("maxTouchPoints".into(), JsValue::Number(0.0));
    obj.insert("webdriver".into(), JsValue::Bool(false));
    let connection = network_information_object();
    obj.insert("connection".into(), connection.clone());
    obj.insert("networkInformation".into(), connection);
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn network_information_object() -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::from([
        ("effectiveType".into(), JsValue::String("4g".into())),
        ("downlink".into(), JsValue::Number(10.0)),
        ("rtt".into(), JsValue::Number(50.0)),
        ("saveData".into(), JsValue::Bool(false)),
        ("onchange".into(), JsValue::Null),
    ])));
    install_network_information_events(&object);
    JsValue::Object(object)
}

fn install_network_information_events(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let add_object = object.clone();
    object.borrow_mut().insert(
        "addEventListener".into(),
        native(
            "NetworkInformation.addEventListener",
            Some(2),
            move |args| {
                if args.first().unwrap_or(&JsValue::Undefined).display() == "change" {
                    let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
                    let mut object = add_object.borrow_mut();
                    let listeners = object
                        .entry("__listeners:change".into())
                        .or_insert_with(|| JsValue::Array(Rc::new(RefCell::new(Vec::new()))));
                    if let JsValue::Array(items) = listeners {
                        items.borrow_mut().push(listener);
                    }
                }
                Ok(JsValue::Undefined)
            },
        ),
    );
    let remove_object = object.clone();
    object.borrow_mut().insert(
        "removeEventListener".into(),
        native(
            "NetworkInformation.removeEventListener",
            Some(2),
            move |args| {
                let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
                if args.first().unwrap_or(&JsValue::Undefined).display() == "change" {
                    if let Some(JsValue::Array(items)) =
                        remove_object.borrow().get("__listeners:change")
                    {
                        items.borrow_mut().retain(|item| *item != listener);
                    }
                }
                Ok(JsValue::Undefined)
            },
        ),
    );
    let dispatch_object = object.clone();
    object.borrow_mut().insert(
        "dispatchEvent".into(),
        native("NetworkInformation.dispatchEvent", Some(1), move |args| {
            dispatch_network_information_event(
                &dispatch_object,
                args.first().cloned().unwrap_or(JsValue::Undefined),
            )
        }),
    );
}

fn dispatch_network_information_event(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    event: JsValue,
) -> Result<JsValue, String> {
    let Some(kind) = event_type(&event) else {
        return Ok(JsValue::Bool(true));
    };
    if kind != "change" {
        return Ok(JsValue::Bool(true));
    }
    let this_value = JsValue::Object(object.clone());
    let event = normalize_event(event, "change", this_value.clone(), this_value.clone());
    let listeners = object
        .borrow()
        .get("__listeners:change")
        .and_then(|value| match value {
            JsValue::Array(items) => Some(items.borrow().clone()),
            _ => None,
        })
        .unwrap_or_default();
    for listener in listeners {
        call_dom_listener(listener, this_value.clone(), event.clone())?;
    }
    if let Some(handler) = object.borrow().get("onchange").cloned() {
        if !matches!(handler, JsValue::Null | JsValue::Undefined) {
            call_dom_listener(handler, this_value, event.clone())?;
        }
    }
    Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")))
}

fn storage_object(
    label: &'static str,
    entries: Vec<(String, String)>,
    event_window: Option<StorageEventWindow>,
) -> (JsValue, Rc<RefCell<StorageArea>>) {
    let area = Rc::new(RefCell::new(StorageArea { entries }));
    let object = Rc::new(RefCell::new(HashMap::new()));

    object
        .borrow_mut()
        .insert("length".into(), JsValue::Number(area.borrow().len() as f64));

    {
        let area = area.clone();
        object.borrow_mut().insert(
            "getItem".into(),
            native(&format!("{}.getItem", label), Some(1), move |args| {
                let key = args.first().unwrap_or(&JsValue::Undefined).display();
                Ok(area
                    .borrow()
                    .get(&key)
                    .map(JsValue::String)
                    .unwrap_or(JsValue::Null))
            }),
        );
    }

    {
        let area = area.clone();
        let object = object.clone();
        let object_for_closure = object.clone();
        let event_window = event_window.clone();
        let set_item = native(&format!("{}.setItem", label), Some(2), move |args| {
            let key = args.first().unwrap_or(&JsValue::Undefined).display();
            let value = args.get(1).unwrap_or(&JsValue::Undefined).display();
            let old_value = area.borrow().get(&key);
            if old_value.as_deref() == Some(value.as_str()) {
                return Ok(JsValue::Undefined);
            }
            area.borrow_mut().set(key.clone(), value.clone());
            sync_storage_length(&object_for_closure, &area);
            if let Some(window) = &event_window {
                dispatch_storage_event(
                    window,
                    &object_for_closure,
                    Some(key),
                    old_value,
                    Some(value),
                )?;
            }
            Ok(JsValue::Undefined)
        });
        object.borrow_mut().insert("setItem".into(), set_item);
    }

    {
        let area = area.clone();
        let object = object.clone();
        let object_for_closure = object.clone();
        let event_window = event_window.clone();
        let remove_item = native(&format!("{}.removeItem", label), Some(1), move |args| {
            let key = args.first().unwrap_or(&JsValue::Undefined).display();
            let old_value = area.borrow_mut().remove(&key);
            if let Some(old_value) = old_value {
                sync_storage_length(&object_for_closure, &area);
                if let Some(window) = &event_window {
                    dispatch_storage_event(
                        window,
                        &object_for_closure,
                        Some(key),
                        Some(old_value),
                        None,
                    )?;
                }
            }
            Ok(JsValue::Undefined)
        });
        object.borrow_mut().insert("removeItem".into(), remove_item);
    }

    {
        let area = area.clone();
        let object = object.clone();
        let object_for_closure = object.clone();
        let event_window = event_window.clone();
        let clear = native(&format!("{}.clear", label), Some(0), move |_| {
            if area.borrow().len() == 0 {
                return Ok(JsValue::Undefined);
            }
            area.borrow_mut().clear();
            sync_storage_length(&object_for_closure, &area);
            if let Some(window) = &event_window {
                dispatch_storage_event(window, &object_for_closure, None, None, None)?;
            }
            Ok(JsValue::Undefined)
        });
        object.borrow_mut().insert("clear".into(), clear);
    }

    {
        let area = area.clone();
        object.borrow_mut().insert(
            "key".into(),
            native(&format!("{}.key", label), Some(1), move |args| {
                let index = args.first().map(storage_index).unwrap_or(usize::MAX);
                Ok(area
                    .borrow()
                    .key(index)
                    .map(JsValue::String)
                    .unwrap_or(JsValue::Null))
            }),
        );
    }

    (JsValue::Object(object), area)
}

fn dispatch_storage_event(
    window: &StorageEventWindow,
    storage: &Rc<RefCell<HashMap<String, JsValue>>>,
    key: Option<String>,
    old_value: Option<String>,
    new_value: Option<String>,
) -> Result<(), String> {
    let Some(window) = window.borrow().clone() else {
        return Ok(());
    };
    let mut event = HashMap::new();
    event.insert("key".into(), storage_event_value(key));
    event.insert("oldValue".into(), storage_event_value(old_value));
    event.insert("newValue".into(), storage_event_value(new_value));
    event.insert("url".into(), JsValue::String(window_location_href(&window)));
    event.insert("storageArea".into(), JsValue::Object(storage.clone()));
    event.insert("bubbles".into(), JsValue::Bool(false));
    event.insert("cancelable".into(), JsValue::Bool(false));
    let event = normalize_event(
        JsValue::Object(Rc::new(RefCell::new(event))),
        "storage",
        window.clone(),
        window.clone(),
    );
    dispatch_window_normalized("storage", event, window)
}

fn storage_event_value(value: Option<String>) -> JsValue {
    value.map(JsValue::String).unwrap_or(JsValue::Null)
}

fn window_location_href(window: &JsValue) -> String {
    let JsValue::Object(window) = window else {
        return "http://localhost/".into();
    };
    match window.borrow().get("location").cloned() {
        Some(JsValue::Object(location)) => location_href(&location),
        _ => "http://localhost/".into(),
    }
}

fn sync_storage_length(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    area: &Rc<RefCell<StorageArea>>,
) {
    object
        .borrow_mut()
        .insert("length".into(), JsValue::Number(area.borrow().len() as f64));
}

fn storage_index(value: &JsValue) -> usize {
    match value {
        JsValue::Number(n) if n.is_finite() && *n >= 0.0 => *n as usize,
        other => other.display().parse().unwrap_or(usize::MAX),
    }
}

impl StorageArea {
    fn get(&self, key: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|(existing, _)| existing == key)
            .map(|(_, value)| value.clone())
    }
    fn set(&mut self, key: String, value: String) {
        if let Some((_, existing)) = self
            .entries
            .iter_mut()
            .find(|(existing, _)| existing == &key)
        {
            *existing = value;
        } else {
            self.entries.push((key, value));
        }
    }
    fn remove(&mut self, key: &str) -> Option<String> {
        let index = self
            .entries
            .iter()
            .position(|(existing, _)| existing == key)?;
        Some(self.entries.remove(index).1)
    }
    fn clear(&mut self) {
        self.entries.clear();
    }
    fn key(&self, index: usize) -> Option<String> {
        self.entries.get(index).map(|(key, _)| key.clone())
    }
    fn len(&self) -> usize {
        self.entries.len()
    }
}

fn node_object(handle: DomHandle) -> JsValue {
    let node = handle.node().unwrap_or(Node::Text(String::new()));
    let handle_id = register_dom_handle(&handle);
    let self_object = Rc::new(RefCell::new(None::<Rc<RefCell<HashMap<String, JsValue>>>>));
    let mut obj = HashMap::new();
    obj.insert(
        "nodeType".into(),
        JsValue::Number(if matches!(node, Node::Text(_)) {
            3.0
        } else if node_name(&node) == "#document" {
            9.0
        } else if node_name(&node) == "#document-fragment" {
            11.0
        } else {
            1.0
        }),
    );
    obj.insert("nodeName".into(), JsValue::String(node_name(&node)));
    obj.insert("__domHandleId".into(), JsValue::String(handle_id));
    obj.insert("__domPath".into(), JsValue::String(path_key(&handle.path)));
    obj.insert(
        "tagName".into(),
        JsValue::String(node_name(&node).to_ascii_uppercase()),
    );
    install_node_text_getters(&mut obj, &handle);
    obj.insert(
        "children".into(),
        children_collection(&handle, &node, "HTMLCollection"),
    );
    obj.insert(
        "childNodes".into(),
        children_collection(&handle, &node, "NodeList"),
    );
    obj.insert(
        "parentNode".into(),
        optional_node_ref_object(handle.parent()),
    );
    obj.insert(
        "parentElement".into(),
        optional_node_ref_object(handle.parent()),
    );
    obj.insert(
        "firstChild".into(),
        optional_node_ref_object(handle.child_at(0)),
    );
    obj.insert(
        "lastChild".into(),
        optional_node_ref_object(handle.last_child()),
    );
    obj.insert(
        "previousSibling".into(),
        optional_node_ref_object(handle.previous_sibling()),
    );
    obj.insert(
        "nextSibling".into(),
        optional_node_ref_object(handle.next_sibling()),
    );
    obj.insert(
        "firstElementChild".into(),
        optional_node_ref_object(handle.first_element_child()),
    );
    obj.insert(
        "lastElementChild".into(),
        optional_node_ref_object(handle.last_element_child()),
    );
    obj.insert(
        "previousElementSibling".into(),
        optional_node_ref_object(handle.previous_element_sibling()),
    );
    obj.insert(
        "nextElementSibling".into(),
        optional_node_ref_object(handle.next_element_sibling()),
    );
    if node_name(&node) != "#document" {
        obj.insert(
            "ownerDocument".into(),
            document_reference_object(handle.root.clone()),
        );
    }
    obj.insert(
        "childElementCount".into(),
        JsValue::Number(child_element_count(&node) as f64),
    );

    if let Node::Element(el) = &node {
        obj.insert(
            "id".into(),
            JsValue::String(el.attrs.get("id").cloned().unwrap_or_default()),
        );
        obj.insert(
            "className".into(),
            JsValue::String(el.attrs.get("class").cloned().unwrap_or_default()),
        );
        obj.insert("classList".into(), class_list_host::object(handle.clone()));
        obj.insert("dataset".into(), dataset_object(handle.clone(), el));
        selection_host::editable_props::install(&mut obj, &handle, el, self_object.clone());
        if !el.tag.starts_with('#') {
            fullscreen_host::install_element(&mut obj, &handle, self_object.clone());
            obj.insert(
                "shadowRoot".into(),
                custom_host::open_shadow_root_object(&handle),
            );
            let h = handle.clone();
            let host_object = self_object.clone();
            obj.insert(
                "attachShadow".into(),
                native("attachShadow", Some(1), move |args| {
                    let root = custom_host::attach_shadow_root(&h, args.first());
                    if let Some(obj) = host_object.borrow().as_ref() {
                        obj.borrow_mut().insert(
                            "shadowRoot".into(),
                            custom_host::open_shadow_root_object(&h),
                        );
                    }
                    Ok(root)
                }),
            );
        }
        if el.tag == "canvas" {
            let (width, height) = canvas_host::dimensions(&handle);
            obj.insert("width".into(), JsValue::Number(width as f64));
            obj.insert("height".into(), JsValue::Number(height as f64));
            obj.insert(
                "getContext".into(),
                canvas_host::get_context(handle.clone()),
            );
        }
        if matches!(el.tag.as_str(), "audio" | "video") {
            media_host::install_element(&mut obj, &handle, &el.tag, self_object.clone());
        }
        live_form_host::install(&mut obj, &handle);
        if matches!(el.tag.as_str(), "input" | "textarea") {
            let (start, end) = selection_for_handle(&handle);
            obj.insert("selectionStart".into(), JsValue::Number(start as f64));
            obj.insert("selectionEnd".into(), JsValue::Number(end as f64));
            obj.insert("selectionDirection".into(), JsValue::String("none".into()));
        }
        install_offset_getters(&mut obj, &handle);

        if el.tag == "form" {
            obj.insert(
                "action".into(),
                JsValue::String(el.attrs.get("action").cloned().unwrap_or_default()),
            );
            obj.insert(
                "method".into(),
                JsValue::String(
                    el.attrs
                        .get("method")
                        .map(|m| m.to_ascii_lowercase())
                        .unwrap_or_else(|| "get".into()),
                ),
            );
        }

        if el.tag == "#document" {
            obj.insert(
                "cookie".into(),
                JsValue::String(cookie_host::cookie_string()),
            );
            let active = FOCUSED_ELEMENT.with(|focused| {
                focused
                    .borrow()
                    .as_ref()
                    .and_then(|key| handle_by_event_key(key))
                    .map(node_object)
                    .unwrap_or(JsValue::Null)
            });
            obj.insert("activeElement".into(), active);
            selection_host::install_document(&mut obj, &handle);
            fullscreen_host::install_document(&mut obj, &handle);
        }
    }

    dom_compat_host::install_node(&mut obj, &handle, &node);
    install_property_setters(&mut obj, &handle);

    obj.insert(
        "createElement".into(),
        native("createElement", Some(1), move |args| {
            let tag = args
                .first()
                .unwrap_or(&JsValue::Undefined)
                .display()
                .to_ascii_lowercase();
            detached_element_object(tag)
        }),
    );

    obj.insert(
        "createDocumentFragment".into(),
        native("createDocumentFragment", Some(0), move |_| {
            Ok(detached_node_object(Node::Element(Element {
                tag: "#document-fragment".into(),
                attrs: HashMap::new(),
                children: Vec::new(),
            })))
        }),
    );

    obj.insert(
        "createTextNode".into(),
        native("createTextNode", Some(1), move |args| {
            let text = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(detached_node_object(Node::Text(text)))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "appendChild".into(),
        native("appendChild", Some(1), move |args| {
            let child_value = args.first().unwrap_or(&JsValue::Undefined);
            let mut h = current_dom_handle(&object_ref, &h);
            let (child, moving) = adopted_node_for_insert(child_value, &h)?;
            h = current_dom_handle(&object_ref, &h);
            let path = h.append_child(child, InsertPosition::Append)?;
            reattach_inserted_dom_handles(moving.as_ref(), &h.root, &path);
            if let Some(object) = object_ref.borrow().as_ref() {
                refresh_node_properties(object, &h);
            }
            refresh_value_node_properties(child_value);
            Ok(child_value.clone())
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "insertBefore".into(),
        native("insertBefore", Some(2), move |args| {
            let child_value = args.first().unwrap_or(&JsValue::Undefined);
            let mut h = current_dom_handle(&object_ref, &h);
            if same_optional_dom_handle(
                dom_handle_from_value(child_value).as_ref(),
                args.get(1).and_then(dom_handle_from_value).as_ref(),
            ) {
                return Ok(child_value.clone());
            }
            let (child, moving) = adopted_node_for_insert(child_value, &h)?;
            h = current_dom_handle(&object_ref, &h);
            let reference = args.get(1).and_then(dom_handle_from_value);
            let path = h.insert_child_before(child, reference.as_ref())?;
            reattach_inserted_dom_handles(moving.as_ref(), &h.root, &path);
            if let Some(object) = object_ref.borrow().as_ref() {
                refresh_node_properties(object, &h);
            }
            refresh_value_node_properties(child_value);
            Ok(child_value.clone())
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "replaceChild".into(),
        native("replaceChild", Some(2), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let child = js_value_to_node(args.first().unwrap_or(&JsValue::Undefined));
            let old = args.get(1).and_then(dom_handle_from_value);
            Ok(h.replace_child(child, old.as_ref())?
                .map(detached_node_object)
                .unwrap_or(JsValue::Null))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "removeChild".into(),
        native("removeChild", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let old = args.first().and_then(dom_handle_from_value);
            Ok(h.remove_child(old.as_ref())?
                .map(detached_node_object)
                .unwrap_or(JsValue::Null))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "cloneNode".into(),
        native("cloneNode", None, move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let deep = args.first().is_some_and(JsValue::truthy);
            Ok(h.node()
                .map(|node| detached_node_object(clone_node(&node, deep)))
                .unwrap_or(JsValue::Null))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "setSelectionRange".into(),
        native("setSelectionRange", None, move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let start = args.first().map(selection_index).unwrap_or(0);
            let end = args.get(1).map(selection_index).unwrap_or(start);
            set_selection_for_handle(&h, start, end);
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "select".into(),
        native("select", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            let len = h.input_value().chars().count();
            set_selection_for_handle(&h, 0, len);
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "typeText".into(),
        native("typeText", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let text = args.first().unwrap_or(&JsValue::Undefined).display();
            h.insert_text_at_selection(&text)?;
            if let Some(object) = object_ref.borrow().as_ref() {
                refresh_text_properties(object, &h);
            }
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "focus".into(),
        native("focus", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            h.focus()?;
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "blur".into(),
        native("blur", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            h.blur()?;
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "getAttribute".into(),
        native("getAttribute", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(match h.node() {
                Some(Node::Element(el)) => el
                    .attrs
                    .get(&name)
                    .map(|s| JsValue::String(s.clone()))
                    .unwrap_or(JsValue::Null),
                _ => JsValue::Null,
            })
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "setAttribute".into(),
        native("setAttribute", Some(2), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            let value = args.get(1).unwrap_or(&JsValue::Undefined).display();
            let new_value = value.clone();
            let old_value = h.node().and_then(|node| match node {
                Node::Element(el) => el.attrs.get(&name).cloned(),
                Node::Text(_) => None,
            });
            h.with_node_mut(|node| {
                if let Node::Element(el) = node {
                    el.attrs.insert(name.clone(), value);
                }
            });
            if canvas_host::is_dimension_attr(&h, &name) {
                canvas_host::reset_surface(&h);
            }
            queue_mutation_record(
                &h,
                "attributes",
                Some(name.clone()),
                old_value.clone(),
                Vec::new(),
                Vec::new(),
            );
            custom_host::attribute_changed(&h, &name, old_value, Some(new_value))?;
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "removeAttribute".into(),
        native("removeAttribute", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            let old_value = h.node().and_then(|node| match node {
                Node::Element(el) => el.attrs.get(&name).cloned(),
                Node::Text(_) => None,
            });
            h.with_node_mut(|node| {
                if let Node::Element(el) = node {
                    el.attrs.remove(&name);
                }
            });
            if canvas_host::is_dimension_attr(&h, &name) {
                canvas_host::reset_surface(&h);
            }
            queue_mutation_record(
                &h,
                "attributes",
                Some(name.clone()),
                old_value.clone(),
                Vec::new(),
                Vec::new(),
            );
            custom_host::attribute_changed(&h, &name, old_value, None)?;
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "hasAttribute".into(),
        native("hasAttribute", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(JsValue::Bool(
                matches!(h.node(), Some(Node::Element(el)) if el.attrs.contains_key(&name)),
            ))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "remove".into(),
        native("remove", Some(0), move |_| {
            current_dom_handle(&object_ref, &h).remove_self()?;
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "prepend".into(),
        native("prepend", None, move |args| {
            let h = current_dom_handle(&object_ref, &h);
            for arg in args.iter().rev() {
                let moving = dom_handle_from_value(arg);
                let path = h.append_child(js_value_to_node(arg), InsertPosition::Prepend)?;
                reattach_inserted_dom_handles(moving.as_ref(), &h.root, &path);
            }
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "append".into(),
        native("append", None, move |args| {
            let h = current_dom_handle(&object_ref, &h);
            for arg in args {
                let moving = dom_handle_from_value(arg);
                let path = h.append_child(js_value_to_node(arg), InsertPosition::Append)?;
                reattach_inserted_dom_handles(moving.as_ref(), &h.root, &path);
            }
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    obj.insert(
        "getElementById".into(),
        native("getElementById", Some(1), move |args| {
            let id = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(find_by_id(&h.root, &id)
                .map(|path| {
                    node_object(DomHandle {
                        root: h.root.clone(),
                        path,
                    })
                })
                .unwrap_or(JsValue::Null))
        }),
    );

    let h = handle.clone();
    obj.insert(
        "querySelector".into(),
        native("querySelector", Some(1), move |args| {
            let selector = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(find_by_selector(&h.root, &selector)
                .map(|path| {
                    node_object(DomHandle {
                        root: h.root.clone(),
                        path,
                    })
                })
                .unwrap_or(JsValue::Null))
        }),
    );

    let h = handle.clone();
    obj.insert(
        "querySelectorAll".into(),
        native("querySelectorAll", Some(1), move |args| {
            let selector = args.first().unwrap_or(&JsValue::Undefined).display();
            let nodes = all_by_selector(&h.root, &selector)
                .into_iter()
                .map(|path| {
                    node_object(DomHandle {
                        root: h.root.clone(),
                        path,
                    })
                })
                .collect();
            Ok(dom_collection("NodeList", nodes))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "matches".into(),
        native("matches", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let selector = args.first().unwrap_or(&JsValue::Undefined).display();
            let Some(Node::Element(el)) = h.node() else {
                return Ok(JsValue::Bool(false));
            };
            Ok(JsValue::Bool(browser::element_matches(
                &el,
                &h.ancestors(),
                &selector,
            )))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "closest".into(),
        native("closest", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let selector = args.first().unwrap_or(&JsValue::Undefined).display();
            let mut path = h.path.clone();
            loop {
                let candidate = DomHandle {
                    root: h.root.clone(),
                    path: path.clone(),
                };
                if let Some(Node::Element(el)) = candidate.node() {
                    if browser::element_matches(&el, &candidate.ancestors(), &selector) {
                        return Ok(node_object(candidate));
                    }
                }
                if path.pop().is_none() {
                    return Ok(JsValue::Null);
                }
            }
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "addEventListener".into(),
        native("addEventListener", None, move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let (capture, once) = event_listener_options(args.get(2));
            h.add_event_listener(&event_type, listener, capture, once);
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "removeEventListener".into(),
        native("removeEventListener", None, move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let (capture, _) = event_listener_options(args.get(2));
            h.remove_event_listener(&event_type, &listener, capture);
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "dispatchEvent".into(),
        native("dispatchEvent", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let event = args.first().cloned().unwrap_or(JsValue::Undefined);
            h.dispatch_event(event)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "getBoundingClientRect".into(),
        native("getBoundingClientRect", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            Ok(rect_object(&element_rect(&h)))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "getComputedStyle".into(),
        native("getComputedStyle", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            Ok(computed_style_object(&h))
        }),
    );

    let h = handle.clone();
    obj.insert(
        "getAccessibilityTree".into(),
        native("getAccessibilityTree", Some(0), move |_| {
            Ok(accessibility_tree_object(&root_to_document(&h.root)))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "collectFormData".into(),
        native("collectFormData", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            Ok(form_data_object(&h))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "requestSubmit".into(),
        native("requestSubmit", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            submit_form(&h, true)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "submit".into(),
        native("submit", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            submit_form(&h, true)
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "click".into(),
        native("click", Some(0), move |_| {
            let h = current_dom_handle(&object_ref, &h);
            h.dispatch_event(JsValue::String("click".into()))
        }),
    );

    let h = handle.clone();
    let object_ref = self_object.clone();
    obj.insert(
        "inputText".into(),
        native("inputText", Some(1), move |args| {
            let h = current_dom_handle(&object_ref, &h);
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            h.set_input_value(value);
            h.dispatch_event(JsValue::String("input".into()))?;
            h.dispatch_event(JsValue::String("change".into()))?;
            Ok(JsValue::Undefined)
        }),
    );

    obj.insert(
        "__domApiVersion".into(),
        JsValue::String(DOM_API_VERSION.into()),
    );

    let object = Rc::new(RefCell::new(obj));
    *self_object.borrow_mut() = Some(object.clone());
    dom_compat_host::install_live_node(&object, &handle, &node);
    JsValue::Object(object)
}

fn install_node_text_getters(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    for prop in ["textContent", "innerText", "innerHTML", "outerHTML"] {
        let h = handle.clone();
        obj.insert(
            format!("__get:{prop}"),
            native(&format!("get_{prop}"), Some(0), move |_| {
                Ok(JsValue::String(node_text_property(&h, prop)))
            }),
        );
    }
}

fn install_offset_getters(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    for prop in ["offsetWidth", "offsetHeight"] {
        let h = handle.clone();
        obj.insert(
            format!("__get:{prop}"),
            native(&format!("get_{prop}"), Some(0), move |_| {
                let (width, height) = element_offset_size(&h);
                Ok(JsValue::Number(if prop == "offsetWidth" {
                    width as f64
                } else {
                    height as f64
                }))
            }),
        );
    }
}

fn node_text_property(handle: &DomHandle, prop: &str) -> String {
    let Some(node) = handle.node() else {
        return String::new();
    };
    match prop {
        "textContent" => text_content_raw(&node),
        "innerText" => browser::text_content(&node),
        "innerHTML" => inner_html(&node),
        "outerHTML" => outer_html(&node),
        _ => String::new(),
    }
}

fn refresh_text_properties(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    install_node_text_getters(&mut obj.borrow_mut(), handle);
}

fn refresh_node_properties(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    if let Some(node) = handle.node() {
        let mut obj = obj.borrow_mut();
        obj.insert(
            "children".into(),
            children_collection(handle, &node, "HTMLCollection"),
        );
        obj.insert(
            "childNodes".into(),
            children_collection(handle, &node, "NodeList"),
        );
        obj.insert(
            "childElementCount".into(),
            JsValue::Number(child_element_count(&node) as f64),
        );
        obj.insert(
            "isConnected".into(),
            JsValue::Bool(handle_is_connected(handle)),
        );
        obj.insert(
            "parentNode".into(),
            optional_node_ref_object(handle.parent()),
        );
        obj.insert(
            "parentElement".into(),
            optional_node_ref_object(handle.parent()),
        );
        obj.insert(
            "previousSibling".into(),
            optional_node_ref_object(handle.previous_sibling()),
        );
        obj.insert(
            "nextSibling".into(),
            optional_node_ref_object(handle.next_sibling()),
        );
        if node_name(&node) != "#document" {
            obj.insert(
                "ownerDocument".into(),
                document_reference_object(handle.root.clone()),
            );
        }
        install_node_text_getters(&mut obj, handle);
    }
}

fn handle_is_connected(handle: &DomHandle) -> bool {
    handle.node().is_some()
        && matches!(&*handle.root.borrow(), Node::Element(el) if el.tag == "#document")
}

fn refresh_value_node_properties(value: &JsValue) {
    let JsValue::Object(obj) = value else {
        return;
    };
    if let Some(handle) = dom_handle_from_value(value) {
        refresh_node_properties(obj, &handle);
    }
}

impl DomHandle {
    fn node(&self) -> Option<Node> {
        get_node(&self.root.borrow(), &self.path).cloned()
    }

    fn parent(&self) -> Option<DomHandle> {
        if self.path.is_empty() {
            return None;
        }
        Some(DomHandle {
            root: self.root.clone(),
            path: self.path[..self.path.len() - 1].to_vec(),
        })
    }

    fn child_at(&self, index: usize) -> Option<DomHandle> {
        match self.node()? {
            Node::Element(el) if index < el.children.len() => {
                let mut path = self.path.clone();
                path.push(index);
                Some(DomHandle {
                    root: self.root.clone(),
                    path,
                })
            }
            _ => None,
        }
    }

    fn last_child(&self) -> Option<DomHandle> {
        let len = match self.node()? {
            Node::Element(el) => el.children.len(),
            Node::Text(_) => 0,
        };
        len.checked_sub(1).and_then(|index| self.child_at(index))
    }

    fn previous_sibling(&self) -> Option<DomHandle> {
        let (&index, parent_path) = self.path.split_last()?;
        let previous = index.checked_sub(1)?;
        Some(DomHandle {
            root: self.root.clone(),
            path: parent_path.iter().copied().chain([previous]).collect(),
        })
    }

    fn next_sibling(&self) -> Option<DomHandle> {
        let (&index, parent_path) = self.path.split_last()?;
        let parent = DomHandle {
            root: self.root.clone(),
            path: parent_path.to_vec(),
        };
        let next = index + 1;
        match parent.node()? {
            Node::Element(el) if next < el.children.len() => Some(DomHandle {
                root: self.root.clone(),
                path: parent_path.iter().copied().chain([next]).collect(),
            }),
            _ => None,
        }
    }

    fn first_element_child(&self) -> Option<DomHandle> {
        self.element_child_range(0, 1)
    }

    fn last_element_child(&self) -> Option<DomHandle> {
        let len = match self.node()? {
            Node::Element(el) => el.children.len(),
            Node::Text(_) => 0,
        };
        self.element_child_range(len.checked_sub(1)?, -1)
    }

    fn previous_element_sibling(&self) -> Option<DomHandle> {
        let index = self.path.last().copied()?.checked_sub(1)?;
        self.parent()?.element_child_range(index, -1)
    }

    fn next_element_sibling(&self) -> Option<DomHandle> {
        self.parent()?
            .element_child_range(self.path.last().copied()? + 1, 1)
    }

    fn element_child_range(&self, start: usize, step: isize) -> Option<DomHandle> {
        let len = match self.node()? {
            Node::Element(el) => el.children.len(),
            Node::Text(_) => 0,
        };
        let mut index = start as isize;
        while index >= 0 && (index as usize) < len {
            let child = self.child_at(index as usize)?;
            if matches!(child.node(), Some(Node::Element(el)) if !el.tag.starts_with('#')) {
                return Some(child);
            }
            index += step;
        }
        None
    }

    fn with_node_mut(&self, f: impl FnOnce(&mut Node)) {
        if let Some(node) = get_node_mut(&mut self.root.borrow_mut(), &self.path) {
            f(node);
        }
    }

    fn append_child(&self, child: Node, position: InsertPosition) -> Result<Vec<usize>, String> {
        self.insert_child_at(
            child,
            match position {
                InsertPosition::Append => None,
                InsertPosition::Prepend => Some(0),
            },
        )
    }

    fn insert_child_before(
        &self,
        child: Node,
        reference: Option<&DomHandle>,
    ) -> Result<Vec<usize>, String> {
        let index = reference
            .filter(|reference| {
                Rc::ptr_eq(&self.root, &reference.root)
                    && reference.path.len() == self.path.len() + 1
                    && reference.path.starts_with(&self.path)
            })
            .and_then(|reference| reference.path.last().copied());
        self.insert_child_at(child, index)
    }

    fn insert_child_at(&self, child: Node, index: Option<usize>) -> Result<Vec<usize>, String> {
        let added_nodes = added_nodes_for_record(&child);
        let (path, inserted_count) = {
            let mut root = self.root.borrow_mut();
            let parent = get_node_mut(&mut root, &self.path);
            let Some(Node::Element(el)) = parent else {
                return Ok(self.path.clone());
            };
            let mut inserted_at = index.unwrap_or(el.children.len()).min(el.children.len());
            let inserted_count = match child {
                Node::Element(fragment) if fragment.tag == "#document-fragment" => {
                    let mut fragment_children = fragment.children;
                    let count = fragment_children.len();
                    for child in fragment_children.drain(..).rev() {
                        el.children.insert(inserted_at, child);
                    }
                    count
                }
                child => {
                    el.children.insert(inserted_at, child);
                    1
                }
            };
            if inserted_count == 0 {
                inserted_at = inserted_at.saturating_sub(1);
            }
            let mut path = self.path.clone();
            path.push(inserted_at);
            (path, inserted_count)
        };
        adjust_dom_handles_after_insert(
            &self.root,
            &self.path,
            *path.last().unwrap_or(&0),
            inserted_count,
        );
        queue_mutation_record(self, "childList", None, None, added_nodes, Vec::new());
        self.connect_inserted_children(&path, inserted_count)?;
        Ok(path)
    }

    fn connect_inserted_children(&self, first_path: &[usize], count: usize) -> Result<(), String> {
        let Some(start) = first_path.last().copied() else {
            return Ok(());
        };
        for offset in 0..count {
            let mut path = self.path.clone();
            path.push(start + offset);
            custom_host::connected(&DomHandle {
                root: self.root.clone(),
                path,
            })?;
        }
        Ok(())
    }

    fn remove_self(&self) -> Result<bool, String> {
        let Some((&index, parent_path)) = self.path.split_last() else {
            return Ok(false);
        };
        let removed = {
            let mut root = self.root.borrow_mut();
            let Some(Node::Element(parent)) = get_node_mut(&mut root, parent_path) else {
                return Ok(false);
            };
            if index >= parent.children.len() {
                return Ok(false);
            }
            parent.children.remove(index)
        };
        adjust_dom_handles_after_remove(&self.root, parent_path, index, &removed);
        let parent_handle = DomHandle {
            root: self.root.clone(),
            path: parent_path.to_vec(),
        };
        queue_mutation_record(
            &parent_handle,
            "childList",
            None,
            None,
            Vec::new(),
            vec![removed.clone()],
        );
        custom_host::disconnected(removed)?;
        Ok(true)
    }

    fn remove_child(&self, child: Option<&DomHandle>) -> Result<Option<Node>, String> {
        let Some(child) = child else {
            return Ok(None);
        };
        if !Rc::ptr_eq(&self.root, &child.root)
            || child.path.len() != self.path.len() + 1
            || !child.path.starts_with(&self.path)
        {
            return Ok(None);
        }
        let Some(index) = child.path.last().copied() else {
            return Ok(None);
        };
        let removed = {
            let mut root = self.root.borrow_mut();
            let Some(Node::Element(parent)) = get_node_mut(&mut root, &self.path) else {
                return Ok(None);
            };
            if index >= parent.children.len() {
                return Ok(None);
            }
            parent.children.remove(index)
        };
        adjust_dom_handles_after_remove(&self.root, &self.path, index, &removed);
        queue_mutation_record(
            self,
            "childList",
            None,
            None,
            Vec::new(),
            vec![removed.clone()],
        );
        custom_host::disconnected(removed.clone())?;
        Ok(Some(removed))
    }

    fn replace_child(
        &self,
        new_child: Node,
        old_child: Option<&DomHandle>,
    ) -> Result<Option<Node>, String> {
        let Some(old_child) = old_child else {
            return Ok(None);
        };
        if !Rc::ptr_eq(&self.root, &old_child.root)
            || old_child.path.len() != self.path.len() + 1
            || !old_child.path.starts_with(&self.path)
        {
            return Ok(None);
        }
        let Some(index) = old_child.path.last().copied() else {
            return Ok(None);
        };
        let added_nodes = added_nodes_for_record(&new_child);
        let added_count = added_nodes.len();
        let removed = {
            let mut root = self.root.borrow_mut();
            let Some(Node::Element(parent)) = get_node_mut(&mut root, &self.path) else {
                return Ok(None);
            };
            if index >= parent.children.len() {
                return Ok(None);
            }
            if let Node::Element(fragment) = new_child {
                if fragment.tag == "#document-fragment" {
                    let removed = parent.children.remove(index);
                    for child in fragment.children.into_iter().rev() {
                        parent.children.insert(index, child);
                    }
                    removed
                } else {
                    std::mem::replace(&mut parent.children[index], Node::Element(fragment))
                }
            } else {
                std::mem::replace(&mut parent.children[index], new_child)
            }
        };
        adjust_dom_handles_after_replace(&self.root, &self.path, index, added_count, &removed);
        queue_mutation_record(
            self,
            "childList",
            None,
            None,
            added_nodes,
            vec![removed.clone()],
        );
        custom_host::disconnected(removed.clone())?;
        let mut path = self.path.clone();
        path.push(index);
        self.connect_inserted_children(&path, added_count)?;
        Ok(Some(removed))
    }

    fn event_key(&self) -> String {
        format!("{:p}:{:?}", Rc::as_ptr(&self.root), self.path)
    }

    fn focus(&self) -> Result<(), String> {
        let next = self.event_key();
        let previous = FOCUSED_ELEMENT.with(|focused| focused.borrow().clone());
        if previous.as_deref() == Some(&next) {
            return Ok(());
        }
        if let Some(previous) = previous.and_then(|key| handle_by_event_key(&key)) {
            previous.dispatch_event(JsValue::String("blur".into()))?;
        }
        FOCUSED_ELEMENT.with(|focused| *focused.borrow_mut() = Some(next));
        self.dispatch_event(JsValue::String("focus".into()))?;
        Ok(())
    }

    fn blur(&self) -> Result<(), String> {
        let key = self.event_key();
        let was_focused = FOCUSED_ELEMENT.with(|focused| focused.borrow().as_deref() == Some(&key));
        if was_focused {
            FOCUSED_ELEMENT.with(|focused| *focused.borrow_mut() = None);
            self.dispatch_event(JsValue::String("blur".into()))?;
        }
        Ok(())
    }

    fn ancestors(&self) -> Vec<Element> {
        let root = self.root.borrow();
        let mut ancestors = Vec::new();
        let mut current: &Node = &root;
        for index in &self.path {
            if let Node::Element(el) = current {
                ancestors.push(el.clone());
                current = &el.children[*index];
            } else {
                break;
            }
        }
        ancestors
    }

    fn add_event_listener(&self, event_type: &str, listener: JsValue, capture: bool, once: bool) {
        EVENT_REGISTRY.with(|registry| {
            registry
                .borrow_mut()
                .entry(self.event_key())
                .or_default()
                .listeners
                .entry(event_type.into())
                .or_default()
                .push(RegisteredListener {
                    callback: listener,
                    capture,
                    once,
                });
        });
    }

    fn remove_event_listener(&self, event_type: &str, listener: &JsValue, capture: bool) {
        EVENT_REGISTRY.with(|registry| {
            if let Some(entry) = registry.borrow_mut().get_mut(&self.event_key()) {
                if let Some(list) = entry.listeners.get_mut(event_type) {
                    list.retain(|item| item.callback != *listener || item.capture != capture);
                }
            }
        });
    }

    fn set_handler(&self, name: &str, handler: JsValue) {
        EVENT_REGISTRY.with(|registry| {
            registry
                .borrow_mut()
                .entry(self.event_key())
                .or_default()
                .handlers
                .insert(name.into(), handler);
        });
    }

    fn dispatch_event(&self, event: JsValue) -> Result<JsValue, String> {
        let event_type = event_type(&event).unwrap_or_else(|| "event".into());
        let target = node_object(self.clone());
        let event = normalize_event(event, &event_type, target.clone(), target.clone());
        let path = self.event_path();
        event_path_host::install(&event, &path);

        for ancestor in path.iter().take(path.len().saturating_sub(1)) {
            set_event_position(&event, node_object(ancestor.clone()), 1);
            for listener in ancestor.listeners(&event_type, Some(true)) {
                call_registered_listener(ancestor, &event_type, listener, event.clone())?;
                if event_flag(&event, "__immediatePropagationStopped") {
                    return Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")));
                }
            }
            if event_flag(&event, "__propagationStopped") {
                return Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")));
            }
        }

        set_event_position(&event, target.clone(), 2);
        for listener in self.listeners(&event_type, Some(true)) {
            call_registered_listener(self, &event_type, listener, event.clone())?;
            if event_flag(&event, "__immediatePropagationStopped") {
                return Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")));
            }
        }
        for listener in self.listeners(&event_type, Some(false)) {
            call_registered_listener(self, &event_type, listener, event.clone())?;
            if event_flag(&event, "__immediatePropagationStopped") {
                return Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")));
            }
        }
        if let Some(handler) = self.handler(&format!("on{}", event_type)) {
            call_dom_listener(handler, target.clone(), event.clone())?;
        }
        if event_flag(&event, "__propagationStopped") {
            return Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")));
        }

        if event_flag(&event, "bubbles") {
            for ancestor in path.iter().take(path.len().saturating_sub(1)).rev() {
                set_event_position(&event, node_object(ancestor.clone()), 3);
                for listener in ancestor.listeners(&event_type, Some(false)) {
                    call_registered_listener(ancestor, &event_type, listener, event.clone())?;
                    if event_flag(&event, "__immediatePropagationStopped") {
                        return Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")));
                    }
                }
                if let Some(handler) = ancestor.handler(&format!("on{}", event_type)) {
                    call_dom_listener(handler, node_object(ancestor.clone()), event.clone())?;
                }
                if event_flag(&event, "__propagationStopped") {
                    return Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")));
                }
            }
        }

        set_event_position(&event, JsValue::Null, 0);
        let default_prevented = match &event {
            JsValue::Object(obj) => obj
                .borrow()
                .get("defaultPrevented")
                .is_some_and(JsValue::truthy),
            _ => false,
        };
        let allowed = !default_prevented && self.run_default_action(&event_type)?;
        Ok(JsValue::Bool(allowed))
    }

    fn listeners(&self, event_type: &str, capture: Option<bool>) -> Vec<RegisteredListener> {
        EVENT_REGISTRY.with(|registry| {
            registry
                .borrow()
                .get(&self.event_key())
                .and_then(|entry| entry.listeners.get(event_type).cloned())
                .map(|listeners| {
                    listeners
                        .into_iter()
                        .filter(|listener| {
                            capture.is_none_or(|capture| listener.capture == capture)
                        })
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    fn handler(&self, name: &str) -> Option<JsValue> {
        EVENT_REGISTRY.with(|registry| {
            registry
                .borrow()
                .get(&self.event_key())
                .and_then(|entry| entry.handlers.get(name).cloned())
        })
    }

    fn event_path(&self) -> Vec<DomHandle> {
        let mut out = Vec::new();
        for index in 0..=self.path.len() {
            out.push(DomHandle {
                root: self.root.clone(),
                path: self.path[..index].to_vec(),
            });
        }
        out
    }

    fn run_default_action(&self, event_type: &str) -> Result<bool, String> {
        if event_type != "click" {
            return Ok(true);
        }
        let Some(Node::Element(el)) = self.node() else {
            return Ok(true);
        };
        if el.tag == "input" {
            let input_type = el
                .attrs
                .get("type")
                .map(|ty| ty.to_ascii_lowercase())
                .unwrap_or_else(|| "text".into());
            if input_type == "checkbox" {
                self.set_checked_state(!el.attrs.contains_key("checked"));
                self.dispatch_event(JsValue::String("input".into()))?;
                self.dispatch_event(JsValue::String("change".into()))?;
            } else if input_type == "radio" {
                self.set_checked_state(true);
                self.dispatch_event(JsValue::String("input".into()))?;
                self.dispatch_event(JsValue::String("change".into()))?;
            } else if input_type == "submit" {
                if let Some(form) = self.closest_form() {
                    return Ok(submit_form(&form, true)?.truthy());
                }
            }
        } else if el.tag == "button"
            && el
                .attrs
                .get("type")
                .map(|ty| ty.eq_ignore_ascii_case("submit"))
                .unwrap_or(true)
        {
            if let Some(form) = self.closest_form() {
                return Ok(submit_form(&form, true)?.truthy());
            }
        }
        Ok(true)
    }

    fn set_checked_state(&self, checked: bool) {
        let Some(Node::Element(el)) = self.node() else {
            return;
        };
        let input_type = el
            .attrs
            .get("type")
            .map(|ty| ty.to_ascii_lowercase())
            .unwrap_or_else(|| "text".into());
        if checked && input_type == "radio" {
            if let Some(name) = el.attrs.get("name").cloned() {
                uncheck_radio_group(&self.root, &name, &self.path);
            }
        }
        self.with_node_mut(|node| {
            if let Node::Element(el) = node {
                if checked {
                    el.attrs.insert("checked".into(), String::new());
                } else {
                    el.attrs.remove("checked");
                }
            }
        });
    }

    fn input_value(&self) -> String {
        match self.node() {
            Some(Node::Element(el)) if el.tag == "textarea" => {
                text_content_raw(&Node::Element(el.clone()))
            }
            Some(Node::Element(el)) if selection_host::is_contenteditable(&el) => {
                text_content_raw(&Node::Element(el.clone()))
            }
            Some(Node::Element(el)) => el.attrs.get("value").cloned().unwrap_or_default(),
            _ => String::new(),
        }
    }

    fn set_input_value(&self, value: String) {
        let char_len = value.chars().count();
        let old_value = Some(self.input_value());
        let text_backed = matches!(
            self.node(),
            Some(Node::Element(el))
                if el.tag == "textarea" || selection_host::is_contenteditable(&el)
        );
        self.with_node_mut(|node| {
            if let Node::Element(el) = node {
                if el.tag == "textarea" || selection_host::is_contenteditable(el) {
                    el.children = vec![Node::Text(value)];
                } else {
                    el.attrs.insert("value".into(), value);
                }
            }
        });
        set_selection_for_handle(self, char_len, char_len);
        queue_mutation_record(
            self,
            if text_backed {
                "characterData"
            } else {
                "attributes"
            },
            (!text_backed).then_some("value".into()),
            old_value,
            Vec::new(),
            Vec::new(),
        );
    }

    fn insert_text_at_selection(&self, text: &str) -> Result<(), String> {
        if selection_host::replace_contenteditable_selection(self, text).is_some() {
            self.dispatch_event(JsValue::String("input".into()))?;
            return Ok(());
        }
        let current = self.input_value();
        let len = current.chars().count();
        let (mut start, mut end) = selection_for_handle(self);
        start = start.min(len);
        end = end.min(len);
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        let before = current.chars().take(start).collect::<String>();
        let after = current.chars().skip(end).collect::<String>();
        let next = format!("{}{}{}", before, text, after);
        let cursor = start + text.chars().count();
        self.set_input_value(next);
        set_selection_for_handle(self, cursor, cursor);
        self.dispatch_event(JsValue::String("input".into()))?;
        Ok(())
    }

    fn closest_form(&self) -> Option<DomHandle> {
        let mut path = self.path.clone();
        loop {
            let handle = DomHandle {
                root: self.root.clone(),
                path: path.clone(),
            };
            if matches!(handle.node(), Some(Node::Element(el)) if el.tag == "form") {
                return Some(handle);
            }
            path.pop()?;
        }
    }
}

fn install_property_setters(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    for prop in ["textContent", "innerText"] {
        let h = handle.clone();
        obj.insert(
            format!("__set:{}", prop),
            native(&format!("set_{}", prop), Some(1), move |args| {
                let text = args.first().unwrap_or(&JsValue::Undefined).display();
                let old_value = h.node().map(|node| text_content_raw(&node));
                h.with_node_mut(|node| match node {
                    Node::Text(existing) => *existing = text,
                    Node::Element(el) => el.children = vec![Node::Text(text)],
                });
                queue_mutation_record(&h, "characterData", None, old_value, Vec::new(), Vec::new());
                Ok(JsValue::Undefined)
            }),
        );

        for prop in ["onclick", "oninput", "onchange", "onsubmit"] {
            let h = handle.clone();
            obj.insert(
                format!("__set:{}", prop),
                native(&format!("set_{}", prop), Some(1), move |args| {
                    h.set_handler(prop, args.first().cloned().unwrap_or(JsValue::Undefined));
                    Ok(JsValue::Undefined)
                }),
            );
        }
    }

    let h = handle.clone();
    obj.insert(
        "__set:id".into(),
        native("set_id", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            h.with_node_mut(|node| {
                if let Node::Element(el) = node {
                    el.attrs.insert("id".into(), value);
                }
            });
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    obj.insert(
        "__set:className".into(),
        native("set_className", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            h.with_node_mut(|node| {
                if let Node::Element(el) = node {
                    el.attrs.insert("class".into(), value);
                }
            });
            Ok(JsValue::Undefined)
        }),
    );

    let h = handle.clone();
    obj.insert(
        "__set:value".into(),
        native("set_value", Some(1), move |args| {
            let value = args.first().unwrap_or(&JsValue::Undefined).display();
            h.set_input_value(value);
            Ok(JsValue::Undefined)
        }),
    );

    if matches!(handle.node(), Some(Node::Element(el)) if el.tag == "canvas") {
        for prop in ["width", "height"] {
            let h = handle.clone();
            obj.insert(
                format!("__set:{}", prop),
                native(&format!("set_canvas_{}", prop), Some(1), move |args| {
                    let value = canvas_host::dimension_value(args.first());
                    canvas_host::set_dimension(&h, prop, value);
                    Ok(JsValue::Number(value as f64))
                }),
            );
        }
    }

    let h = handle.clone();
    obj.insert(
        "__set:selectionStart".into(),
        native("set_selectionStart", Some(1), move |args| {
            let start = args.first().map(selection_index).unwrap_or(0);
            let (_, end) = selection_for_handle(&h);
            set_selection_for_handle(&h, start, end.max(start));
            Ok(JsValue::Number(start as f64))
        }),
    );

    let h = handle.clone();
    obj.insert(
        "__set:selectionEnd".into(),
        native("set_selectionEnd", Some(1), move |args| {
            let end = args.first().map(selection_index).unwrap_or(0);
            let (start, _) = selection_for_handle(&h);
            set_selection_for_handle(&h, start.min(end), end);
            Ok(JsValue::Number(end as f64))
        }),
    );

    if matches!(handle.node(), Some(Node::Element(el)) if el.tag == "#document") {
        obj.insert(
            "__set:cookie".into(),
            native("set_cookie", Some(1), move |args| {
                let value = args.first().unwrap_or(&JsValue::Undefined).display();
                cookie_host::set_document_cookie(&value);
                Ok(JsValue::String(cookie_host::cookie_string()))
            }),
        );
    }

    let h = handle.clone();
    obj.insert(
        "__set:checked".into(),
        native("set_checked", Some(1), move |args| {
            let checked = args.first().unwrap_or(&JsValue::Undefined).truthy();
            let old_value = h.node().and_then(|node| match node {
                Node::Element(el) => el.attrs.get("checked").cloned(),
                Node::Text(_) => None,
            });
            h.set_checked_state(checked);
            queue_mutation_record(
                &h,
                "attributes",
                Some("checked".into()),
                old_value,
                Vec::new(),
                Vec::new(),
            );
            Ok(JsValue::Undefined)
        }),
    );
}

fn dataset_object(handle: DomHandle, element: &Element) -> JsValue {
    let mut obj = HashMap::new();
    for (name, value) in &element.attrs {
        if let Some(prop) = data_attr_to_prop(name) {
            obj.insert(prop.clone(), JsValue::String(value.clone()));
            let h = handle.clone();
            let attr = prop_to_data_attr(&prop);
            obj.insert(
                format!("__set:{}", prop),
                native(&format!("set_dataset_{}", prop), Some(1), move |args| {
                    let value = args.first().unwrap_or(&JsValue::Undefined).display();
                    h.with_node_mut(|node| {
                        if let Node::Element(el) = node {
                            el.attrs.insert(attr.clone(), value);
                        }
                    });
                    Ok(JsValue::Undefined)
                }),
            );
        }
    }
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn data_attr_to_prop(name: &str) -> Option<String> {
    let rest = name.strip_prefix("data-")?;
    if rest.is_empty() {
        return None;
    }
    let mut out = String::new();
    let mut upper_next = false;
    for ch in rest.chars() {
        if ch == '-' {
            upper_next = true;
            continue;
        }
        if upper_next {
            out.push(ch.to_ascii_uppercase());
            upper_next = false;
        } else {
            out.push(ch);
        }
    }
    Some(out)
}

fn prop_to_data_attr(prop: &str) -> String {
    let mut out = String::from("data-");
    for ch in prop.chars() {
        if ch.is_ascii_uppercase() {
            out.push('-');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn js_value_to_node(value: &JsValue) -> Node {
    if let Some(handle) = dom_handle_from_value(value) {
        if let Some(node) = handle.node() {
            return node;
        }
    }
    match value {
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            if let Some(JsValue::String(tag)) = obj.get("nodeName") {
                if tag != "#text" && tag != "#document" {
                    let mut attrs = HashMap::new();
                    if let Some(JsValue::String(id)) = obj.get("id") {
                        if !id.is_empty() {
                            attrs.insert("id".into(), id.clone());
                        }
                    }
                    if let Some(JsValue::String(class_name)) = obj.get("className") {
                        if !class_name.is_empty() {
                            attrs.insert("class".into(), class_name.clone());
                        }
                    }
                    let text = obj
                        .get("textContent")
                        .map(JsValue::display)
                        .unwrap_or_default();
                    return Node::Element(Element {
                        tag: tag.to_ascii_lowercase(),
                        attrs,
                        children: if text.is_empty() {
                            Vec::new()
                        } else {
                            vec![Node::Text(text)]
                        },
                    });
                }
            }
            Node::Text(value.display())
        }
        _ => Node::Text(value.display()),
    }
}

fn detached_node_object(node: Node) -> JsValue {
    node_object(DomHandle {
        root: Rc::new(RefCell::new(node)),
        path: Vec::new(),
    })
}

fn current_dom_handle(object: &OptionalJsObjectRef, fallback: &DomHandle) -> DomHandle {
    object
        .borrow()
        .as_ref()
        .map(|object| JsValue::Object(object.clone()))
        .and_then(|value| dom_handle_from_value(&value))
        .unwrap_or_else(|| fallback.clone())
}

fn clone_node(node: &Node, deep: bool) -> Node {
    match node {
        Node::Text(text) => Node::Text(text.clone()),
        Node::Element(el) => Node::Element(Element {
            tag: el.tag.clone(),
            attrs: el.attrs.clone(),
            children: if deep {
                el.children.clone()
            } else {
                Vec::new()
            },
        }),
    }
}

fn adopted_node_for_insert(
    value: &JsValue,
    target_parent: &DomHandle,
) -> Result<(Node, Option<DomHandle>), String> {
    let Some(handle) = dom_handle_from_value(value) else {
        return Ok((js_value_to_node(value), None));
    };
    reject_insert_into_self_or_descendant(&handle, target_parent)?;
    if is_document_fragment(&handle) {
        return Ok((take_fragment_for_insert(&handle)?, Some(handle)));
    }
    if let Some(parent) = handle.parent() {
        if let Some(node) = parent.remove_child(Some(&handle))? {
            let current = dom_handle_from_value(value).unwrap_or(handle);
            return Ok((node, Some(current)));
        }
    }
    let node = handle
        .node()
        .ok_or_else(|| "Cannot insert a missing DOM node".to_string())?;
    Ok((node, Some(handle)))
}

fn reject_insert_into_self_or_descendant(
    handle: &DomHandle,
    target_parent: &DomHandle,
) -> Result<(), String> {
    if Rc::ptr_eq(&handle.root, &target_parent.root) && target_parent.path.starts_with(&handle.path)
    {
        return Err("Cannot insert a DOM node into itself or its descendant".into());
    }
    Ok(())
}

fn is_document_fragment(handle: &DomHandle) -> bool {
    matches!(handle.node(), Some(Node::Element(el)) if el.tag == "#document-fragment")
}

fn take_fragment_for_insert(handle: &DomHandle) -> Result<Node, String> {
    let mut root = handle.root.borrow_mut();
    let Some(Node::Element(fragment)) = get_node_mut(&mut root, &handle.path) else {
        return Err("Cannot insert a missing DocumentFragment".into());
    };
    let children = std::mem::take(&mut fragment.children);
    Ok(Node::Element(Element {
        tag: fragment.tag.clone(),
        attrs: fragment.attrs.clone(),
        children,
    }))
}

fn same_optional_dom_handle(left: Option<&DomHandle>, right: Option<&DomHandle>) -> bool {
    matches!((left, right), (Some(left), Some(right)) if Rc::ptr_eq(&left.root, &right.root) && left.path == right.path)
}

fn added_nodes_for_record(node: &Node) -> Vec<Node> {
    match node {
        Node::Element(el) if el.tag == "#document-fragment" => el.children.clone(),
        node => vec![node.clone()],
    }
}

fn adjust_dom_handles_after_insert(
    root: &Rc<RefCell<Node>>,
    parent_path: &[usize],
    index: usize,
    count: usize,
) {
    if count > 0 {
        shift_dom_handles(root, parent_path, index, count as isize, true);
    }
}

fn adjust_dom_handles_after_remove(
    root: &Rc<RefCell<Node>>,
    parent_path: &[usize],
    index: usize,
    removed: &Node,
) {
    detach_removed_dom_handles(root, parent_path, index, removed);
    shift_dom_handles(root, parent_path, index, -1, false);
}

fn adjust_dom_handles_after_replace(
    root: &Rc<RefCell<Node>>,
    parent_path: &[usize],
    index: usize,
    added_count: usize,
    removed: &Node,
) {
    detach_removed_dom_handles(root, parent_path, index, removed);
    let delta = added_count as isize - 1;
    if delta != 0 {
        shift_dom_handles(root, parent_path, index, delta, false);
    }
}

fn detach_removed_dom_handles(
    root: &Rc<RefCell<Node>>,
    parent_path: &[usize],
    index: usize,
    removed: &Node,
) {
    let mut removed_path = parent_path.to_vec();
    removed_path.push(index);
    let detached_root = Rc::new(RefCell::new(removed.clone()));
    DOM_HANDLE_REGISTRY.with(|registry| {
        for handle in registry.borrow_mut().values_mut() {
            if Rc::ptr_eq(root, &handle.root) && handle.path.starts_with(&removed_path) {
                let old_key = handle.event_key();
                handle.root = detached_root.clone();
                handle.path = handle.path[removed_path.len()..].to_vec();
                rekey_event_entry(&old_key, &handle.event_key());
            }
        }
    });
}

fn shift_dom_handles(
    root: &Rc<RefCell<Node>>,
    parent_path: &[usize],
    index: usize,
    delta: isize,
    include_index: bool,
) {
    DOM_HANDLE_REGISTRY.with(|registry| {
        for handle in registry.borrow_mut().values_mut() {
            let depth = parent_path.len();
            if !Rc::ptr_eq(root, &handle.root)
                || handle.path.len() <= depth
                || !handle.path.starts_with(parent_path)
            {
                continue;
            }
            let slot = handle.path[depth];
            if slot > index || (include_index && slot >= index) {
                let old_key = handle.event_key();
                handle.path[depth] = ((slot as isize) + delta).max(0) as usize;
                rekey_event_entry(&old_key, &handle.event_key());
            }
        }
    });
}

fn rekey_event_entry(old_key: &str, new_key: &str) {
    if old_key == new_key {
        return;
    }
    EVENT_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(entry) = registry.remove(old_key) else {
            return;
        };
        let target = registry.entry(new_key.into()).or_default();
        for (event_type, mut listeners) in entry.listeners {
            target
                .listeners
                .entry(event_type)
                .or_default()
                .append(&mut listeners);
        }
        target.handlers.extend(entry.handlers);
    });
    FOCUSED_ELEMENT.with(|focused| {
        if focused.borrow().as_deref() == Some(old_key) {
            *focused.borrow_mut() = Some(new_key.into());
        }
    });
}

fn reattach_inserted_dom_handles(
    source: Option<&DomHandle>,
    target_root: &Rc<RefCell<Node>>,
    target_path: &[usize],
) {
    let Some(source) = source else {
        return;
    };
    if Rc::ptr_eq(&source.root, target_root) {
        return;
    }
    let source_root = source.root.clone();
    let source_path = source.path.clone();
    let source_is_fragment =
        matches!(source.node(), Some(Node::Element(el)) if el.tag == "#document-fragment");
    DOM_HANDLE_REGISTRY.with(|registry| {
        for handle in registry.borrow_mut().values_mut() {
            if !Rc::ptr_eq(&handle.root, &source_root) || !handle.path.starts_with(&source_path) {
                continue;
            }
            if source_is_fragment && handle.path.len() == source_path.len() {
                continue;
            }
            let old_key = handle.event_key();
            let suffix_start = if source_is_fragment {
                source_path.len() + 1
            } else {
                source_path.len()
            };
            let target_start = if source_is_fragment {
                let mut path = target_path.to_vec();
                if let (Some(last), Some(child_index)) =
                    (path.last_mut(), handle.path.get(source_path.len()))
                {
                    *last += *child_index;
                }
                path
            } else {
                target_path.to_vec()
            };
            let suffix = handle.path[suffix_start..].to_vec();
            handle.root = target_root.clone();
            handle.path = target_start.into_iter().chain(suffix).collect();
            rekey_event_entry(&old_key, &handle.event_key());
        }
    });
}

fn handle_by_event_key(key: &str) -> Option<DomHandle> {
    DOM_HANDLE_REGISTRY.with(|registry| {
        registry
            .borrow()
            .values()
            .find(|handle| handle.event_key() == key)
            .cloned()
    })
}

fn get_node<'a>(node: &'a Node, path: &[usize]) -> Option<&'a Node> {
    if path.is_empty() {
        return Some(node);
    }
    match node {
        Node::Element(el) => el
            .children
            .get(path[0])
            .and_then(|child| get_node(child, &path[1..])),
        Node::Text(_) => None,
    }
}

fn get_node_mut<'a>(node: &'a mut Node, path: &[usize]) -> Option<&'a mut Node> {
    if path.is_empty() {
        return Some(node);
    }
    match node {
        Node::Element(el) => el
            .children
            .get_mut(path[0])
            .and_then(|child| get_node_mut(child, &path[1..])),
        Node::Text(_) => None,
    }
}

fn call_dom_listener(listener: JsValue, this_value: JsValue, event: JsValue) -> Result<(), String> {
    js::call_function_with_this(listener, this_value, std::slice::from_ref(&event))?;
    Ok(())
}

fn call_registered_listener(
    handle: &DomHandle,
    event_type: &str,
    listener: RegisteredListener,
    event: JsValue,
) -> Result<(), String> {
    call_dom_listener(
        listener.callback.clone(),
        node_object(handle.clone()),
        event,
    )?;
    if listener.once {
        handle.remove_event_listener(event_type, &listener.callback, listener.capture);
    }
    Ok(())
}

fn event_type(event: &JsValue) -> Option<String> {
    match event {
        JsValue::Object(obj) => obj.borrow().get("type").map(JsValue::display),
        JsValue::String(s) => Some(s.clone()),
        _ => None,
    }
}

fn event_listener_options(value: Option<&JsValue>) -> (bool, bool) {
    match value {
        Some(JsValue::Bool(capture)) => (*capture, false),
        Some(JsValue::Object(obj)) => {
            let obj = obj.borrow();
            (
                obj.get("capture").is_some_and(JsValue::truthy),
                obj.get("once").is_some_and(JsValue::truthy),
            )
        }
        _ => (false, false),
    }
}

fn normalize_event(
    event: JsValue,
    event_type: &str,
    target: JsValue,
    current_target: JsValue,
) -> JsValue {
    let event_ref = match event {
        JsValue::Object(obj) => obj,
        _ => Rc::new(RefCell::new(HashMap::new())),
    };
    {
        let mut map = event_ref.borrow_mut();
        map.insert("type".into(), JsValue::String(event_type.into()));
        map.insert("target".into(), target);
        map.insert("currentTarget".into(), current_target);
        map.entry("bubbles".into()).or_insert(JsValue::Bool(true));
        map.entry("cancelable".into())
            .or_insert(JsValue::Bool(true));
        map.entry("eventPhase".into())
            .or_insert(JsValue::Number(0.0));
        map.entry("defaultPrevented".into())
            .or_insert(JsValue::Bool(false));
    }
    let event_for_prevent = event_ref.clone();
    event_ref.borrow_mut().insert(
        "preventDefault".into(),
        native("preventDefault", Some(0), move |_| {
            if event_for_prevent
                .borrow()
                .get("cancelable")
                .is_none_or(JsValue::truthy)
            {
                event_for_prevent
                    .borrow_mut()
                    .insert("defaultPrevented".into(), JsValue::Bool(true));
            }
            Ok(JsValue::Undefined)
        }),
    );
    let event_for_stop = event_ref.clone();
    event_ref.borrow_mut().insert(
        "stopPropagation".into(),
        native("stopPropagation", Some(0), move |_| {
            event_for_stop
                .borrow_mut()
                .insert("__propagationStopped".into(), JsValue::Bool(true));
            Ok(JsValue::Undefined)
        }),
    );
    let event_for_stop_immediate = event_ref.clone();
    event_ref.borrow_mut().insert(
        "stopImmediatePropagation".into(),
        native("stopImmediatePropagation", Some(0), move |_| {
            let mut event = event_for_stop_immediate.borrow_mut();
            event.insert("__propagationStopped".into(), JsValue::Bool(true));
            event.insert("__immediatePropagationStopped".into(), JsValue::Bool(true));
            Ok(JsValue::Undefined)
        }),
    );
    JsValue::Object(event_ref)
}

fn set_event_position(event: &JsValue, current_target: JsValue, phase: i64) {
    if let JsValue::Object(obj) = event {
        let mut obj = obj.borrow_mut();
        obj.insert("currentTarget".into(), current_target);
        obj.insert("eventPhase".into(), JsValue::Number(phase as f64));
        obj.insert("__immediatePropagationStopped".into(), JsValue::Bool(false));
    }
}

fn event_flag(event: &JsValue, name: &str) -> bool {
    matches!(event, JsValue::Object(obj) if obj.borrow().get(name).is_some_and(JsValue::truthy))
}

fn parse_location(href: &str) -> HashMap<String, JsValue> {
    let mut protocol = "http:".to_string();
    let mut host = "localhost".to_string();
    let mut pathname = "/".to_string();
    let mut search = String::new();
    let mut hash = String::new();
    if let Some((p, rest)) = href.split_once("://") {
        protocol = format!("{}:", p);
        let (before_hash, h) = rest.split_once('#').map_or((rest, ""), |(a, b)| (a, b));
        hash = if h.is_empty() {
            String::new()
        } else {
            format!("#{}", h)
        };
        let (before_query, q) = before_hash
            .split_once('?')
            .map_or((before_hash, ""), |(a, b)| (a, b));
        search = if q.is_empty() {
            String::new()
        } else {
            format!("?{}", q)
        };
        if let Some((parsed_host, path)) = before_query.split_once('/') {
            host = parsed_host.into();
            pathname = format!("/{}", path);
        } else {
            host = before_query.into();
            pathname = "/".into();
        }
    } else if href.starts_with('/') {
        let (before_hash, h) = href.split_once('#').map_or((href, ""), |(a, b)| (a, b));
        hash = if h.is_empty() {
            String::new()
        } else {
            format!("#{}", h)
        };
        let (before_query, q) = before_hash
            .split_once('?')
            .map_or((before_hash, ""), |(a, b)| (a, b));
        search = if q.is_empty() {
            String::new()
        } else {
            format!("?{}", q)
        };
        pathname = before_query.to_string();
    }
    let origin = format!("{}//{}", protocol, host);
    let mut obj = HashMap::new();
    for (k, v) in [
        ("href", href.to_string()),
        ("protocol", protocol),
        ("host", host.clone()),
        ("hostname", host),
        ("pathname", pathname),
        ("search", search),
        ("hash", hash),
        ("origin", origin),
    ] {
        obj.insert(k.into(), JsValue::String(v));
    }
    obj
}

fn native(
    name: &str,
    arity: Option<usize>,
    func: impl Fn(&[JsValue]) -> Result<JsValue, String> + 'static,
) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, arity, func)))
}

fn root_to_document(root: &Rc<RefCell<Node>>) -> browser::Document {
    match &*root.borrow() {
        Node::Element(el) if el.tag == "#document" => browser::Document {
            children: el.children.clone(),
        },
        node => browser::Document {
            children: vec![node.clone()],
        },
    }
}

fn register_dom_handle(handle: &DomHandle) -> String {
    let id = NEXT_DOM_HANDLE_ID.with(|next| {
        let mut next = next.borrow_mut();
        let id = *next;
        *next = next.saturating_add(1);
        id
    });
    let key = format!("h{}", id);
    DOM_HANDLE_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(key.clone(), handle.clone());
    });
    key
}

fn dom_handle_from_value(value: &JsValue) -> Option<DomHandle> {
    let JsValue::Object(obj) = value else {
        return None;
    };
    let Some(JsValue::String(id)) = obj.borrow().get("__domHandleId").cloned() else {
        return None;
    };
    DOM_HANDLE_REGISTRY.with(|registry| registry.borrow().get(&id).cloned())
}

fn path_key(path: &[usize]) -> String {
    path.iter()
        .map(|idx| idx.to_string())
        .collect::<Vec<_>>()
        .join(".")
}

fn dom_path_from_value(value: &JsValue) -> Vec<usize> {
    let JsValue::Object(obj) = value else {
        return Vec::new();
    };
    let Some(JsValue::String(path)) = obj.borrow().get("__domPath").cloned() else {
        return Vec::new();
    };
    if path.is_empty() {
        return Vec::new();
    }
    path.split('.')
        .filter_map(|part| part.parse::<usize>().ok())
        .collect()
}

fn layout_for_handle(handle: &DomHandle) -> browser::LayoutBox {
    let document = root_to_document(&handle.root);
    let css = LAYOUT_CSS.with(|c| c.borrow().clone());
    browser::layout_document(&document, &css, 80)
}

fn element_rect(handle: &DomHandle) -> (i64, i64, i64, i64) {
    let layout = layout_for_handle(handle);
    browser::find_layout_box_at_path(&layout, &handle.path)
        .map(|b| (b.x, b.y, b.width, b.height))
        .unwrap_or((0, 0, 0, 0))
}

fn element_offset_size(handle: &DomHandle) -> (i64, i64) {
    let (_, _, width, height) = element_rect(handle);
    (width, height)
}

fn rect_object(rect: &(i64, i64, i64, i64)) -> JsValue {
    let (x, y, width, height) = *rect;
    let mut obj = HashMap::new();
    let fields = [
        ("x", x),
        ("y", y),
        ("width", width),
        ("height", height),
        ("left", x),
        ("top", y),
        ("right", x + width),
        ("bottom", y + height),
    ];
    for (key, value) in fields {
        obj.insert(key.into(), JsValue::Number(value as f64));
    }
    obj.insert(
        "toJSON".into(),
        native("DOMRect.toJSON", Some(0), move |_| Ok(rect_json(fields))),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn rect_json(fields: [(&'static str, i64); 8]) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(
        fields
            .into_iter()
            .map(|(key, value)| (key.into(), JsValue::Number(value as f64)))
            .collect(),
    )))
}

fn computed_style_object(handle: &DomHandle) -> JsValue {
    let mut styles = HashMap::new();
    if let Some(styled) = styled_node_at_path(handle) {
        styles.extend(styled.styles);
    }
    if let Some(Node::Element(el)) = handle.node() {
        if let Some(inline) = el.attrs.get("style") {
            styles.extend(browser::parse_inline_style(inline));
        }
    }

    let (x, y, width, height) = element_rect(handle);
    styles
        .entry("display".into())
        .or_insert_with(|| "block".into());
    styles
        .entry("width".into())
        .or_insert_with(|| format!("{}px", width));
    styles
        .entry("height".into())
        .or_insert_with(|| format!("{}px", height));
    styles.insert("left".into(), format!("{}px", x));
    styles.insert("top".into(), format!("{}px", y));

    let object = Rc::new(RefCell::new(
        styles
            .into_iter()
            .map(|(k, v)| (k, JsValue::String(v)))
            .collect::<HashMap<_, _>>(),
    ));
    let object_for_get = object.clone();
    object.borrow_mut().insert(
        "getPropertyValue".into(),
        native(
            "CSSStyleDeclaration.getPropertyValue",
            Some(1),
            move |args| {
                let name = args.first().unwrap_or(&JsValue::Undefined).display();
                Ok(object_for_get
                    .borrow()
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| JsValue::String(String::new())))
            },
        ),
    );
    JsValue::Object(object)
}

fn styled_node_at_path(handle: &DomHandle) -> Option<browser::StyledNode> {
    let document = root_to_document(&handle.root);
    let css = LAYOUT_CSS.with(|c| c.borrow().clone());
    let styled = browser::computed_styles(&document, &css);
    let (first, rest) = handle.path.split_first()?;
    let mut current = styled.get(*first)?.clone();
    for index in rest {
        current = current.children.get(*index)?.clone();
    }
    Some(current)
}

fn accessibility_tree_object(document: &browser::Document) -> JsValue {
    let nodes = browser::build_accessibility_tree(document)
        .into_iter()
        .map(a11y_node_object)
        .collect::<Vec<_>>();
    JsValue::Array(Rc::new(RefCell::new(nodes)))
}

fn a11y_node_object(node: browser::A11yNode) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("role".into(), JsValue::String(node.role));
    obj.insert(
        "name".into(),
        node.name.map(JsValue::String).unwrap_or(JsValue::Null),
    );
    obj.insert("tag".into(), JsValue::String(node.tag));
    obj.insert("focusable".into(), JsValue::Bool(node.focusable));
    obj.insert("disabled".into(), JsValue::Bool(node.disabled));
    obj.insert(
        "children".into(),
        JsValue::Array(Rc::new(RefCell::new(
            node.children.into_iter().map(a11y_node_object).collect(),
        ))),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn form_data_object(handle: &DomHandle) -> JsValue {
    let entries = collect_form_entries(handle);
    let object = Rc::new(RefCell::new(HashMap::new()));
    for (name, value) in &entries {
        object
            .borrow_mut()
            .insert(name.clone(), JsValue::String(value.clone()));
    }
    object.borrow_mut().insert(
        "entries".into(),
        JsValue::Array(Rc::new(RefCell::new(
            entries
                .iter()
                .map(|(name, value)| {
                    JsValue::Array(Rc::new(RefCell::new(vec![
                        JsValue::String(name.clone()),
                        JsValue::String(value.clone()),
                    ])))
                })
                .collect(),
        ))),
    );
    let entries_for_get = entries.clone();
    object.borrow_mut().insert(
        "get".into(),
        native("FormData.get", Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(entries_for_get
                .iter()
                .find(|(n, _)| n == &name)
                .map(|(_, v)| JsValue::String(v.clone()))
                .unwrap_or(JsValue::Null))
        }),
    );
    JsValue::Object(object)
}

fn collect_form_entries(handle: &DomHandle) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    if let Some(node) = handle.node() {
        collect_form_entries_from_node(&node, &mut entries);
    }
    entries
}

fn collect_form_entries_from_node(node: &Node, entries: &mut Vec<(String, String)>) {
    let Node::Element(el) = node else {
        return;
    };
    if matches!(el.tag.as_str(), "input" | "textarea" | "select")
        && !el.attrs.contains_key("disabled")
    {
        if let Some(name) = el.attrs.get("name") {
            match el.tag.as_str() {
                "select" => collect_select_entries(el, name, entries),
                "textarea" => entries.push((name.clone(), browser::text_content(node))),
                "input" => collect_input_entry(el, name, entries),
                _ => {}
            }
        }
    }
    for child in &el.children {
        collect_form_entries_from_node(child, entries);
    }
}

fn collect_input_entry(el: &Element, name: &str, entries: &mut Vec<(String, String)>) {
    let input_type = el
        .attrs
        .get("type")
        .map(|ty| ty.to_ascii_lowercase())
        .unwrap_or_else(|| "text".into());
    let include = !matches!(input_type.as_str(), "submit" | "button" | "reset")
        && (!matches!(input_type.as_str(), "checkbox" | "radio")
            || el.attrs.contains_key("checked"));
    if include {
        let value = el.attrs.get("value").cloned().unwrap_or_else(|| {
            if input_type == "checkbox" {
                "on".into()
            } else {
                String::new()
            }
        });
        entries.push((name.to_string(), value));
    }
}

fn collect_select_entries(select: &Element, name: &str, entries: &mut Vec<(String, String)>) {
    let mut options = Vec::new();
    collect_select_options(select, &mut options);
    if select.attrs.contains_key("multiple") {
        for option in options
            .into_iter()
            .filter(|option| option.attrs.contains_key("selected"))
        {
            entries.push((name.to_string(), option_form_value(option)));
        }
    } else if let Some(option) = options
        .iter()
        .copied()
        .find(|option| option.attrs.contains_key("selected"))
        .or_else(|| options.first().copied())
    {
        entries.push((name.to_string(), option_form_value(option)));
    }
}

fn collect_select_options<'a>(node: &'a Element, options: &mut Vec<&'a Element>) {
    for child in &node.children {
        let Node::Element(el) = child else {
            continue;
        };
        if el.tag == "option" {
            if !el.attrs.contains_key("disabled") {
                options.push(el);
            }
        } else {
            collect_select_options(el, options);
        }
    }
}

fn option_form_value(option: &Element) -> String {
    option.attrs.get("value").cloned().unwrap_or_else(|| {
        option
            .children
            .iter()
            .map(text_content_raw)
            .collect::<Vec<_>>()
            .join("")
    })
}

fn submit_form(handle: &DomHandle, dispatch_submit: bool) -> Result<JsValue, String> {
    if dispatch_submit
        && !handle
            .dispatch_event(JsValue::String("submit".into()))?
            .truthy()
    {
        return Ok(JsValue::Bool(false));
    }
    let mut obj = HashMap::new();
    if let Some(Node::Element(el)) = handle.node() {
        obj.insert(
            "action".into(),
            JsValue::String(el.attrs.get("action").cloned().unwrap_or_default()),
        );
        obj.insert(
            "method".into(),
            JsValue::String(
                el.attrs
                    .get("method")
                    .map(|m| m.to_ascii_lowercase())
                    .unwrap_or_else(|| "get".into()),
            ),
        );
    }
    obj.insert("data".into(), form_data_object(handle));
    Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
}

fn collect_inline_scripts(node: &Node) -> Vec<String> {
    let mut out = Vec::new();
    collect_scripts(node, &mut out);
    out
}

fn collect_scripts(node: &Node, out: &mut Vec<String>) {
    if let Node::Element(el) = node {
        if el.tag.eq_ignore_ascii_case("script") && !el.attrs.contains_key("src") {
            out.push(
                el.children
                    .iter()
                    .map(text_content_raw)
                    .collect::<Vec<_>>()
                    .join(""),
            );
        }
        for child in &el.children {
            collect_scripts(child, out);
        }
    }
}

fn find_by_id(root: &Rc<RefCell<Node>>, id: &str) -> Option<Vec<usize>> {
    find_path(
        &root.borrow(),
        &mut Vec::new(),
        &|node| matches!(node, Node::Element(el) if el.attrs.get("id").map(|s| s.as_str()) == Some(id)),
    )
}

fn find_by_selector(root: &Rc<RefCell<Node>>, selector: &str) -> Option<Vec<usize>> {
    all_by_selector(root, selector).into_iter().next()
}

fn all_by_selector(root: &Rc<RefCell<Node>>, selector: &str) -> Vec<Vec<usize>> {
    let document = root_to_document(root);
    let matches = browser::query_selector(&document, selector);
    if matches.is_empty() {
        return Vec::new();
    }
    let mut claimed = Vec::<Vec<usize>>::new();
    matches
        .iter()
        .filter_map(|matched| {
            find_unclaimed_browser_node(&root.borrow(), matched, &mut Vec::new(), &mut claimed)
        })
        .collect()
}

fn find_path(
    node: &Node,
    path: &mut Vec<usize>,
    pred: &impl Fn(&Node) -> bool,
) -> Option<Vec<usize>> {
    if pred(node) {
        return Some(path.clone());
    }
    if let Node::Element(el) = node {
        for (index, child) in el.children.iter().enumerate() {
            path.push(index);
            if let Some(found) = find_path(child, path, pred) {
                return Some(found);
            }
            path.pop();
        }
    }
    None
}

fn find_unclaimed_browser_node(
    node: &Node,
    matched: &Node,
    path: &mut Vec<usize>,
    claimed: &mut Vec<Vec<usize>>,
) -> Option<Vec<usize>> {
    if node_name(node) != "#document"
        && !claimed.iter().any(|existing| existing == path)
        && browser_node_eq(node, matched)
    {
        let found = path.clone();
        claimed.push(found.clone());
        return Some(found);
    }
    if let Node::Element(el) = node {
        for (index, child) in el.children.iter().enumerate() {
            path.push(index);
            if let Some(found) = find_unclaimed_browser_node(child, matched, path, claimed) {
                path.pop();
                return Some(found);
            }
            path.pop();
        }
    }
    None
}

fn browser_node_eq(left: &Node, right: &Node) -> bool {
    match (left, right) {
        (Node::Text(left), Node::Text(right)) => left == right,
        (Node::Element(left), Node::Element(right)) => {
            left.tag == right.tag
                && left.attrs == right.attrs
                && left.children.len() == right.children.len()
                && left
                    .children
                    .iter()
                    .zip(&right.children)
                    .all(|(left, right)| browser_node_eq(left, right))
        }
        _ => false,
    }
}

fn uncheck_radio_group(root: &Rc<RefCell<Node>>, name: &str, except_path: &[usize]) {
    fn visit(node: &mut Node, path: &mut Vec<usize>, name: &str, except_path: &[usize]) {
        if let Node::Element(el) = node {
            let is_same_group = el.tag == "input"
                && el
                    .attrs
                    .get("type")
                    .is_some_and(|ty| ty.eq_ignore_ascii_case("radio"))
                && el
                    .attrs
                    .get("name")
                    .is_some_and(|candidate| candidate == name)
                && path.as_slice() != except_path;
            if is_same_group {
                el.attrs.remove("checked");
            }
            for (index, child) in el.children.iter_mut().enumerate() {
                path.push(index);
                visit(child, path, name, except_path);
                path.pop();
            }
        }
    }

    visit(&mut root.borrow_mut(), &mut Vec::new(), name, except_path);
}

fn node_name(node: &Node) -> String {
    match node {
        Node::Element(el) => el.tag.clone(),
        Node::Text(_) => "#text".into(),
    }
}

fn children_collection(handle: &DomHandle, node: &Node, kind: &'static str) -> JsValue {
    let len = match node {
        Node::Element(el) => el.children.len(),
        Node::Text(_) => 0,
    };
    let handles = (0..len)
        .map(|index| {
            let mut path = handle.path.clone();
            path.push(index);
            DomHandle {
                root: handle.root.clone(),
                path,
            }
        })
        .collect();
    lazy_dom_collection(kind, handles)
}

fn lazy_dom_collection(kind: &'static str, handles: Vec<DomHandle>) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("length".into(), JsValue::Number(handles.len() as f64));
    for (index, handle) in handles.iter().cloned().enumerate() {
        obj.insert(
            format!("__get:{index}"),
            native(&format!("{kind}.{index}"), Some(0), move |_| {
                Ok(node_object(handle.clone()))
            }),
        );
    }
    let object = Rc::new(RefCell::new(obj));
    let collection = JsValue::Object(object.clone());
    let item_handles = handles.clone();
    object.borrow_mut().insert(
        "item".into(),
        native(&format!("{kind}.item"), Some(1), move |args| {
            Ok(item_handles
                .get(collection_index(args.first()))
                .cloned()
                .map(node_object)
                .unwrap_or(JsValue::Null))
        }),
    );
    let weak = Rc::downgrade(&object);
    let each_handles = handles.clone();
    object.borrow_mut().insert(
        "forEach".into(),
        native(&format!("{kind}.forEach"), None, move |args| {
            let callback = args
                .first()
                .cloned()
                .ok_or_else(|| format!("{kind}.forEach: expected callback"))?;
            let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let collection = weak
                .upgrade()
                .map(JsValue::Object)
                .unwrap_or(JsValue::Undefined);
            for (index, handle) in each_handles.iter().cloned().enumerate() {
                js::call_function_with_this(
                    callback.clone(),
                    this_arg.clone(),
                    &[
                        node_object(handle),
                        JsValue::Number(index as f64),
                        collection.clone(),
                    ],
                )?;
            }
            Ok(JsValue::Undefined)
        }),
    );
    if kind == "HTMLCollection" {
        install_lazy_html_collection_named_items(&object, &handles, kind);
    }
    collection
}

fn dom_collection(kind: &'static str, values: Vec<JsValue>) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("length".into(), JsValue::Number(values.len() as f64));
    for (index, value) in values.iter().enumerate() {
        obj.insert(index.to_string(), value.clone());
    }
    let object = Rc::new(RefCell::new(obj));
    let collection = JsValue::Object(object.clone());
    let item_values = values.clone();
    object.borrow_mut().insert(
        "item".into(),
        native(&format!("{kind}.item"), Some(1), move |args| {
            Ok(item_values
                .get(collection_index(args.first()))
                .cloned()
                .unwrap_or(JsValue::Null))
        }),
    );
    let weak = Rc::downgrade(&object);
    let for_each_values = values.clone();
    object.borrow_mut().insert(
        "forEach".into(),
        native(&format!("{kind}.forEach"), None, move |args| {
            let callback = args
                .first()
                .cloned()
                .ok_or_else(|| format!("{kind}.forEach: expected callback"))?;
            let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let collection = weak
                .upgrade()
                .map(JsValue::Object)
                .unwrap_or(JsValue::Undefined);
            for (index, value) in for_each_values.iter().cloned().enumerate() {
                js::call_function_with_this(
                    callback.clone(),
                    this_arg.clone(),
                    &[value, JsValue::Number(index as f64), collection.clone()],
                )?;
            }
            Ok(JsValue::Undefined)
        }),
    );
    if kind == "HTMLCollection" {
        install_html_collection_named_items(&object, &values, kind);
    }
    collection
}

fn install_lazy_html_collection_named_items(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    handles: &[DomHandle],
    kind: &'static str,
) {
    let named_handles = handles.to_vec();
    object.borrow_mut().insert(
        "namedItem".into(),
        native(&format!("{kind}.namedItem"), Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(named_handles
                .iter()
                .find(|handle| html_collection_handle_name_matches(handle, &name))
                .cloned()
                .map(node_object)
                .unwrap_or(JsValue::Null))
        }),
    );
}

fn html_collection_handle_name_matches(handle: &DomHandle, name: &str) -> bool {
    html_collection_names_from_handle(handle)
        .iter()
        .any(|item| item == name)
}

fn html_collection_names_from_handle(handle: &DomHandle) -> Vec<String> {
    let Some(Node::Element(el)) = handle.node() else {
        return Vec::new();
    };
    ["id", "name"]
        .iter()
        .filter_map(|attr| el.attrs.get(*attr))
        .filter(|name| !name.is_empty())
        .cloned()
        .collect()
}

fn install_html_collection_named_items(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    values: &[JsValue],
    kind: &'static str,
) {
    let named_values = values.to_vec();
    object.borrow_mut().insert(
        "namedItem".into(),
        native(&format!("{kind}.namedItem"), Some(1), move |args| {
            let name = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(named_values
                .iter()
                .find(|value| html_collection_name_matches(value, &name))
                .cloned()
                .unwrap_or(JsValue::Null))
        }),
    );
    for value in values {
        for name in html_collection_names(value) {
            object
                .borrow_mut()
                .entry(name)
                .or_insert_with(|| value.clone());
        }
    }
}

fn html_collection_name_matches(value: &JsValue, name: &str) -> bool {
    html_collection_names(value).iter().any(|item| item == name)
}

fn html_collection_names(value: &JsValue) -> Vec<String> {
    let Some(Node::Element(el)) = dom_handle_from_value(value).and_then(|handle| handle.node())
    else {
        return Vec::new();
    };
    ["id", "name"]
        .iter()
        .filter_map(|attr| el.attrs.get(*attr))
        .filter(|name| !name.is_empty())
        .cloned()
        .collect()
}

fn collection_index(value: Option<&JsValue>) -> usize {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() && *n >= 0.0 => n.trunc() as usize,
        other => other.display().parse().unwrap_or(usize::MAX),
    }
}

fn optional_node_ref_object(handle: Option<DomHandle>) -> JsValue {
    handle.map(node_reference_object).unwrap_or(JsValue::Null)
}

fn node_reference_object(handle: DomHandle) -> JsValue {
    let node = handle.node().unwrap_or(Node::Text(String::new()));
    let handle_id = register_dom_handle(&handle);
    let mut obj = HashMap::new();
    obj.insert("__domHandleId".into(), JsValue::String(handle_id));
    obj.insert("__domPath".into(), JsValue::String(path_key(&handle.path)));
    obj.insert(
        "nodeType".into(),
        JsValue::Number(if matches!(node, Node::Text(_)) {
            3.0
        } else if node_name(&node) == "#document" {
            9.0
        } else if node_name(&node) == "#document-fragment" {
            11.0
        } else {
            1.0
        }),
    );
    obj.insert("nodeName".into(), JsValue::String(node_name(&node)));
    install_node_text_getters(&mut obj, &handle);
    if let Node::Element(el) = &node {
        obj.insert(
            "id".into(),
            JsValue::String(el.attrs.get("id").cloned().unwrap_or_default()),
        );
        obj.insert(
            "className".into(),
            JsValue::String(el.attrs.get("class").cloned().unwrap_or_default()),
        );
        obj.insert(
            "tagName".into(),
            JsValue::String(el.tag.to_ascii_uppercase()),
        );
    }
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn document_reference_object(root: Rc<RefCell<Node>>) -> JsValue {
    let handle = DomHandle {
        root,
        path: Vec::new(),
    };
    let mut obj = HashMap::new();
    obj.insert(
        "__domHandleId".into(),
        JsValue::String(register_dom_handle(&handle)),
    );
    obj.insert("__domPath".into(), JsValue::String(String::new()));
    obj.insert("nodeType".into(), JsValue::Number(9.0));
    obj.insert("nodeName".into(), JsValue::String("#document".into()));
    obj.insert(
        "createElement".into(),
        native("document.createElement", Some(1), move |args| {
            let tag = args
                .first()
                .unwrap_or(&JsValue::Undefined)
                .display()
                .to_ascii_lowercase();
            detached_element_object(tag)
        }),
    );
    obj.insert(
        "createElementNS".into(),
        native("document.createElementNS", Some(2), move |args| {
            let namespace = namespace_value(args.first());
            let qualified = args.get(1).unwrap_or(&JsValue::Undefined).display();
            detached_namespaced_element(namespace, qualified)
        }),
    );
    obj.insert(
        "createTextNode".into(),
        native("document.createTextNode", Some(1), move |args| {
            let text = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(detached_node_object(Node::Text(text)))
        }),
    );
    obj.insert(
        "execCommand".into(),
        native("document.execCommand", None, |_| Ok(JsValue::Bool(false))),
    );
    obj.insert(
        "queryCommandSupported".into(),
        native("document.queryCommandSupported", None, |_| {
            Ok(JsValue::Bool(false))
        }),
    );
    obj.insert(
        "queryCommandEnabled".into(),
        native("document.queryCommandEnabled", None, |_| {
            Ok(JsValue::Bool(false))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "addEventListener".into(),
        native("document.addEventListener", None, move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let (capture, once) = event_listener_options(args.get(2));
            h.add_event_listener(&event_type, listener, capture, once);
            Ok(JsValue::Undefined)
        }),
    );
    let h = handle.clone();
    obj.insert(
        "removeEventListener".into(),
        native("document.removeEventListener", None, move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let (capture, _) = event_listener_options(args.get(2));
            h.remove_event_listener(&event_type, &listener, capture);
            Ok(JsValue::Undefined)
        }),
    );
    obj.insert(
        "dispatchEvent".into(),
        native("document.dispatchEvent", Some(1), move |args| {
            handle.dispatch_event(args.first().cloned().unwrap_or(JsValue::Undefined))
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn detached_element_object(tag: String) -> Result<JsValue, String> {
    let element = detached_node_object(Node::Element(Element {
        tag,
        attrs: HashMap::new(),
        children: Vec::new(),
    }));
    let tag = node_name(&js_value_to_node(&element));
    custom_host::construct_created(&tag, &element)?;
    Ok(element)
}

fn detached_namespaced_element(namespace: JsValue, qualified: String) -> Result<JsValue, String> {
    let (prefix, local) = split_qualified_name(&qualified);
    let element = detached_element_object(qualified.clone())?;
    let JsValue::Object(obj) = &element else {
        return Ok(element);
    };
    let node_name = namespaced_node_name(&namespace, &qualified);
    let mut obj = obj.borrow_mut();
    obj.insert("namespaceURI".into(), namespace);
    obj.insert("localName".into(), JsValue::String(local));
    obj.insert(
        "prefix".into(),
        prefix.map(JsValue::String).unwrap_or(JsValue::Null),
    );
    obj.insert("nodeName".into(), JsValue::String(node_name.clone()));
    obj.insert("tagName".into(), JsValue::String(node_name));
    drop(obj);
    Ok(element)
}

fn namespace_value(value: Option<&JsValue>) -> JsValue {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Null | JsValue::Undefined => JsValue::Null,
        other => JsValue::String(other.display()),
    }
}

fn split_qualified_name(name: &str) -> (Option<String>, String) {
    name.split_once(':')
        .map(|(prefix, local)| (Some(prefix.into()), local.into()))
        .unwrap_or_else(|| (None, name.into()))
}

fn namespaced_node_name(namespace: &JsValue, name: &str) -> String {
    if matches!(namespace, JsValue::String(ns) if ns == "http://www.w3.org/1999/xhtml") {
        name.to_ascii_uppercase()
    } else {
        name.into()
    }
}

fn selection_for_handle(handle: &DomHandle) -> (usize, usize) {
    let key = handle.event_key();
    INPUT_SELECTIONS.with(|selections| {
        selections.borrow().get(&key).copied().unwrap_or_else(|| {
            let len = handle.input_value().chars().count();
            (len, len)
        })
    })
}

fn set_selection_for_handle(handle: &DomHandle, start: usize, end: usize) {
    let len = handle.input_value().chars().count();
    INPUT_SELECTIONS.with(|selections| {
        selections
            .borrow_mut()
            .insert(handle.event_key(), (start.min(len), end.min(len)));
    });
}

fn selection_index(value: &JsValue) -> usize {
    match value {
        JsValue::Number(n) if n.is_finite() && *n >= 0.0 => *n as usize,
        other => other.display().parse().unwrap_or(0),
    }
}

fn text_content_raw(node: &Node) -> String {
    match node {
        Node::Text(text) => text.clone(),
        Node::Element(el) => el
            .children
            .iter()
            .map(text_content_raw)
            .collect::<Vec<_>>()
            .join(""),
    }
}

fn inner_html(node: &Node) -> String {
    match node {
        Node::Text(text) => escape_html(text),
        Node::Element(el) => el
            .children
            .iter()
            .map(outer_html)
            .collect::<Vec<_>>()
            .join(""),
    }
}

fn outer_html(node: &Node) -> String {
    match node {
        Node::Text(text) => escape_html(text),
        Node::Element(el) => {
            if el.tag == "#document" {
                return inner_html(node);
            }
            let mut out = String::new();
            out.push('<');
            out.push_str(&el.tag);
            for (k, v) in &el.attrs {
                out.push(' ');
                out.push_str(k);
                out.push_str("=\"");
                out.push_str(&escape_attr(v));
                out.push('"');
            }
            out.push('>');
            out.push_str(&inner_html(node));
            out.push_str("</");
            out.push_str(&el.tag);
            out.push('>');
            out
        }
    }
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn escape_attr(text: &str) -> String {
    escape_html(text).replace('"', "&quot;")
}

pub fn browser_run_scripts_to_value(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "browser_run_scripts: expected 1 arg, got {}",
            args.len()
        ));
    }
    let Value::Str(html) = &args[0] else {
        return Err(format!(
            "browser_run_scripts: expected str, got {}",
            args[0].type_name()
        ));
    };
    result_to_value(run_html_scripts(html)?)
}

pub fn browser_run_scripts_at_to_value(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "browser_run_scripts_at: expected 2 args, got {}",
            args.len()
        ));
    }
    let Value::Str(url) = &args[0] else {
        return Err(format!(
            "browser_run_scripts_at: url must be str, got {}",
            args[0].type_name()
        ));
    };
    let Value::Str(html) = &args[1] else {
        return Err(format!(
            "browser_run_scripts_at: html must be str, got {}",
            args[1].type_name()
        ));
    };
    result_to_value(run_html_scripts_at_url(html, url)?)
}

pub fn browser_eval_js_to_value(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "browser_eval_js: expected 2 args, got {}",
            args.len()
        ));
    }
    let Value::Str(html) = &args[0] else {
        return Err(format!(
            "browser_eval_js: html must be str, got {}",
            args[0].type_name()
        ));
    };
    let Value::Str(script) = &args[1] else {
        return Err(format!(
            "browser_eval_js: script must be str, got {}",
            args[1].type_name()
        ));
    };
    result_to_value(eval_with_dom(html, script)?)
}

fn result_to_value(result: BrowserJsResult) -> Result<Value, String> {
    let mut map = HashMap::new();
    map.insert("dom".into(), document_value(&result.document));
    map.insert("value".into(), js::js_to_tether(&result.value));
    map.insert("url".into(), Value::Str(Rc::new(result.state.url.clone())));
    map.insert(
        "network".into(),
        Value::List(Rc::new(RefCell::new(
            result
                .network
                .into_iter()
                .map(network_event_value)
                .collect(),
        ))),
    );
    map.insert(
        "console".into(),
        Value::List(Rc::new(RefCell::new(
            result
                .console
                .into_iter()
                .map(|s| Value::Str(Rc::new(s)))
                .collect(),
        ))),
    );
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn network_event_value(event: BrowserJsNetworkEvent) -> Value {
    let mut map = HashMap::new();
    map.insert("method".into(), Value::Str(Rc::new(event.method)));
    map.insert("url".into(), Value::Str(Rc::new(event.url)));
    map.insert(
        "status".into(),
        event
            .status
            .map(|status| Value::Int(status as i64))
            .unwrap_or(Value::Nil),
    );
    map.insert(
        "route_result".into(),
        event
            .route_result
            .map(|result| Value::Str(Rc::new(result)))
            .unwrap_or(Value::Nil),
    );
    Value::Map(Rc::new(RefCell::new(map)))
}

fn document_value(document: &browser::Document) -> Value {
    Value::List(Rc::new(RefCell::new(
        document.children.iter().map(node_value).collect(),
    )))
}

fn node_value(node: &Node) -> Value {
    let mut map = HashMap::new();
    match node {
        Node::Text(text) => {
            map.insert("type".into(), Value::Str(Rc::new("text".into())));
            map.insert("text".into(), Value::Str(Rc::new(text.clone())));
        }
        Node::Element(el) => {
            map.insert("type".into(), Value::Str(Rc::new("element".into())));
            map.insert("tag".into(), Value::Str(Rc::new(el.tag.clone())));
            map.insert(
                "attrs".into(),
                Value::Map(Rc::new(RefCell::new(
                    el.attrs
                        .iter()
                        .map(|(k, v)| (k.clone(), Value::Str(Rc::new(v.clone()))))
                        .collect(),
                ))),
            );
            map.insert(
                "children".into(),
                Value::List(Rc::new(RefCell::new(
                    el.children.iter().map(node_value).collect(),
                ))),
            );
        }
    }
    Value::Map(Rc::new(RefCell::new(map)))
}

fn child_element_count(node: &Node) -> usize {
    match node {
        Node::Element(el) => el
            .children
            .iter()
            .filter(|child| matches!(child, Node::Element(_)))
            .count(),
        Node::Text(_) => 0,
    }
}

// ── Fetch API ──────────────────────────────────────────────────────────

#[derive(Clone)]
struct FetchRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    aborted: bool,
}

fn install_fetch_binding(
    window: &mut HashMap<String, JsValue>,
    timers: Rc<RefCell<TimerQueue>>,
    route_handler: SharedBrowserJsRouteHandler,
    location: Rc<RefCell<HashMap<String, JsValue>>>,
) {
    window.insert(
        "fetch".into(),
        native("fetch", None, move |args| {
            let mut request = request_from_fetch_args(args);
            resolve_request_url(&mut request, &location);
            if request.aborted {
                return Ok(make_rejected_fetch_promise("AbortError", timers.clone()));
            }
            let response = match make_response(&request, &route_handler) {
                Ok(response) => response,
                Err(reason) => return Ok(make_rejected_fetch_promise(&reason, timers.clone())),
            };
            let mut promise_obj = HashMap::new();
            promise_obj.insert(
                "__promise_state".into(),
                JsValue::String("fulfilled".into()),
            );
            promise_obj.insert("__promise_value".into(), response.clone());

            let then_value = response.clone();
            let then_queue = timers.clone();
            promise_obj.insert(
                "then".into(),
                native("Promise.then", Some(1), move |args| {
                    let on_fulfilled = args.first().cloned().unwrap_or(JsValue::Undefined);
                    if !matches!(on_fulfilled, JsValue::Undefined) {
                        then_queue
                            .borrow_mut()
                            .microtasks
                            .push_back(ScheduledCallback {
                                id: 0,
                                callback: on_fulfilled,
                                args: vec![then_value.clone()],
                                this_value: JsValue::Undefined,
                            });
                        let mut next = HashMap::new();
                        next.insert(
                            "__promise_state".into(),
                            JsValue::String("fulfilled".into()),
                        );
                        next.insert("__promise_value".into(), JsValue::Undefined);
                        install_then_catch_simple(&mut next, JsValue::Undefined);
                        return Ok(JsValue::Object(Rc::new(RefCell::new(next))));
                    }
                    Ok(JsValue::Object(Rc::new(RefCell::new(promise_obj_clone(
                        &then_value,
                    )))))
                }),
            );

            let catch_value = response.clone();
            promise_obj.insert(
                "catch".into(),
                native("Promise.catch", Some(1), move |_args| {
                    let mut next = HashMap::new();
                    next.insert(
                        "__promise_state".into(),
                        JsValue::String("fulfilled".into()),
                    );
                    next.insert("__promise_value".into(), catch_value.clone());
                    Ok(JsValue::Object(Rc::new(RefCell::new(next))))
                }),
            );

            Ok(JsValue::Object(Rc::new(RefCell::new(promise_obj))))
        }),
    );
}

fn resolve_request_url(
    request: &mut FetchRequest,
    location: &Rc<RefCell<HashMap<String, JsValue>>>,
) {
    let initiator = location_href(location);
    network_cookie_host::set_document_url(&initiator);
    request.url = resolve_url(&request.url, Some(&initiator));
    network_cookie_host::append_request_header(&mut request.headers, &request.url, &initiator);
}

fn promise_obj_clone(value: &JsValue) -> HashMap<String, JsValue> {
    let mut next = HashMap::new();
    next.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    next.insert("__promise_value".into(), value.clone());
    install_then_catch_simple(&mut next, value.clone());
    next
}

fn install_then_catch_simple(obj: &mut HashMap<String, JsValue>, fulfilled_value: JsValue) {
    let v = fulfilled_value.clone();
    obj.insert(
        "then".into(),
        native("Promise.then", Some(1), move |args| {
            let on_fulfilled = args.first().cloned().unwrap_or(JsValue::Undefined);
            if !matches!(on_fulfilled, JsValue::Undefined) {
                let result = js::call_function_with_this(
                    on_fulfilled,
                    JsValue::Undefined,
                    std::slice::from_ref(&v),
                )?;
                let mut next = HashMap::new();
                next.insert(
                    "__promise_state".into(),
                    JsValue::String("fulfilled".into()),
                );
                next.insert("__promise_value".into(), result.clone());
                install_then_catch_simple(&mut next, result);
                return Ok(JsValue::Object(Rc::new(RefCell::new(next))));
            }
            Ok(JsValue::Object(Rc::new(RefCell::new(promise_obj_clone(
                &v,
            )))))
        }),
    );
    obj.insert(
        "catch".into(),
        native("Promise.catch", Some(1), move |_args| {
            Ok(JsValue::Object(Rc::new(RefCell::new(promise_obj_clone(
                &fulfilled_value,
            )))))
        }),
    );
}

fn request_from_fetch_args(args: &[JsValue]) -> FetchRequest {
    let input = args.first().unwrap_or(&JsValue::Undefined);
    let init = args.get(1).unwrap_or(&JsValue::Undefined);
    let mut request = request_from_value(input);
    apply_request_init(&mut request, init);
    request
}

fn request_from_value(value: &JsValue) -> FetchRequest {
    match value {
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            let url = obj
                .get("url")
                .or_else(|| obj.get("href"))
                .map(JsValue::display)
                .unwrap_or_else(|| value.display());
            FetchRequest {
                url,
                method: obj
                    .get("method")
                    .map(JsValue::display)
                    .unwrap_or_else(|| "GET".into())
                    .to_ascii_uppercase(),
                headers: obj
                    .get("headers")
                    .map(headers_from_value)
                    .or_else(|| obj.get("__requestHeaders").map(headers_from_value))
                    .unwrap_or_default(),
                body: obj.get("body").and_then(request_body_from_value),
                aborted: obj.get("signal").is_some_and(signal_aborted),
            }
        }
        _ => FetchRequest {
            url: value.display(),
            method: "GET".into(),
            headers: Vec::new(),
            body: None,
            aborted: false,
        },
    }
}

fn apply_request_init(request: &mut FetchRequest, init: &JsValue) {
    let JsValue::Object(obj) = init else {
        return;
    };
    let obj = obj.borrow();
    if let Some(method) = obj.get("method") {
        request.method = method.display().to_ascii_uppercase();
    }
    if let Some(headers) = obj.get("headers") {
        request.headers = headers_from_value(headers);
    }
    if let Some(body) = obj.get("body") {
        request.body = Some(body.display());
    }
    if let Some(signal) = obj.get("signal") {
        request.aborted = signal_aborted(signal);
    }
}

fn signal_aborted(value: &JsValue) -> bool {
    matches!(value, JsValue::Object(obj) if obj.borrow().get("aborted").is_some_and(JsValue::truthy))
}

fn make_rejected_fetch_promise(reason: &str, timers: Rc<RefCell<TimerQueue>>) -> JsValue {
    let reason = JsValue::String(reason.into());
    let mut promise_obj = HashMap::new();
    promise_obj.insert("__promise_state".into(), JsValue::String("rejected".into()));
    promise_obj.insert("__promise_reason".into(), reason.clone());
    promise_obj.insert(
        "then".into(),
        native("Promise.then", None, move |_args| {
            Ok(JsValue::Object(Rc::new(RefCell::new(promise_obj_clone(
                &JsValue::Undefined,
            )))))
        }),
    );
    let catch_queue = timers;
    promise_obj.insert(
        "catch".into(),
        native("Promise.catch", Some(1), move |args| {
            let on_rejected = args.first().cloned().unwrap_or(JsValue::Undefined);
            if !matches!(on_rejected, JsValue::Undefined) {
                catch_queue
                    .borrow_mut()
                    .microtasks
                    .push_back(ScheduledCallback {
                        id: 0,
                        callback: on_rejected,
                        args: vec![reason.clone()],
                        this_value: JsValue::Undefined,
                    });
            }
            Ok(JsValue::Object(Rc::new(RefCell::new(promise_obj_clone(
                &JsValue::Undefined,
            )))))
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(promise_obj)))
}

struct FetchResponseParts {
    status: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
    route_result: Option<String>,
}

#[derive(Clone)]
struct ResponseFields {
    status: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
    url: String,
    method: Option<String>,
    body_used: bool,
    response_type: Option<String>,
}

fn make_response(
    request: &FetchRequest,
    route_handler: &SharedBrowserJsRouteHandler,
) -> Result<JsValue, String> {
    let parts = fetch_response_parts(request, route_handler)?;
    record_network_event(
        &request.method,
        &request.url,
        Some(parts.status),
        parts.route_result.clone(),
    );
    Ok(response_object(ResponseFields {
        status: parts.status,
        status_text: parts.status_text,
        headers: headers_with_default(parts.headers),
        body: parts.body,
        url: request.url.clone(),
        method: Some(request.method.clone()),
        body_used: request.body.as_ref().is_some_and(|body| !body.is_empty()),
        response_type: None,
    }))
}

fn response_object(fields: ResponseFields) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    let headers = headers_object(fields.headers.clone());
    {
        let mut obj = object.borrow_mut();
        obj.insert(
            "ok".into(),
            JsValue::Bool(fields.status >= 200 && fields.status < 300),
        );
        obj.insert("status".into(), JsValue::Number(fields.status as f64));
        obj.insert(
            "statusText".into(),
            JsValue::String(fields.status_text.clone()),
        );
        obj.insert("headers".into(), headers.clone());
        obj.insert("url".into(), JsValue::String(fields.url.clone()));
        obj.insert("bodyUsed".into(), JsValue::Bool(fields.body_used));
        if let Some(response_type) = &fields.response_type {
            obj.insert("type".into(), JsValue::String(response_type.clone()));
        }
        if let Some(method) = &fields.method {
            obj.insert("method".into(), JsValue::String(method.clone()));
        }
    }

    let text_body = fields.body.clone();
    let text_object = object.clone();
    object.borrow_mut().insert(
        "text".into(),
        native("Response.text", Some(0), move |_| {
            mark_response_body_used(&text_object);
            Ok(fulfilled_thenable(JsValue::String(text_body.clone())))
        }),
    );

    let json_body = fields.body.clone();
    let json_object = object.clone();
    object.borrow_mut().insert(
        "json".into(),
        native("Response.json", Some(0), move |_| {
            mark_response_body_used(&json_object);
            let parsed = parse_simple_json(&json_body);
            Ok(fulfilled_thenable(parsed))
        }),
    );

    let buffer_body = fields.body.clone();
    let buffer_object = object.clone();
    object.borrow_mut().insert(
        "arrayBuffer".into(),
        native("Response.arrayBuffer", Some(0), move |_| {
            mark_response_body_used(&buffer_object);
            Ok(fulfilled_thenable(byte_array_from_text(&buffer_body)))
        }),
    );

    let blob_body = fields.body.clone();
    let blob_object = object.clone();
    let blob_headers = headers.clone();
    object.borrow_mut().insert(
        "blob".into(),
        native("Response.blob", Some(0), move |_| {
            mark_response_body_used(&blob_object);
            let mime_type = body_mime_type(&headers_from_value(&blob_headers));
            Ok(fulfilled_thenable(blob_from_text_body(
                &blob_body, mime_type,
            )))
        }),
    );

    let clone_headers = headers;
    object.borrow_mut().insert(
        "clone".into(),
        native("Response.clone", Some(0), move |_| {
            let mut cloned = fields.clone();
            cloned.headers = headers_from_value(&clone_headers);
            cloned.body_used = false;
            Ok(response_object(cloned))
        }),
    );
    JsValue::Object(object)
}

fn fulfilled_thenable(value: JsValue) -> JsValue {
    let mut p = HashMap::new();
    p.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    p.insert("__promise_value".into(), value.clone());
    install_then_catch_simple(&mut p, value);
    JsValue::Object(Rc::new(RefCell::new(p)))
}

fn byte_array_from_text(text: &str) -> JsValue {
    byte_array_from_bytes(text.as_bytes())
}

fn byte_array_from_bytes(bytes: &[u8]) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        bytes
            .iter()
            .map(|byte| JsValue::Number(*byte as f64))
            .collect(),
    )))
}

fn blob_from_text_body(text: &str, mime_type: String) -> JsValue {
    let data = text.as_bytes().to_vec();
    let object = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut obj = object.borrow_mut();
        obj.insert("size".into(), JsValue::Number(data.len() as f64));
        obj.insert("type".into(), JsValue::String(mime_type.clone()));
        obj.insert("__blobBytes".into(), byte_array_from_bytes(&data));
        obj.insert("__blobType".into(), JsValue::String(mime_type));
    }

    let text_body = text.to_string();
    object.borrow_mut().insert(
        "text".into(),
        native("Blob.text", Some(0), move |_| {
            Ok(fulfilled_thenable(JsValue::String(text_body.clone())))
        }),
    );

    let buffer_data = data;
    object.borrow_mut().insert(
        "arrayBuffer".into(),
        native("Blob.arrayBuffer", Some(0), move |_| {
            Ok(fulfilled_thenable(byte_array_from_bytes(&buffer_data)))
        }),
    );

    JsValue::Object(object)
}

fn body_mime_type(headers: &[(String, String)]) -> String {
    headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("content-type"))
        .map(|(_, value)| value.clone())
        .unwrap_or_default()
}

fn mark_response_body_used(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    object
        .borrow_mut()
        .insert("bodyUsed".into(), JsValue::Bool(true));
}

fn fetch_response_parts(
    request: &FetchRequest,
    route_handler: &SharedBrowserJsRouteHandler,
) -> Result<FetchResponseParts, String> {
    let parts = match route_action_for(request, route_handler) {
        None | Some(BrowserJsRouteAction::PassThrough) => {
            live_response_parts(request).or_else(|_| Ok(default_response_parts(request, None)))
        }
        Some(BrowserJsRouteAction::Continue) => live_response_parts(request)
            .or_else(|_| Ok(default_response_parts(request, Some("continue")))),
        Some(BrowserJsRouteAction::Abort(reason)) => {
            record_network_event(&request.method, &request.url, None, Some("abort".into()));
            Err(reason)
        }
        Some(BrowserJsRouteAction::Blocked(reason)) => {
            record_network_event(&request.method, &request.url, None, Some("blocked".into()));
            Err(reason)
        }
        Some(BrowserJsRouteAction::Fulfill(response)) => Ok(FetchResponseParts {
            status: response.status,
            status_text: status_text(response.status).into(),
            headers: response.headers,
            body: response.body,
            route_result: Some("fulfill".into()),
        }),
    }?;
    network_cookie_host::apply_response_headers(&request.url, &parts.headers);
    Ok(parts)
}

fn live_response_parts(request: &FetchRequest) -> Result<FetchResponseParts, String> {
    if !should_live_fetch(&request.url) {
        return Err("synthetic fetch target".into());
    }
    let response = crate::http::client_request(
        &request.method,
        &request.url,
        request.body.as_deref(),
        &request.headers,
    )?;
    response_value_parts(response)
}

fn should_live_fetch(url: &str) -> bool {
    (url.starts_with("https://") || url.starts_with("http://"))
        && !url.contains("://localhost")
        && !url.contains("://127.0.0.1")
}

fn response_value_parts(value: Value) -> Result<FetchResponseParts, String> {
    let Value::Map(response) = value else {
        return Err("http response must be a map".into());
    };
    let response = response.borrow();
    let status = match response.get("status") {
        Some(Value::Int(status)) => *status as u16,
        _ => 200,
    };
    let status_text = match response.get("reason") {
        Some(Value::Str(reason)) => reason.to_string(),
        _ => status_text(status).into(),
    };
    let headers = match response.get("headers") {
        Some(Value::Map(headers)) => headers
            .borrow()
            .iter()
            .filter_map(|(name, value)| match value {
                Value::Str(value) => Some((name.clone(), value.to_string())),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    };
    let body = match response.get("body") {
        Some(Value::Str(body)) => body.to_string(),
        _ => String::new(),
    };
    Ok(FetchResponseParts {
        status,
        status_text,
        headers,
        body,
        route_result: Some("live".into()),
    })
}

fn default_response_parts(
    request: &FetchRequest,
    route_result: Option<&str>,
) -> FetchResponseParts {
    let (status, status_text, body) = response_parts(&request.url);
    FetchResponseParts {
        status,
        status_text: status_text.into(),
        headers: request.headers.clone(),
        body,
        route_result: route_result.map(str::to_string),
    }
}

fn route_action_for(
    request: &FetchRequest,
    route_handler: &SharedBrowserJsRouteHandler,
) -> Option<BrowserJsRouteAction> {
    let handler = route_handler.borrow().clone()?;
    let action = handler.borrow_mut()(BrowserJsRouteRequest {
        method: request.method.clone(),
        url: request.url.clone(),
        headers: request.headers.clone(),
        body: request.body.clone(),
    });
    Some(action)
}

fn headers_with_default(mut headers: Vec<(String, String)>) -> Vec<(String, String)> {
    if !headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("content-type"))
    {
        headers.push(("content-type".into(), "application/json".into()));
    }
    headers
}

fn status_text(status: u16) -> &'static str {
    match status {
        200..=299 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500..=599 => "Internal Server Error",
        _ => "",
    }
}

fn response_parts(url: &str) -> (u16, &'static str, String) {
    if url.starts_with("data:") {
        if let Some(comma_pos) = url.find(',') {
            let payload = &url[comma_pos + 1..];
            (200, "OK", payload.to_string())
        } else {
            (200, "OK", String::new())
        }
    } else if url.contains("404") || url.contains("not-found") {
        (404, "Not Found", "not found".to_string())
    } else if url.contains("500") {
        (500, "Internal Server Error", "server error".to_string())
    } else {
        (
            200,
            "OK",
            format!(
                "{{\"url\":\"{}\"}}",
                url.replace('\\', "\\\\").replace('"', "\\\"")
            ),
        )
    }
}

fn record_network_event(
    method: &str,
    url: &str,
    status: Option<u16>,
    route_result: Option<String>,
) {
    NETWORK_EVENTS.with(|events| {
        events.borrow_mut().push(BrowserJsNetworkEvent {
            method: method.to_ascii_uppercase(),
            url: url.into(),
            status,
            route_result,
        });
    });
}

fn install_xml_http_request(
    window: &mut HashMap<String, JsValue>,
    timers: Rc<RefCell<TimerQueue>>,
    route_handler: SharedBrowserJsRouteHandler,
    location: Rc<RefCell<HashMap<String, JsValue>>>,
) {
    window.insert(
        "XMLHttpRequest".into(),
        native("XMLHttpRequest", Some(0), move |_| {
            let xhr = Rc::new(RefCell::new(HashMap::new()));
            {
                let mut obj = xhr.borrow_mut();
                obj.insert("UNSENT".into(), JsValue::Number(0.0));
                obj.insert("OPENED".into(), JsValue::Number(1.0));
                obj.insert("HEADERS_RECEIVED".into(), JsValue::Number(2.0));
                obj.insert("LOADING".into(), JsValue::Number(3.0));
                obj.insert("DONE".into(), JsValue::Number(4.0));
                obj.insert("readyState".into(), JsValue::Number(0.0));
                obj.insert("status".into(), JsValue::Number(0.0));
                obj.insert("statusText".into(), JsValue::String(String::new()));
                obj.insert("response".into(), JsValue::String(String::new()));
                obj.insert("responseText".into(), JsValue::String(String::new()));
                obj.insert("responseType".into(), JsValue::String(String::new()));
                obj.insert("responseURL".into(), JsValue::String(String::new()));
                obj.insert("timeout".into(), JsValue::Number(0.0));
                obj.insert("withCredentials".into(), JsValue::Bool(false));
                obj.insert("onabort".into(), JsValue::Null);
                obj.insert("onreadystatechange".into(), JsValue::Null);
                obj.insert("onloadstart".into(), JsValue::Null);
                obj.insert("onload".into(), JsValue::Null);
                obj.insert("onloadend".into(), JsValue::Null);
                obj.insert("onerror".into(), JsValue::Null);
                obj.insert("ontimeout".into(), JsValue::Null);
                obj.insert("onprogress".into(), JsValue::Null);
            }

            let open_xhr = xhr.clone();
            let open_location = location.clone();
            xhr.borrow_mut().insert(
                "open".into(),
                native("XMLHttpRequest.open", None, move |args| {
                    let method = args
                        .first()
                        .map(JsValue::display)
                        .unwrap_or_else(|| "GET".into())
                        .to_ascii_uppercase();
                    let raw_url = args.get(1).map(JsValue::display).unwrap_or_default();
                    let url = resolve_url(&raw_url, Some(&location_href(&open_location)));
                    {
                        let mut xhr = open_xhr.borrow_mut();
                        xhr.insert("__method".into(), JsValue::String(method));
                        xhr.insert("__url".into(), JsValue::String(url));
                        xhr.insert("readyState".into(), JsValue::Number(1.0));
                    }
                    fire_xhr_event(&open_xhr, "readystatechange")?;
                    Ok(JsValue::Undefined)
                }),
            );

            let header_xhr = xhr.clone();
            xhr.borrow_mut().insert(
                "setRequestHeader".into(),
                native("XMLHttpRequest.setRequestHeader", Some(2), move |args| {
                    let name = args
                        .first()
                        .unwrap_or(&JsValue::Undefined)
                        .display()
                        .to_ascii_lowercase();
                    let value = args.get(1).unwrap_or(&JsValue::Undefined).display();
                    let mut xhr = header_xhr.borrow_mut();
                    let headers = xhr
                        .entry("__requestHeaders".into())
                        .or_insert_with(|| JsValue::Array(Rc::new(RefCell::new(Vec::new()))));
                    if let JsValue::Array(items) = headers {
                        items
                            .borrow_mut()
                            .push(JsValue::Array(Rc::new(RefCell::new(vec![
                                JsValue::String(name),
                                JsValue::String(value),
                            ]))));
                    }
                    Ok(JsValue::Undefined)
                }),
            );

            let send_xhr = xhr.clone();
            let send_timers = timers.clone();
            let send_routes = route_handler.clone();
            let send_location = location.clone();
            xhr.borrow_mut().insert(
                "send".into(),
                native("XMLHttpRequest.send", None, move |args| {
                    let body = args.first().map(JsValue::display);
                    let xhr_for_task = send_xhr.clone();
                    let routes_for_task = send_routes.clone();
                    let location_for_task = send_location.clone();
                    let callback = native("XMLHttpRequest.complete", Some(0), move |_| {
                        let request = xhr_request(
                            &xhr_for_task,
                            body.clone(),
                            &location_href(&location_for_task),
                        );
                        let parts = match fetch_response_parts(&request, &routes_for_task) {
                            Ok(parts) => parts,
                            Err(reason) => {
                                xhr_fail(&xhr_for_task, &request.url, &reason)?;
                                return Ok(JsValue::Undefined);
                            }
                        };
                        record_network_event(
                            &request.method,
                            &request.url,
                            Some(parts.status),
                            parts.route_result.clone(),
                        );
                        {
                            let mut xhr = xhr_for_task.borrow_mut();
                            let response = xhr_response_value(&xhr, &parts.body, &parts.headers);
                            xhr.insert("readyState".into(), JsValue::Number(4.0));
                            xhr.insert("status".into(), JsValue::Number(parts.status as f64));
                            xhr.insert("statusText".into(), JsValue::String(parts.status_text));
                            xhr.insert("response".into(), response);
                            xhr.insert("responseText".into(), JsValue::String(parts.body.clone()));
                            xhr.insert("responseURL".into(), JsValue::String(request.url));
                            xhr.insert("__responseHeaders".into(), headers_array(parts.headers));
                        }
                        fire_xhr_event(&xhr_for_task, "readystatechange")?;
                        fire_xhr_event(&xhr_for_task, "load")?;
                        fire_xhr_event(&xhr_for_task, "loadend")?;
                        Ok(JsValue::Undefined)
                    });
                    fire_xhr_event(&send_xhr, "loadstart")?;
                    send_timers
                        .borrow_mut()
                        .microtasks
                        .push_back(ScheduledCallback {
                            id: 0,
                            callback,
                            args: Vec::new(),
                            this_value: JsValue::Undefined,
                        });
                    Ok(JsValue::Undefined)
                }),
            );

            let response_header_xhr = xhr.clone();
            xhr.borrow_mut().insert(
                "getResponseHeader".into(),
                native("XMLHttpRequest.getResponseHeader", Some(1), move |args| {
                    let name = args
                        .first()
                        .unwrap_or(&JsValue::Undefined)
                        .display()
                        .to_ascii_lowercase();
                    Ok(xhr_response_header(&response_header_xhr, &name)
                        .map(JsValue::String)
                        .unwrap_or(JsValue::Null))
                }),
            );

            let all_headers_xhr = xhr.clone();
            xhr.borrow_mut().insert(
                "getAllResponseHeaders".into(),
                native("XMLHttpRequest.getAllResponseHeaders", Some(0), move |_| {
                    Ok(JsValue::String(xhr_all_response_headers(&all_headers_xhr)))
                }),
            );

            let abort_xhr = xhr.clone();
            xhr.borrow_mut().insert(
                "abort".into(),
                native("XMLHttpRequest.abort", Some(0), move |_| {
                    {
                        let mut xhr = abort_xhr.borrow_mut();
                        xhr.insert("readyState".into(), JsValue::Number(0.0));
                        xhr.insert("status".into(), JsValue::Number(0.0));
                        xhr.insert("statusText".into(), JsValue::String(String::new()));
                        xhr.insert("response".into(), JsValue::String(String::new()));
                        xhr.insert("responseText".into(), JsValue::String(String::new()));
                    }
                    fire_xhr_event(&abort_xhr, "readystatechange")?;
                    fire_xhr_event(&abort_xhr, "abort")?;
                    fire_xhr_event(&abort_xhr, "loadend")?;
                    Ok(JsValue::Undefined)
                }),
            );

            let listener_xhr = xhr.clone();
            xhr.borrow_mut().insert(
                "addEventListener".into(),
                native("XMLHttpRequest.addEventListener", Some(2), move |args| {
                    let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
                    let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
                    let key = format!("__listeners:{}", event_type);
                    let mut xhr = listener_xhr.borrow_mut();
                    let listeners = xhr
                        .entry(key)
                        .or_insert_with(|| JsValue::Array(Rc::new(RefCell::new(Vec::new()))));
                    if let JsValue::Array(items) = listeners {
                        items.borrow_mut().push(listener);
                    }
                    Ok(JsValue::Undefined)
                }),
            );

            Ok(JsValue::Object(xhr))
        }),
    );
}

fn xhr_request(
    xhr: &Rc<RefCell<HashMap<String, JsValue>>>,
    body: Option<String>,
    initiator_url: &str,
) -> FetchRequest {
    let xhr = xhr.borrow();
    let mut request = FetchRequest {
        url: xhr.get("__url").map(JsValue::display).unwrap_or_default(),
        method: xhr
            .get("__method")
            .map(JsValue::display)
            .unwrap_or_else(|| "GET".into()),
        headers: xhr
            .get("__requestHeaders")
            .map(headers_from_value)
            .unwrap_or_default(),
        body,
        aborted: false,
    };
    network_cookie_host::set_document_url(initiator_url);
    network_cookie_host::append_request_header(&mut request.headers, &request.url, initiator_url);
    request
}

fn xhr_status(xhr: &Rc<RefCell<HashMap<String, JsValue>>>) -> i64 {
    match xhr.borrow().get("status") {
        Some(JsValue::Number(status)) => *status as i64,
        _ => 0,
    }
}

fn xhr_fail(
    xhr: &Rc<RefCell<HashMap<String, JsValue>>>,
    url: &str,
    reason: &str,
) -> Result<(), String> {
    {
        let mut xhr = xhr.borrow_mut();
        xhr.insert("readyState".into(), JsValue::Number(4.0));
        xhr.insert("status".into(), JsValue::Number(0.0));
        xhr.insert("statusText".into(), JsValue::String(reason.into()));
        xhr.insert("response".into(), JsValue::String(String::new()));
        xhr.insert("responseText".into(), JsValue::String(String::new()));
        xhr.insert("responseURL".into(), JsValue::String(url.into()));
    }
    fire_xhr_event(xhr, "readystatechange")?;
    fire_xhr_event(xhr, "error")?;
    fire_xhr_event(xhr, "loadend")
}

fn xhr_response_value(
    xhr: &HashMap<String, JsValue>,
    body: &str,
    headers: &[(String, String)],
) -> JsValue {
    match xhr
        .get("responseType")
        .map(JsValue::display)
        .unwrap_or_default()
        .as_str()
    {
        "arraybuffer" => byte_array_from_text(body),
        "blob" => blob_from_text_body(body, body_mime_type(headers)),
        "json" => parse_simple_json(body),
        _ => JsValue::String(body.into()),
    }
}

fn headers_array(headers: Vec<(String, String)>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        headers_with_default(headers)
            .into_iter()
            .map(|(name, value)| {
                JsValue::Array(Rc::new(RefCell::new(vec![
                    JsValue::String(name.to_ascii_lowercase()),
                    JsValue::String(value),
                ])))
            })
            .collect(),
    )))
}

fn xhr_response_headers(xhr: &Rc<RefCell<HashMap<String, JsValue>>>) -> Vec<(String, String)> {
    if xhr_status(xhr) == 0 {
        return Vec::new();
    }
    xhr.borrow()
        .get("__responseHeaders")
        .map(headers_from_value)
        .unwrap_or_else(|| headers_with_default(Vec::new()))
}

fn xhr_response_header(xhr: &Rc<RefCell<HashMap<String, JsValue>>>, name: &str) -> Option<String> {
    xhr_response_headers(xhr)
        .into_iter()
        .find(|(candidate, _)| candidate == name)
        .map(|(_, value)| value)
}

fn xhr_all_response_headers(xhr: &Rc<RefCell<HashMap<String, JsValue>>>) -> String {
    xhr_response_headers(xhr)
        .into_iter()
        .map(|(name, value)| format!("{}: {}\r\n", name, value))
        .collect()
}

fn fire_xhr_event(
    xhr: &Rc<RefCell<HashMap<String, JsValue>>>,
    event_type: &str,
) -> Result<(), String> {
    let this_value = JsValue::Object(xhr.clone());
    let event = normalize_event(
        JsValue::String(event_type.into()),
        event_type,
        this_value.clone(),
        this_value.clone(),
    );
    let handler = xhr.borrow().get(&format!("on{}", event_type)).cloned();
    if let Some(handler) = handler {
        if !matches!(handler, JsValue::Null | JsValue::Undefined) {
            call_dom_listener(handler, this_value.clone(), event.clone())?;
        }
    }
    let listeners = xhr
        .borrow()
        .get(&format!("__listeners:{}", event_type))
        .and_then(|value| match value {
            JsValue::Array(items) => Some(items.borrow().clone()),
            _ => None,
        })
        .unwrap_or_default();
    for listener in listeners {
        call_dom_listener(listener, this_value.clone(), event.clone())?;
    }
    Ok(())
}

fn install_realtime_bindings(
    window: &mut HashMap<String, JsValue>,
    timers: Rc<RefCell<TimerQueue>>,
    realtime: Rc<RefCell<RealtimeHost>>,
) {
    let ws_host = realtime.clone();
    let ws_timers = timers.clone();
    window.insert(
        "WebSocket".into(),
        native("WebSocket", None, move |args| {
            let url = args.first().unwrap_or(&JsValue::Undefined).display();
            let object = realtime_object_base(&url, 3);
            let id = register_realtime(&ws_host, BrowserJsRealtimeKind::WebSocket, &url, &object);
            install_websocket_object(&object, ws_host.clone(), id, &url);
            queue_realtime_open(
                &ws_timers,
                ws_host.clone(),
                object.clone(),
                BrowserJsRealtimeKind::WebSocket,
                id,
                url.clone(),
            );
            Ok(JsValue::Object(object))
        }),
    );

    let es_host = realtime;
    let es_timers = timers;
    window.insert(
        "EventSource".into(),
        native("EventSource", None, move |args| {
            let url = args.first().unwrap_or(&JsValue::Undefined).display();
            let object = realtime_object_base(&url, 2);
            let id = register_realtime(&es_host, BrowserJsRealtimeKind::EventSource, &url, &object);
            install_event_source_object(&object, es_host.clone(), id, &url);
            queue_realtime_open(
                &es_timers,
                es_host.clone(),
                object.clone(),
                BrowserJsRealtimeKind::EventSource,
                id,
                url.clone(),
            );
            Ok(JsValue::Object(object))
        }),
    );
}

fn realtime_object_base(url: &str, closed: i64) -> Rc<RefCell<HashMap<String, JsValue>>> {
    Rc::new(RefCell::new(HashMap::from([
        ("url".into(), JsValue::String(url.into())),
        ("readyState".into(), JsValue::Number(0.0)),
        ("CONNECTING".into(), JsValue::Number(0.0)),
        ("OPEN".into(), JsValue::Number(1.0)),
        ("CLOSED".into(), JsValue::Number(closed as f64)),
        ("onopen".into(), JsValue::Null),
        ("onmessage".into(), JsValue::Null),
        ("onclose".into(), JsValue::Null),
        ("onerror".into(), JsValue::Null),
    ])))
}

fn register_realtime(
    host: &Rc<RefCell<RealtimeHost>>,
    kind: BrowserJsRealtimeKind,
    url: &str,
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
) -> u64 {
    let mut state = host.borrow_mut();
    let id = state.next_id;
    state.next_id += 1;
    state.connections.push(RealtimeConnectionHandle {
        id,
        kind,
        url: url.into(),
        object: object.clone(),
    });
    drop(state);
    push_realtime_event(
        host,
        id,
        kind,
        url,
        realtime_model::BrowserJsRealtimeEventKind::Connect,
        0,
        None,
        None,
        None,
        None,
    );
    id
}

fn install_websocket_object(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    host: Rc<RefCell<RealtimeHost>>,
    id: u64,
    url: &str,
) {
    let mut obj = object.borrow_mut();
    obj.insert("CLOSING".into(), JsValue::Number(2.0));
    install_realtime_listeners(&mut obj, object.clone());
    let send_object = object.clone();
    let send_url = url.to_string();
    let send_host = host.clone();
    obj.insert(
        "send".into(),
        native("WebSocket.send", Some(1), move |args| {
            if realtime_ready_state(&send_object) != 1 {
                return Err("WebSocket.send: socket is not open".into());
            }
            let data = args.first().unwrap_or(&JsValue::Undefined).display();
            send_host
                .borrow_mut()
                .outbound
                .push(BrowserJsRealtimeOutbound {
                    connection_id: id,
                    url: send_url.clone(),
                    data: data.clone(),
                });
            push_realtime_event(
                &send_host,
                id,
                BrowserJsRealtimeKind::WebSocket,
                &send_url,
                realtime_model::BrowserJsRealtimeEventKind::Send,
                1,
                Some(data),
                None,
                None,
                None,
            );
            Ok(JsValue::Undefined)
        }),
    );
    let close_object = object.clone();
    let close_host = host;
    let close_url = url.to_string();
    obj.insert(
        "close".into(),
        native("WebSocket.close", None, move |args| {
            let code = args.first().and_then(js_u16);
            let reason = args.get(1).map(JsValue::display);
            if close_realtime(&close_object, 3)? {
                push_realtime_event(
                    &close_host,
                    id,
                    BrowserJsRealtimeKind::WebSocket,
                    &close_url,
                    realtime_model::BrowserJsRealtimeEventKind::Close,
                    3,
                    None,
                    code,
                    reason,
                    None,
                );
            }
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_event_source_object(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    host: Rc<RefCell<RealtimeHost>>,
    id: u64,
    url: &str,
) {
    let mut obj = object.borrow_mut();
    obj.insert("__eventSourceId".into(), JsValue::Number(id as f64));
    obj.insert("__retryMs".into(), JsValue::Number(3000.0));
    install_realtime_listeners(&mut obj, object.clone());
    let close_object = object.clone();
    let close_url = url.to_string();
    obj.insert(
        "close".into(),
        native("EventSource.close", Some(0), move |_| {
            if close_realtime(&close_object, 2)? {
                push_realtime_event(
                    &host,
                    id,
                    BrowserJsRealtimeKind::EventSource,
                    &close_url,
                    realtime_model::BrowserJsRealtimeEventKind::Close,
                    2,
                    None,
                    None,
                    None,
                    Some(realtime_retry_ms(&close_object)),
                );
            }
            Ok(JsValue::Undefined)
        }),
    );
}

fn install_realtime_listeners(
    obj: &mut HashMap<String, JsValue>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
) {
    obj.insert(
        "addEventListener".into(),
        native("Realtime.addEventListener", Some(2), move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let mut obj = object.borrow_mut();
            let entry = obj
                .entry(format!("__listeners:{}", event_type))
                .or_insert_with(|| JsValue::Array(Rc::new(RefCell::new(Vec::new()))));
            if let JsValue::Array(items) = entry {
                items.borrow_mut().push(listener);
            }
            Ok(JsValue::Undefined)
        }),
    );
}

fn queue_realtime_open(
    timers: &Rc<RefCell<TimerQueue>>,
    host: Rc<RefCell<RealtimeHost>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    kind: BrowserJsRealtimeKind,
    id: u64,
    url: String,
) {
    timers.borrow_mut().microtasks.push_back(ScheduledCallback {
        id: 0,
        callback: native("Realtime.open", Some(0), move |_| {
            if realtime_ready_state(&object) == 0 {
                set_realtime_ready_state(&object, 1);
                push_realtime_event(
                    &host,
                    id,
                    kind,
                    &url,
                    realtime_model::BrowserJsRealtimeEventKind::Open,
                    1,
                    None,
                    None,
                    None,
                    event_source_retry(kind, &object),
                );
                fire_realtime_event(&object, "open", None)?;
            }
            Ok(JsValue::Undefined)
        }),
        args: Vec::new(),
        this_value: JsValue::Undefined,
    });
}

fn inject_realtime_message(
    host: &Rc<RefCell<RealtimeHost>>,
    kind: BrowserJsRealtimeKind,
    connection_id: u64,
    data: &str,
) -> Result<(), String> {
    let (url, object) = realtime_object(host, kind, connection_id)
        .ok_or_else(|| format!("realtime connection {} was not found", connection_id))?;
    if realtime_ready_state(&object) != 1 {
        return Err(format!("realtime connection {} is not open", connection_id));
    }
    fire_realtime_event(&object, "message", Some(data))?;
    push_realtime_event(
        host,
        connection_id,
        kind,
        &url,
        realtime_model::BrowserJsRealtimeEventKind::Receive,
        1,
        Some(data.into()),
        None,
        None,
        event_source_retry(kind, &object),
    );
    Ok(())
}

fn realtime_connections(host: &Rc<RefCell<RealtimeHost>>) -> Vec<BrowserJsRealtimeConnection> {
    host.borrow()
        .connections
        .iter()
        .map(|connection| BrowserJsRealtimeConnection {
            id: connection.id,
            kind: connection.kind,
            url: connection.url.clone(),
            ready_state: realtime_ready_state(&connection.object),
            retry_ms: event_source_retry(connection.kind, &connection.object),
        })
        .collect()
}

fn realtime_object(
    host: &Rc<RefCell<RealtimeHost>>,
    kind: BrowserJsRealtimeKind,
    connection_id: u64,
) -> RealtimeObjectLookup {
    host.borrow()
        .connections
        .iter()
        .find(|connection| connection.id == connection_id && connection.kind == kind)
        .map(|connection| (connection.url.clone(), connection.object.clone()))
}

fn fail_realtime_connection(
    host: &Rc<RefCell<RealtimeHost>>,
    kind: BrowserJsRealtimeKind,
    connection_id: u64,
    reason: &str,
    retry_ms: Option<u64>,
) -> Result<(), String> {
    let (url, object) = realtime_object(host, kind, connection_id)
        .ok_or_else(|| format!("realtime connection {} was not found", connection_id))?;
    let retry = retry_ms.or_else(|| event_source_retry(kind, &object));
    if kind == BrowserJsRealtimeKind::EventSource {
        set_event_source_retry(&object, retry.unwrap_or(3000));
        set_realtime_ready_state(&object, 0);
    } else {
        set_realtime_ready_state(&object, 3);
    }
    fire_realtime_error(&object, reason, retry)?;
    push_realtime_event(
        host,
        connection_id,
        kind,
        &url,
        realtime_model::BrowserJsRealtimeEventKind::Error,
        realtime_ready_state(&object),
        None,
        None,
        Some(reason.into()),
        retry,
    );
    if kind == BrowserJsRealtimeKind::WebSocket {
        fire_realtime_event(&object, "close", None)?;
        push_realtime_event(
            host,
            connection_id,
            kind,
            &url,
            realtime_model::BrowserJsRealtimeEventKind::Close,
            3,
            None,
            None,
            Some(reason.into()),
            None,
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn push_realtime_event(
    host: &Rc<RefCell<RealtimeHost>>,
    connection_id: u64,
    kind: BrowserJsRealtimeKind,
    url: &str,
    event: realtime_model::BrowserJsRealtimeEventKind,
    ready_state: i64,
    data: Option<String>,
    code: Option<u16>,
    reason: Option<String>,
    retry_ms: Option<u64>,
) {
    let mut host = host.borrow_mut();
    let sequence = host.next_sequence;
    host.next_sequence += 1;
    host.events.push(realtime_model::BrowserJsRealtimeEvent {
        sequence,
        connection_id,
        event,
        connection_kind: kind,
        url: url.into(),
        ready_state,
        data,
        code,
        reason,
        retry_ms,
    });
    drop(host);
    record_network_event(
        realtime_method(kind),
        url,
        realtime_status(kind, event),
        Some(format!("realtime:{}", realtime_label(event))),
    );
}

fn realtime_method(kind: BrowserJsRealtimeKind) -> &'static str {
    match kind {
        BrowserJsRealtimeKind::WebSocket => "WEBSOCKET",
        BrowserJsRealtimeKind::EventSource => "EVENTSOURCE",
    }
}

fn realtime_status(
    kind: BrowserJsRealtimeKind,
    event: realtime_model::BrowserJsRealtimeEventKind,
) -> Option<u16> {
    match (kind, event) {
        (BrowserJsRealtimeKind::WebSocket, realtime_model::BrowserJsRealtimeEventKind::Open) => {
            Some(101)
        }
        (BrowserJsRealtimeKind::EventSource, realtime_model::BrowserJsRealtimeEventKind::Open) => {
            Some(200)
        }
        _ => None,
    }
}

fn realtime_label(event: realtime_model::BrowserJsRealtimeEventKind) -> &'static str {
    match event {
        realtime_model::BrowserJsRealtimeEventKind::Connect => "connect",
        realtime_model::BrowserJsRealtimeEventKind::Open => "open",
        realtime_model::BrowserJsRealtimeEventKind::Send => "send",
        realtime_model::BrowserJsRealtimeEventKind::Receive => "receive",
        realtime_model::BrowserJsRealtimeEventKind::Close => "close",
        realtime_model::BrowserJsRealtimeEventKind::Error => "error",
    }
}

fn close_realtime(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    closed_state: i64,
) -> Result<bool, String> {
    if realtime_ready_state(object) == closed_state {
        return Ok(false);
    }
    set_realtime_ready_state(object, closed_state);
    fire_realtime_event(object, "close", None)?;
    Ok(true)
}

fn fire_realtime_event(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    event_type: &str,
    data: Option<&str>,
) -> Result<(), String> {
    let this_value = JsValue::Object(object.clone());
    let event = normalize_event(
        JsValue::String(event_type.into()),
        event_type,
        this_value.clone(),
        this_value.clone(),
    );
    if let (Some(data), JsValue::Object(event_obj)) = (data, &event) {
        event_obj
            .borrow_mut()
            .insert("data".into(), JsValue::String(data.into()));
    }
    fire_realtime_handler(object, event_type, this_value, event)
}

fn fire_realtime_error(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    reason: &str,
    retry_ms: Option<u64>,
) -> Result<(), String> {
    let this_value = JsValue::Object(object.clone());
    let event = normalize_event(
        JsValue::String("error".into()),
        "error",
        this_value.clone(),
        this_value.clone(),
    );
    if let JsValue::Object(event_obj) = &event {
        let mut event_obj = event_obj.borrow_mut();
        event_obj.insert("message".into(), JsValue::String(reason.into()));
        event_obj.insert("reason".into(), JsValue::String(reason.into()));
        if let Some(retry) = retry_ms {
            event_obj.insert("retry".into(), JsValue::Number(retry as f64));
        }
    }
    fire_realtime_handler(object, "error", this_value, event)
}

fn fire_realtime_handler(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    event_type: &str,
    this_value: JsValue,
    event: JsValue,
) -> Result<(), String> {
    let handler = object.borrow().get(&format!("on{}", event_type)).cloned();
    if let Some(handler) = handler {
        if !matches!(handler, JsValue::Null | JsValue::Undefined) {
            call_dom_listener(handler, this_value.clone(), event.clone())?;
        }
    }
    for listener in realtime_listeners(object, event_type) {
        call_dom_listener(listener, this_value.clone(), event.clone())?;
    }
    Ok(())
}

fn realtime_listeners(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    event_type: &str,
) -> Vec<JsValue> {
    object
        .borrow()
        .get(&format!("__listeners:{}", event_type))
        .and_then(|value| match value {
            JsValue::Array(items) => Some(items.borrow().clone()),
            _ => None,
        })
        .unwrap_or_default()
}

fn realtime_ready_state(object: &Rc<RefCell<HashMap<String, JsValue>>>) -> i64 {
    match object.borrow().get("readyState") {
        Some(JsValue::Number(state)) => *state as i64,
        _ => 0,
    }
}

fn set_realtime_ready_state(object: &Rc<RefCell<HashMap<String, JsValue>>>, state: i64) {
    object
        .borrow_mut()
        .insert("readyState".into(), JsValue::Number(state as f64));
}

fn event_source_retry(
    kind: BrowserJsRealtimeKind,
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
) -> Option<u64> {
    (kind == BrowserJsRealtimeKind::EventSource).then(|| realtime_retry_ms(object))
}

fn realtime_retry_ms(object: &Rc<RefCell<HashMap<String, JsValue>>>) -> u64 {
    match object.borrow().get("__retryMs") {
        Some(JsValue::Number(retry)) => *retry as u64,
        _ => 3000,
    }
}

fn set_event_source_retry(object: &Rc<RefCell<HashMap<String, JsValue>>>, retry_ms: u64) {
    object
        .borrow_mut()
        .insert("__retryMs".into(), JsValue::Number(retry_ms as f64));
}

fn js_u16(value: &JsValue) -> Option<u16> {
    match value {
        JsValue::Number(number) if *number >= 0.0 && *number <= u16::MAX as f64 => {
            Some(*number as u16)
        }
        _ => None,
    }
}

fn json_body_from_value(value: &JsValue) -> String {
    match value {
        JsValue::Undefined | JsValue::Null => "null".into(),
        JsValue::Bool(value) => value.to_string(),
        JsValue::Number(value) if value.is_finite() => JsValue::Number(*value).display(),
        JsValue::Number(_) => "null".into(),
        JsValue::String(value) => json_string(value),
        JsValue::Symbol(_) => "null".into(),
        JsValue::Array(items) => json_array_body(&items.borrow()),
        JsValue::Object(obj) => json_object_body(&obj.borrow()),
        JsValue::Function(_)
        | JsValue::BoundFunction(_)
        | JsValue::Class(_)
        | JsValue::Native(_) => "null".into(),
    }
}

fn json_array_body(items: &[JsValue]) -> String {
    format!(
        "[{}]",
        items
            .iter()
            .map(json_body_from_value)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn json_object_body(obj: &HashMap<String, JsValue>) -> String {
    let mut rows = obj
        .iter()
        .filter(|(key, value)| {
            !key.starts_with("__")
                && !matches!(
                    value,
                    JsValue::Function(_) | JsValue::BoundFunction(_) | JsValue::Native(_)
                )
        })
        .collect::<Vec<_>>();
    rows.sort_by(|(left, _), (right, _)| left.cmp(right));
    format!(
        "{{{}}}",
        rows.into_iter()
            .map(|(key, value)| format!("{}:{}", json_string(key), json_body_from_value(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn parse_simple_json(input: &str) -> JsValue {
    let s = input.trim();
    if s.starts_with('{') && s.ends_with('}') {
        let inner = &s[1..s.len() - 1];
        let mut map = HashMap::new();
        let mut depth = 0usize;
        let mut start = 0usize;
        for (i, ch) in inner.char_indices() {
            match ch {
                '{' | '[' => depth += 1,
                '}' | ']' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => {
                    if let Some((k, v)) = parse_kv_pair(&inner[start..i]) {
                        map.insert(k, v);
                    }
                    start = i + 1;
                }
                _ => {}
            }
        }
        if start < inner.len() {
            if let Some((k, v)) = parse_kv_pair(&inner[start..]) {
                map.insert(k, v);
            }
        }
        return JsValue::Object(Rc::new(RefCell::new(map)));
    }
    JsValue::String(input.into())
}

fn parse_kv_pair(s: &str) -> Option<(String, JsValue)> {
    let colon = s.find(':')?;
    let key = s[..colon].trim().trim_matches('"').to_string();
    let val = s[colon + 1..].trim();
    let js_val = if val.starts_with('"') && val.ends_with('"') {
        JsValue::String(val[1..val.len() - 1].to_string())
    } else if val == "true" {
        JsValue::Bool(true)
    } else if val == "false" {
        JsValue::Bool(false)
    } else if val == "null" {
        JsValue::Null
    } else if let Ok(n) = val.parse::<f64>() {
        JsValue::Number(n)
    } else {
        JsValue::String(val.to_string())
    };
    Some((key, js_val))
}

// ── MutationObserver ────────────────────────────────────────────────────

fn install_mutation_observer(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "MutationObserver".into(),
        native("MutationObserver", Some(1), |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            let observer_id = NEXT_MUTATION_OBSERVER_ID.with(|next| {
                let mut next = next.borrow_mut();
                let id = *next;
                *next = next.saturating_add(1);
                id
            });
            MUTATION_OBSERVERS.with(|observers| {
                observers.borrow_mut().insert(
                    observer_id,
                    MutationObserverState {
                        callback: callback.clone(),
                        targets: Vec::new(),
                        records: Vec::new(),
                        connected: true,
                    },
                );
            });
            let mut obj = HashMap::new();
            obj.insert(
                "observe".into(),
                native("MutationObserver.observe", Some(2), move |args| {
                    let target = args.first().and_then(dom_handle_from_value);
                    let options = mutation_observer_options(args.get(1));
                    if let Some(target) = target {
                        MUTATION_OBSERVERS.with(|observers| {
                            if let Some(observer) = observers.borrow_mut().get_mut(&observer_id) {
                                observer
                                    .targets
                                    .retain(|observed| !same_dom_handle(&observed.handle, &target));
                                observer.targets.push(MutationObserverTarget {
                                    handle: target,
                                    options,
                                });
                                observer.connected = true;
                            }
                        });
                    }
                    Ok(JsValue::Undefined)
                }),
            );
            obj.insert(
                "disconnect".into(),
                native("MutationObserver.disconnect", Some(0), move |_| {
                    MUTATION_OBSERVERS.with(|observers| {
                        if let Some(observer) = observers.borrow_mut().get_mut(&observer_id) {
                            observer.targets.clear();
                            observer.records.clear();
                            observer.connected = false;
                        }
                    });
                    Ok(JsValue::Undefined)
                }),
            );
            obj.insert(
                "takeRecords".into(),
                native("MutationObserver.takeRecords", Some(0), move |_| {
                    let recs = MUTATION_OBSERVERS.with(|observers| {
                        observers
                            .borrow_mut()
                            .get_mut(&observer_id)
                            .map(|observer| std::mem::take(&mut observer.records))
                            .unwrap_or_default()
                    });
                    Ok(JsValue::Array(Rc::new(RefCell::new(recs))))
                }),
            );
            obj.insert("__observerId".into(), JsValue::Number(observer_id as f64));
            obj.insert("__callback".into(), callback);
            Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
        }),
    );
}

impl MutationObserverTarget {
    fn options_for(
        &self,
        target: &DomHandle,
        mutation_type: &str,
    ) -> Option<MutationObserverOptions> {
        if !Rc::ptr_eq(&self.handle.root, &target.root) {
            return None;
        }
        let exact = self.handle.path == target.path;
        let nested = self.options.subtree && target.path.starts_with(&self.handle.path);
        (exact || nested)
            .then_some(self.options)
            .filter(|options| options.accepts(mutation_type))
    }
}

impl MutationObserverOptions {
    fn accepts(self, mutation_type: &str) -> bool {
        match mutation_type {
            "attributes" => self.attributes,
            "characterData" => self.character_data,
            "childList" => self.child_list,
            _ => false,
        }
    }
}

fn same_dom_handle(a: &DomHandle, b: &DomHandle) -> bool {
    Rc::ptr_eq(&a.root, &b.root) && a.path == b.path
}

fn mutation_observer_options(value: Option<&JsValue>) -> MutationObserverOptions {
    let Some(JsValue::Object(obj)) = value else {
        return default_mutation_observer_options();
    };
    let obj = obj.borrow();
    let attribute_old_value = option_bool(&obj, "attributeOldValue");
    let character_data_old_value = option_bool(&obj, "characterDataOldValue");
    let mut options = MutationObserverOptions {
        child_list: option_bool(&obj, "childList"),
        attributes: option_bool(&obj, "attributes") || attribute_old_value,
        character_data: option_bool(&obj, "characterData") || character_data_old_value,
        subtree: option_bool(&obj, "subtree"),
        attribute_old_value,
        character_data_old_value,
    };
    if !options.child_list && !options.attributes && !options.character_data {
        options.child_list = true;
        options.attributes = true;
        options.character_data = true;
    }
    options
}

fn default_mutation_observer_options() -> MutationObserverOptions {
    MutationObserverOptions {
        child_list: true,
        attributes: true,
        character_data: true,
        subtree: false,
        attribute_old_value: false,
        character_data_old_value: false,
    }
}

fn option_bool(obj: &HashMap<String, JsValue>, name: &str) -> bool {
    obj.get(name).is_some_and(JsValue::truthy)
}

fn observer_old_value(
    mutation_type: &str,
    old_value: &Option<String>,
    options: MutationObserverOptions,
) -> JsValue {
    let enabled = match mutation_type {
        "attributes" => options.attribute_old_value,
        "characterData" => options.character_data_old_value,
        _ => false,
    };
    if enabled {
        old_value
            .clone()
            .map(JsValue::String)
            .unwrap_or(JsValue::Null)
    } else {
        JsValue::Null
    }
}

fn queue_mutation_record(
    target: &DomHandle,
    mutation_type: &str,
    attribute_name: Option<String>,
    old_value: Option<String>,
    added_nodes: Vec<Node>,
    removed_nodes: Vec<Node>,
) {
    MUTATION_OBSERVERS.with(|observers| {
        for observer in observers.borrow_mut().values_mut() {
            if !observer.connected {
                continue;
            }
            let options = observer
                .targets
                .iter()
                .find_map(|observed| observed.options_for(target, mutation_type));
            if let Some(options) = options {
                observer.records.push(mutation_record(
                    target,
                    mutation_type,
                    &attribute_name,
                    &old_value,
                    options,
                    &added_nodes,
                    &removed_nodes,
                ));
            }
        }
    });
}

fn mutation_record(
    target: &DomHandle,
    mutation_type: &str,
    attribute_name: &Option<String>,
    old_value: &Option<String>,
    options: MutationObserverOptions,
    added_nodes: &[Node],
    removed_nodes: &[Node],
) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("type".into(), JsValue::String(mutation_type.into()));
    obj.insert("target".into(), node_object(target.clone()));
    obj.insert("targetKey".into(), JsValue::String(target.event_key()));
    obj.insert(
        "attributeName".into(),
        attribute_name
            .clone()
            .map(JsValue::String)
            .unwrap_or(JsValue::Null),
    );
    obj.insert(
        "oldValue".into(),
        observer_old_value(mutation_type, old_value, options),
    );
    obj.insert(
        "addedNodes".into(),
        JsValue::Array(Rc::new(RefCell::new(
            added_nodes
                .iter()
                .cloned()
                .map(detached_node_object)
                .collect(),
        ))),
    );
    obj.insert(
        "removedNodes".into(),
        JsValue::Array(Rc::new(RefCell::new(
            removed_nodes
                .iter()
                .cloned()
                .map(detached_node_object)
                .collect(),
        ))),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn deliver_mutation_observers() -> Result<usize, String> {
    let deliveries = MUTATION_OBSERVERS.with(|observers| {
        observers
            .borrow_mut()
            .values_mut()
            .filter_map(|observer| {
                if observer.connected && !observer.records.is_empty() {
                    Some((
                        observer.callback.clone(),
                        std::mem::take(&mut observer.records),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });
    let delivered = deliveries.len();
    for (callback, records) in deliveries {
        js::call_function_with_this(
            callback,
            JsValue::Undefined,
            &[JsValue::Array(Rc::new(RefCell::new(records)))],
        )?;
    }
    Ok(delivered)
}

// ── IntersectionObserver ────────────────────────────────────────────────

fn install_intersection_observer(window: &mut HashMap<String, JsValue>, root: Rc<RefCell<Node>>) {
    let obs_root = root.clone();
    window.insert(
        "IntersectionObserver".into(),
        native("IntersectionObserver", Some(1), move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            let options = args.get(1).cloned().unwrap_or(JsValue::Undefined);

            let threshold = if let JsValue::Object(opts) = &options {
                opts.borrow()
                    .get("threshold")
                    .and_then(|v| match v {
                        JsValue::Number(n) => Some(*n),
                        JsValue::Array(arr) => Some(
                            arr.borrow()
                                .first()
                                .and_then(|v| match v {
                                    JsValue::Number(n) => Some(*n),
                                    _ => None,
                                })
                                .unwrap_or(0.0),
                        ),
                        _ => None,
                    })
                    .unwrap_or(0.0)
            } else {
                0.0
            };

            let mut obj = HashMap::new();
            obj.insert("threshold".into(), JsValue::Number(threshold));
            obj.insert("rootMargin".into(), JsValue::String("0px".into()));

            let cb = callback.clone();
            let observe_root = obs_root.clone();
            obj.insert(
                "observe".into(),
                native("IntersectionObserver.observe", Some(1), move |args| {
                    let target = args.first().cloned().unwrap_or(JsValue::Undefined);
                    let doc = root_to_document(&observe_root);
                    let rect = dom_handle_from_value(&target)
                        .map(|handle| element_rect(&handle))
                        .unwrap_or((0, 0, 0, 0));
                    let ratio = if doc.children.is_empty() || rect.2 <= 0 || rect.3 <= 0 {
                        0.0
                    } else {
                        1.0
                    };
                    let entry = make_intersection_entry(target.clone(), ratio);
                    let entries = vec![entry];
                    let _ = js::call_function_with_this(
                        cb.clone(),
                        JsValue::Undefined,
                        &[JsValue::Array(Rc::new(RefCell::new(entries)))],
                    );
                    Ok(JsValue::Undefined)
                }),
            );

            let cb_unobserve = callback.clone();
            obj.insert(
                "unobserve".into(),
                native("IntersectionObserver.unobserve", Some(1), move |_args| {
                    let _ = cb_unobserve;
                    Ok(JsValue::Undefined)
                }),
            );

            let cb_disconnect = callback;
            obj.insert(
                "disconnect".into(),
                native("IntersectionObserver.disconnect", Some(0), move |_| {
                    let _ = cb_disconnect;
                    Ok(JsValue::Undefined)
                }),
            );

            obj.insert(
                "takeRecords".into(),
                native("IntersectionObserver.takeRecords", Some(0), move |_| {
                    Ok(JsValue::Array(Rc::new(RefCell::new(Vec::new()))))
                }),
            );

            Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
        }),
    );
}

fn make_intersection_entry(target: JsValue, ratio: f64) -> JsValue {
    let rect = dom_handle_from_value(&target)
        .map(|handle| element_rect(&handle))
        .unwrap_or((0, 0, 0, 0));
    let intersection = if ratio > 0.0 { rect } else { (0, 0, 0, 0) };
    let mut obj = HashMap::new();
    obj.insert("target".into(), target);
    obj.insert("isIntersecting".into(), JsValue::Bool(ratio > 0.0));
    obj.insert("intersectionRatio".into(), JsValue::Number(ratio));
    obj.insert("time".into(), JsValue::Number(0.0));
    obj.insert("boundingClientRect".into(), rect_object(&rect));
    obj.insert("intersectionRect".into(), rect_object(&intersection));
    obj.insert("rootBounds".into(), rect_object(&(0, 0, 80, 1000)));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn install_resize_observer(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "ResizeObserver".into(),
        native("ResizeObserver", Some(1), move |args| {
            let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
            let records: Rc<RefCell<Vec<JsValue>>> = Rc::new(RefCell::new(Vec::new()));
            let connected = Rc::new(RefCell::new(true));
            let mut obj = HashMap::new();

            let observe_callback = callback.clone();
            let observe_records = records.clone();
            let observe_connected = connected.clone();
            obj.insert(
                "observe".into(),
                native("ResizeObserver.observe", Some(1), move |args| {
                    if !*observe_connected.borrow() {
                        return Ok(JsValue::Undefined);
                    }
                    let target = args.first().cloned().unwrap_or(JsValue::Undefined);
                    let entry = resize_observer_entry(target);
                    observe_records.borrow_mut().push(entry.clone());
                    js::call_function_with_this(
                        observe_callback.clone(),
                        JsValue::Undefined,
                        &[JsValue::Array(Rc::new(RefCell::new(vec![entry])))],
                    )?;
                    Ok(JsValue::Undefined)
                }),
            );

            let unobserve_records = records.clone();
            obj.insert(
                "unobserve".into(),
                native("ResizeObserver.unobserve", Some(1), move |_args| {
                    unobserve_records.borrow_mut().clear();
                    Ok(JsValue::Undefined)
                }),
            );

            let disconnect_records = records.clone();
            let disconnect_connected = connected;
            obj.insert(
                "disconnect".into(),
                native("ResizeObserver.disconnect", Some(0), move |_| {
                    *disconnect_connected.borrow_mut() = false;
                    disconnect_records.borrow_mut().clear();
                    Ok(JsValue::Undefined)
                }),
            );

            let take_records = records;
            obj.insert(
                "takeRecords".into(),
                native("ResizeObserver.takeRecords", Some(0), move |_| {
                    let records = std::mem::take(&mut *take_records.borrow_mut());
                    Ok(JsValue::Array(Rc::new(RefCell::new(records))))
                }),
            );

            Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
        }),
    );
}

fn resize_observer_entry(target: JsValue) -> JsValue {
    let rect = dom_handle_from_value(&target)
        .map(|handle| element_rect(&handle))
        .unwrap_or((0, 0, 0, 0));
    let mut obj = HashMap::new();
    obj.insert("target".into(), target);
    obj.insert("contentRect".into(), rect_object(&rect));
    obj.insert(
        "contentBoxSize".into(),
        JsValue::Array(Rc::new(RefCell::new(vec![resize_size_object(rect)]))),
    );
    obj.insert(
        "borderBoxSize".into(),
        JsValue::Array(Rc::new(RefCell::new(vec![resize_size_object(rect)]))),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn resize_size_object(rect: (i64, i64, i64, i64)) -> JsValue {
    let (_, _, width, height) = rect;
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        ("inlineSize".into(), JsValue::Number(width as f64)),
        ("blockSize".into(), JsValue::Number(height as f64)),
    ]))))
}

fn install_web_api_bindings(window: &mut HashMap<String, JsValue>) {
    window.insert("URL".into(), url_constructor());
    window.insert(
        "URLSearchParams".into(),
        native("URLSearchParams", None, move |args| {
            Ok(url_search_params_object(url_search_params_entries(
                args.first().unwrap_or(&JsValue::Undefined),
            )))
        }),
    );
    window.insert(
        "Headers".into(),
        native("Headers", None, move |args| {
            Ok(headers_object(
                args.first().map(headers_from_value).unwrap_or_default(),
            ))
        }),
    );
    window.insert(
        "Request".into(),
        native("Request", None, move |args| {
            Ok(request_object(&request_from_fetch_args(args)))
        }),
    );
    window.insert("Response".into(), response_constructor());
    window.insert(
        "AbortController".into(),
        native("AbortController", Some(0), move |_| {
            Ok(abort_controller_object())
        }),
    );
    window.insert(
        "atob".into(),
        native("atob", Some(1), move |args| {
            base64_decode(&args.first().unwrap_or(&JsValue::Undefined).display())
                .map(JsValue::String)
        }),
    );
    window.insert(
        "btoa".into(),
        native("btoa", Some(1), move |args| {
            Ok(JsValue::String(base64_encode(
                args.first()
                    .unwrap_or(&JsValue::Undefined)
                    .display()
                    .as_bytes(),
            )))
        }),
    );
}

fn url_constructor() -> JsValue {
    JsValue::Native(Rc::new(
        NativeFunction::new("URL", None, move |args| {
            let input = args.first().unwrap_or(&JsValue::Undefined).display();
            let base = args.get(1).map(JsValue::display);
            Ok(url_object(&resolve_url(&input, base.as_deref())))
        })
        .with_property(
            "canParse",
            native("URL.canParse", None, move |args| {
                Ok(JsValue::Bool(static_url_href(args).is_some()))
            }),
        )
        .with_property(
            "parse",
            native("URL.parse", None, move |args| {
                Ok(static_url_href(args)
                    .map(|href| url_object(&href))
                    .unwrap_or(JsValue::Null))
            }),
        )
        .with_property(
            "createObjectURL",
            native("URL.createObjectURL", Some(1), move |args| {
                Ok(create_object_url(
                    args.first().unwrap_or(&JsValue::Undefined),
                ))
            }),
        )
        .with_property(
            "revokeObjectURL",
            native("URL.revokeObjectURL", Some(1), move |args| {
                revoke_object_url(&args[0]);
                Ok(JsValue::Undefined)
            }),
        ),
    ))
}

fn response_constructor() -> JsValue {
    JsValue::Native(Rc::new(
        NativeFunction::new("Response", None, move |args| Ok(response_from_args(args)))
            .with_property(
                "json",
                native("Response.json", None, move |args| {
                    Ok(response_json_from_args(args))
                }),
            )
            .with_property(
                "redirect",
                native("Response.redirect", None, move |args| {
                    Ok(response_redirect_from_args(args))
                }),
            )
            .with_property(
                "error",
                native("Response.error", Some(0), move |_| {
                    Ok(response_error_object())
                }),
            ),
    ))
}

fn response_from_args(args: &[JsValue]) -> JsValue {
    let body = match args.first() {
        None | Some(JsValue::Undefined | JsValue::Null) => String::new(),
        Some(value) => value.display(),
    };
    let init = args.get(1).unwrap_or(&JsValue::Undefined);
    let status = response_init_value(init, "status")
        .and_then(|value| js_u16(&value))
        .unwrap_or(200);
    response_object(ResponseFields {
        status,
        status_text: response_init_value(init, "statusText")
            .map(|value| value.display())
            .unwrap_or_default(),
        headers: response_init_value(init, "headers")
            .map(|value| headers_from_value(&value))
            .unwrap_or_default(),
        body,
        url: String::new(),
        method: None,
        body_used: false,
        response_type: None,
    })
}

fn response_json_from_args(args: &[JsValue]) -> JsValue {
    let init = args.get(1).unwrap_or(&JsValue::Undefined);
    response_object(ResponseFields {
        status: response_status_from_init(init, 200),
        status_text: response_init_value(init, "statusText")
            .map(|value| value.display())
            .unwrap_or_default(),
        headers: headers_with_default(response_headers_from_init(init)),
        body: json_body_from_value(args.first().unwrap_or(&JsValue::Null)),
        url: String::new(),
        method: None,
        body_used: false,
        response_type: None,
    })
}

fn response_redirect_from_args(args: &[JsValue]) -> JsValue {
    let url = args.first().unwrap_or(&JsValue::Undefined).display();
    let status = args
        .get(1)
        .and_then(js_u16)
        .filter(|status| matches!(*status, 301 | 302 | 303 | 307 | 308))
        .unwrap_or(302);
    response_object(ResponseFields {
        status,
        status_text: String::new(),
        headers: vec![("location".into(), url)],
        body: String::new(),
        url: String::new(),
        method: None,
        body_used: false,
        response_type: None,
    })
}

fn response_error_object() -> JsValue {
    response_object(ResponseFields {
        status: 0,
        status_text: String::new(),
        headers: Vec::new(),
        body: String::new(),
        url: String::new(),
        method: None,
        body_used: false,
        response_type: Some("error".into()),
    })
}

fn response_status_from_init(init: &JsValue, default: u16) -> u16 {
    response_init_value(init, "status")
        .and_then(|value| js_u16(&value))
        .unwrap_or(default)
}

fn response_headers_from_init(init: &JsValue) -> Vec<(String, String)> {
    response_init_value(init, "headers")
        .map(|value| headers_from_value(&value))
        .unwrap_or_default()
}

fn response_init_value(init: &JsValue, key: &str) -> Option<JsValue> {
    let JsValue::Object(obj) = init else {
        return None;
    };
    obj.borrow().get(key).cloned()
}

fn url_object(href: &str) -> JsValue {
    let parsed = parse_location(href);
    let search = parsed
        .get("search")
        .map(JsValue::display)
        .unwrap_or_default();
    let object = Rc::new(RefCell::new(parsed));
    object.borrow_mut().insert(
        "searchParams".into(),
        url_search_params_object(parse_search_params(&search)),
    );
    let object_for_string = object.clone();
    object.borrow_mut().insert(
        "toString".into(),
        native("URL.toString", Some(0), move |_| {
            Ok(object_for_string
                .borrow()
                .get("href")
                .cloned()
                .unwrap_or_else(|| JsValue::String(String::new())))
        }),
    );
    JsValue::Object(object)
}

fn create_object_url(value: &JsValue) -> JsValue {
    let kind = object_url_kind(value);
    let id = NEXT_OBJECT_URL_ID.with(|next| {
        let mut next = next.borrow_mut();
        let id = *next;
        *next += 1;
        id
    });
    let url = format!("blob:tetherscript://{kind}/{id}");
    OBJECT_URLS.with(|urls| {
        urls.borrow_mut().insert(url.clone());
    });
    JsValue::String(url)
}

fn object_url_kind(value: &JsValue) -> &'static str {
    let JsValue::Object(object) = value else {
        return "object";
    };
    let object = object.borrow();
    if object.contains_key("name") && object.contains_key("lastModified") {
        "file"
    } else if object.contains_key("__blobBytes") {
        "blob"
    } else {
        "object"
    }
}

fn revoke_object_url(value: &JsValue) {
    let url = value.display();
    OBJECT_URLS.with(|urls| {
        urls.borrow_mut().remove(&url);
    });
}

fn request_object(request: &FetchRequest) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    let headers = headers_object(request.headers.clone());
    {
        let mut obj = object.borrow_mut();
        obj.insert("url".into(), JsValue::String(request.url.clone()));
        obj.insert("method".into(), JsValue::String(request.method.clone()));
        obj.insert("headers".into(), headers.clone());
        obj.insert("bodyUsed".into(), JsValue::Bool(false));
        obj.insert(
            "__requestHeaders".into(),
            JsValue::Array(Rc::new(RefCell::new(
                request
                    .headers
                    .iter()
                    .map(|(name, value)| {
                        JsValue::Array(Rc::new(RefCell::new(vec![
                            JsValue::String(name.clone()),
                            JsValue::String(value.clone()),
                        ])))
                    })
                    .collect(),
            ))),
        );
        obj.insert(
            "body".into(),
            request
                .body
                .clone()
                .map(JsValue::String)
                .unwrap_or(JsValue::Null),
        );
    }
    let text_body = request.body.clone().unwrap_or_default();
    let text_object = object.clone();
    object.borrow_mut().insert(
        "text".into(),
        native("Request.text", Some(0), move |_| {
            mark_request_body_used(&text_object);
            Ok(fulfilled_thenable(JsValue::String(text_body.clone())))
        }),
    );

    let json_body = request.body.clone().unwrap_or_default();
    let json_object = object.clone();
    object.borrow_mut().insert(
        "json".into(),
        native("Request.json", Some(0), move |_| {
            mark_request_body_used(&json_object);
            Ok(fulfilled_thenable(parse_simple_json(&json_body)))
        }),
    );

    let buffer_body = request.body.clone().unwrap_or_default();
    let buffer_object = object.clone();
    object.borrow_mut().insert(
        "arrayBuffer".into(),
        native("Request.arrayBuffer", Some(0), move |_| {
            mark_request_body_used(&buffer_object);
            Ok(fulfilled_thenable(byte_array_from_text(&buffer_body)))
        }),
    );

    let blob_body = request.body.clone().unwrap_or_default();
    let blob_object = object.clone();
    let blob_headers = headers.clone();
    object.borrow_mut().insert(
        "blob".into(),
        native("Request.blob", Some(0), move |_| {
            mark_request_body_used(&blob_object);
            let mime_type = body_mime_type(&headers_from_value(&blob_headers));
            Ok(fulfilled_thenable(blob_from_text_body(
                &blob_body, mime_type,
            )))
        }),
    );

    let clone_object = object.clone();
    object.borrow_mut().insert(
        "clone".into(),
        native("Request.clone", Some(0), move |_| {
            Ok(request_object(&request_from_object_fields(
                &clone_object.borrow(),
            )))
        }),
    );
    JsValue::Object(object)
}

fn mark_request_body_used(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    object
        .borrow_mut()
        .insert("bodyUsed".into(), JsValue::Bool(true));
}

fn request_from_object_fields(obj: &HashMap<String, JsValue>) -> FetchRequest {
    FetchRequest {
        url: obj.get("url").map(JsValue::display).unwrap_or_default(),
        method: obj
            .get("method")
            .map(JsValue::display)
            .unwrap_or_else(|| "GET".into())
            .to_ascii_uppercase(),
        headers: obj
            .get("headers")
            .map(headers_from_value)
            .unwrap_or_default(),
        body: obj.get("body").and_then(request_body_from_value),
        aborted: obj.get("signal").is_some_and(signal_aborted),
    }
}

fn request_body_from_value(value: &JsValue) -> Option<String> {
    match value {
        JsValue::Undefined | JsValue::Null => None,
        value => Some(value.display()),
    }
}

fn headers_object(entries: Vec<(String, String)>) -> JsValue {
    let entries = Rc::new(RefCell::new(normalize_headers(entries)));
    let object = Rc::new(RefCell::new(HashMap::new()));
    let this_value = JsValue::Object(object.clone());
    let mut obj = object.borrow_mut();
    obj.insert("__headersBrand".into(), JsValue::Bool(true));
    let get_entries = entries.clone();
    obj.insert(
        "get".into(),
        native("Headers.get", Some(1), move |args| {
            let key = header_name_arg(args);
            Ok(get_entries
                .borrow()
                .iter()
                .find(|(name, _)| name == &key)
                .map(|(_, value)| JsValue::String(value.clone()))
                .unwrap_or(JsValue::Null))
        }),
    );
    let has_entries = entries.clone();
    obj.insert(
        "has".into(),
        native("Headers.has", Some(1), move |args| {
            let key = header_name_arg(args);
            Ok(JsValue::Bool(
                has_entries.borrow().iter().any(|(name, _)| name == &key),
            ))
        }),
    );
    let set_entries = entries.clone();
    obj.insert(
        "set".into(),
        native("Headers.set", Some(2), move |args| {
            let key = header_name_arg(args);
            let value = args.get(1).unwrap_or(&JsValue::Undefined).display();
            let mut entries = set_entries.borrow_mut();
            entries.retain(|(name, _)| name != &key);
            entries.push((key, value));
            Ok(JsValue::Undefined)
        }),
    );
    let append_entries = entries.clone();
    obj.insert(
        "append".into(),
        native("Headers.append", Some(2), move |args| {
            append_entries.borrow_mut().push((
                header_name_arg(args),
                args.get(1).unwrap_or(&JsValue::Undefined).display(),
            ));
            Ok(JsValue::Undefined)
        }),
    );
    let delete_entries = entries.clone();
    obj.insert(
        "delete".into(),
        native("Headers.delete", Some(1), move |args| {
            let key = header_name_arg(args);
            delete_entries.borrow_mut().retain(|(name, _)| name != &key);
            Ok(JsValue::Undefined)
        }),
    );
    let keys_entries = entries.clone();
    obj.insert(
        "keys".into(),
        native("Headers.keys", Some(0), move |_| {
            Ok(headers_keys_array(&keys_entries.borrow()))
        }),
    );
    let values_entries = entries.clone();
    obj.insert(
        "values".into(),
        native("Headers.values", Some(0), move |_| {
            Ok(headers_values_array(&values_entries.borrow()))
        }),
    );
    let entries_for_rows = entries.clone();
    obj.insert(
        "entries".into(),
        native("Headers.entries", Some(0), move |_| {
            Ok(headers_entries_array(&entries_for_rows.borrow()))
        }),
    );
    let for_each_entries = entries;
    obj.insert(
        "forEach".into(),
        native("Headers.forEach", None, move |args| {
            let callback = args
                .first()
                .cloned()
                .ok_or_else(|| "Headers.forEach: expected callback".to_string())?;
            let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            for (name, value) in for_each_entries.borrow().clone() {
                js::call_function_with_this(
                    callback.clone(),
                    this_arg.clone(),
                    &[
                        JsValue::String(value),
                        JsValue::String(name),
                        this_value.clone(),
                    ],
                )?;
            }
            Ok(JsValue::Undefined)
        }),
    );
    drop(obj);
    JsValue::Object(object)
}

fn headers_from_value(value: &JsValue) -> Vec<(String, String)> {
    match value {
        JsValue::Object(obj) => {
            let (is_headers, entries) = {
                let obj = obj.borrow();
                (
                    obj.get("__headersBrand").is_some(),
                    obj.get("entries").cloned(),
                )
            };
            if is_headers {
                if let Some(entries) = entries {
                    if let Ok(rows) = js::call_function_with_this(entries, value.clone(), &[]) {
                        return headers_from_value(&rows);
                    }
                }
            }
            obj.borrow()
                .iter()
                .filter_map(|(name, value)| {
                    if name.starts_with("__")
                        || matches!(value, JsValue::Native(_) | JsValue::Function(_))
                    {
                        None
                    } else {
                        Some((name.to_ascii_lowercase(), value.display()))
                    }
                })
                .collect()
        }
        JsValue::Array(items) => items
            .borrow()
            .iter()
            .filter_map(|item| match item {
                JsValue::Array(pair) => {
                    let pair = pair.borrow();
                    Some((
                        pair.first()?.display().to_ascii_lowercase(),
                        pair.get(1)?.display(),
                    ))
                }
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn header_name_arg(args: &[JsValue]) -> String {
    args.first()
        .unwrap_or(&JsValue::Undefined)
        .display()
        .to_ascii_lowercase()
}

fn headers_keys_array(entries: &[(String, String)]) -> JsValue {
    headers_js_array(
        entries
            .iter()
            .map(|(name, _)| JsValue::String(name.clone()))
            .collect(),
    )
}

fn headers_values_array(entries: &[(String, String)]) -> JsValue {
    headers_js_array(
        entries
            .iter()
            .map(|(_, value)| JsValue::String(value.clone()))
            .collect(),
    )
}

fn headers_entries_array(entries: &[(String, String)]) -> JsValue {
    headers_js_array(
        entries
            .iter()
            .map(|(name, value)| {
                headers_js_array(vec![
                    JsValue::String(name.clone()),
                    JsValue::String(value.clone()),
                ])
            })
            .collect(),
    )
}

fn headers_js_array(items: Vec<JsValue>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(items)))
}

fn normalize_headers(entries: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut normalized = Vec::<(String, String)>::new();
    for (name, value) in entries {
        let name = name.to_ascii_lowercase();
        if let Some((_, existing)) = normalized
            .iter_mut()
            .find(|(candidate, _)| candidate == &name)
        {
            *existing = value;
        } else {
            normalized.push((name, value));
        }
    }
    normalized
}

fn resolve_url(input: &str, base: Option<&str>) -> String {
    if input.contains("://") || input.starts_with("data:") {
        return input.to_string();
    }
    if let Some(base) = base {
        if let Some((origin, _)) = base.split_once("://") {
            let rest = &base[origin.len() + 3..];
            let host_end = rest.find('/').unwrap_or(rest.len());
            let origin = format!("{}://{}", origin, &rest[..host_end]);
            if input.starts_with('/') {
                return format!("{}{}", origin, input);
            }
            let base_path = rest
                .get(host_end..)
                .and_then(|path| path.rsplit_once('/').map(|(dir, _)| dir))
                .unwrap_or("");
            return format!("{}/{}{}", origin, base_path.trim_end_matches('/'), input);
        }
    }
    input.to_string()
}

fn static_url_href(args: &[JsValue]) -> Option<String> {
    let input = args.first()?.display();
    let base = args.get(1).map(JsValue::display);
    parse_static_url(&input, base.as_deref())
}

fn parse_static_url(input: &str, base: Option<&str>) -> Option<String> {
    if input.trim().is_empty() {
        return None;
    }
    if is_supported_absolute_url(input) {
        return Some(resolve_url(input, None));
    }
    let base = base.filter(|base| is_supported_absolute_url(base))?;
    let resolved = resolve_url(input, Some(base));
    is_supported_absolute_url(&resolved).then_some(resolved)
}

fn is_supported_absolute_url(input: &str) -> bool {
    if let Some(rest) = input.strip_prefix("data:") {
        return !rest.trim().is_empty();
    }
    let Some((scheme, rest)) = input.split_once("://") else {
        return false;
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    is_url_scheme(scheme) && !host.is_empty() && !host.chars().any(char::is_whitespace)
}

fn is_url_scheme(scheme: &str) -> bool {
    let mut chars = scheme.chars();
    matches!(chars.next(), Some(first) if first.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

fn parse_search_params(input: &str) -> Vec<(String, String)> {
    let query = input
        .strip_prefix('?')
        .or_else(|| input.split_once('?').map(|(_, query)| query))
        .unwrap_or(input);
    if query.is_empty() || query == "null" || query == "undefined" {
        return Vec::new();
    }
    query
        .split('&')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (name, value) = part.split_once('=').unwrap_or((part, ""));
            (percent_decode(name), percent_decode(value))
        })
        .collect()
}

fn url_search_params_entries(value: &JsValue) -> Vec<(String, String)> {
    match value {
        JsValue::Array(items) => search_params_from_pairs(&items.borrow()),
        JsValue::Object(obj) => search_params_from_object(value, obj),
        JsValue::String(input) => parse_search_params(input),
        other => parse_search_params(&other.display()),
    }
}

fn search_params_from_pairs(items: &[JsValue]) -> Vec<(String, String)> {
    items
        .iter()
        .filter_map(|item| match item {
            JsValue::Array(pair) => {
                let pair = pair.borrow();
                Some((pair.first()?.display(), pair.get(1)?.display()))
            }
            _ => None,
        })
        .collect()
}

fn search_params_from_object(
    value: &JsValue,
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
) -> Vec<(String, String)> {
    let (is_params, entries_fn) = {
        let obj = obj.borrow();
        (
            obj.get("__urlSearchParamsBrand").is_some(),
            obj.get("entries").cloned(),
        )
    };
    if is_params {
        if let Some(entries_fn) = entries_fn {
            if let Ok(rows) = js::call_function_with_this(entries_fn, value.clone(), &[]) {
                return url_search_params_entries(&rows);
            }
        }
    }
    let mut entries = obj
        .borrow()
        .iter()
        .filter_map(|(name, value)| {
            if name.starts_with("__")
                || matches!(
                    value,
                    JsValue::Native(_) | JsValue::Function(_) | JsValue::BoundFunction(_)
                )
            {
                None
            } else {
                Some((name.clone(), value.display()))
            }
        })
        .collect::<Vec<_>>();
    entries.sort_by(|(left, _), (right, _)| left.cmp(right));
    entries
}

fn url_search_params_object(entries: Vec<(String, String)>) -> JsValue {
    let entries = Rc::new(RefCell::new(entries));
    let object = Rc::new(RefCell::new(HashMap::new()));
    let this_value = JsValue::Object(object.clone());
    let mut obj = object.borrow_mut();
    obj.insert("__urlSearchParamsBrand".into(), JsValue::Bool(true));
    obj.insert(
        "size".into(),
        JsValue::Number(entries.borrow().len() as f64),
    );
    let get_entries = entries.clone();
    obj.insert(
        "get".into(),
        native("URLSearchParams.get", Some(1), move |args| {
            let key = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(get_entries
                .borrow()
                .iter()
                .find(|(name, _)| name == &key)
                .map(|(_, value)| JsValue::String(value.clone()))
                .unwrap_or(JsValue::Null))
        }),
    );
    let has_entries = entries.clone();
    obj.insert(
        "has".into(),
        native("URLSearchParams.has", Some(1), move |args| {
            let key = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(JsValue::Bool(
                has_entries.borrow().iter().any(|(name, _)| name == &key),
            ))
        }),
    );
    let all_entries = entries.clone();
    obj.insert(
        "getAll".into(),
        native("URLSearchParams.getAll", Some(1), move |args| {
            let key = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(search_params_array(
                all_entries
                    .borrow()
                    .iter()
                    .filter(|(name, _)| name == &key)
                    .map(|(_, value)| JsValue::String(value.clone()))
                    .collect(),
            ))
        }),
    );
    let set_entries = entries.clone();
    let set_object = object.clone();
    obj.insert(
        "set".into(),
        native("URLSearchParams.set", Some(2), move |args| {
            let key = args.first().unwrap_or(&JsValue::Undefined).display();
            let value = args.get(1).unwrap_or(&JsValue::Undefined).display();
            let mut entries = set_entries.borrow_mut();
            if entries.iter().any(|(name, _)| name == &key) {
                let mut kept = false;
                for (name, existing) in entries.iter_mut() {
                    if name == &key && !kept {
                        *existing = value.clone();
                        kept = true;
                    }
                }
                kept = false;
                entries.retain(|(name, _)| {
                    if name != &key {
                        true
                    } else if kept {
                        false
                    } else {
                        kept = true;
                        true
                    }
                });
            } else {
                entries.push((key, value));
            }
            set_search_params_size(&set_object, entries.len());
            Ok(JsValue::Undefined)
        }),
    );
    let append_entries = entries.clone();
    let append_object = object.clone();
    obj.insert(
        "append".into(),
        native("URLSearchParams.append", Some(2), move |args| {
            let mut entries = append_entries.borrow_mut();
            entries.push((
                args.first().unwrap_or(&JsValue::Undefined).display(),
                args.get(1).unwrap_or(&JsValue::Undefined).display(),
            ));
            set_search_params_size(&append_object, entries.len());
            Ok(JsValue::Undefined)
        }),
    );
    let delete_entries = entries.clone();
    let delete_object = object.clone();
    obj.insert(
        "delete".into(),
        native("URLSearchParams.delete", Some(1), move |args| {
            let key = args.first().unwrap_or(&JsValue::Undefined).display();
            let mut entries = delete_entries.borrow_mut();
            entries.retain(|(name, _)| name != &key);
            set_search_params_size(&delete_object, entries.len());
            Ok(JsValue::Undefined)
        }),
    );
    let keys_entries = entries.clone();
    obj.insert(
        "keys".into(),
        native("URLSearchParams.keys", Some(0), move |_| {
            Ok(search_params_array(
                keys_entries
                    .borrow()
                    .iter()
                    .map(|(name, _)| JsValue::String(name.clone()))
                    .collect(),
            ))
        }),
    );
    let values_entries = entries.clone();
    obj.insert(
        "values".into(),
        native("URLSearchParams.values", Some(0), move |_| {
            Ok(search_params_array(
                values_entries
                    .borrow()
                    .iter()
                    .map(|(_, value)| JsValue::String(value.clone()))
                    .collect(),
            ))
        }),
    );
    let row_entries = entries.clone();
    obj.insert(
        "entries".into(),
        native("URLSearchParams.entries", Some(0), move |_| {
            Ok(search_params_entries_array(&row_entries.borrow()))
        }),
    );
    let for_each_entries = entries.clone();
    let for_each_this = this_value.clone();
    obj.insert(
        "forEach".into(),
        native("URLSearchParams.forEach", None, move |args| {
            let callback = args
                .first()
                .cloned()
                .ok_or_else(|| "URLSearchParams.forEach: expected callback".to_string())?;
            let this_arg = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            for (name, value) in for_each_entries.borrow().clone() {
                js::call_function_with_this(
                    callback.clone(),
                    this_arg.clone(),
                    &[
                        JsValue::String(value),
                        JsValue::String(name),
                        for_each_this.clone(),
                    ],
                )?;
            }
            Ok(JsValue::Undefined)
        }),
    );
    let sort_entries = entries.clone();
    let sort_object = object.clone();
    obj.insert(
        "sort".into(),
        native("URLSearchParams.sort", Some(0), move |_| {
            let mut entries = sort_entries.borrow_mut();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            set_search_params_size(&sort_object, entries.len());
            Ok(JsValue::Undefined)
        }),
    );
    let string_entries = entries.clone();
    obj.insert(
        "toString".into(),
        native("URLSearchParams.toString", Some(0), move |_| {
            Ok(JsValue::String(search_params_string(
                &string_entries.borrow(),
            )))
        }),
    );
    let json_entries = entries.clone();
    obj.insert(
        "toJSON".into(),
        native("URLSearchParams.toJSON", Some(0), move |_| {
            Ok(JsValue::String(search_params_string(
                &json_entries.borrow(),
            )))
        }),
    );
    drop(obj);
    this_value
}

fn set_search_params_size(object: &Rc<RefCell<HashMap<String, JsValue>>>, len: usize) {
    object
        .borrow_mut()
        .insert("size".into(), JsValue::Number(len as f64));
}

fn search_params_entries_array(entries: &[(String, String)]) -> JsValue {
    search_params_array(
        entries
            .iter()
            .map(|(name, value)| {
                search_params_array(vec![
                    JsValue::String(name.clone()),
                    JsValue::String(value.clone()),
                ])
            })
            .collect(),
    )
}

fn search_params_array(items: Vec<JsValue>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(items)))
}

fn search_params_string(entries: &[(String, String)]) -> String {
    entries
        .iter()
        .map(|(name, value)| format!("{}={}", percent_encode(name), percent_encode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn abort_controller_object() -> JsValue {
    let signal = Rc::new(RefCell::new(HashMap::from([
        ("aborted".into(), JsValue::Bool(false)),
        ("reason".into(), JsValue::Null),
        ("onabort".into(), JsValue::Null),
    ])));
    install_abort_signal_methods(&signal);
    let mut obj = HashMap::new();
    obj.insert("signal".into(), JsValue::Object(signal.clone()));
    obj.insert(
        "abort".into(),
        native("AbortController.abort", None, move |args| {
            if signal.borrow().get("aborted").is_some_and(JsValue::truthy) {
                return Ok(JsValue::Undefined);
            }
            let reason = args
                .first()
                .cloned()
                .unwrap_or(JsValue::String("AbortError".into()));
            {
                let mut signal = signal.borrow_mut();
                signal.insert("aborted".into(), JsValue::Bool(true));
                signal.insert("reason".into(), reason);
            }
            dispatch_abort_signal_event(&signal, JsValue::String("abort".into()))?;
            Ok(JsValue::Undefined)
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn install_abort_signal_methods(signal: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let add_signal = signal.clone();
    signal.borrow_mut().insert(
        "addEventListener".into(),
        native("AbortSignal.addEventListener", Some(2), move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            if event_type == "abort" {
                let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
                let mut signal = add_signal.borrow_mut();
                let listeners = signal
                    .entry("__listeners:abort".into())
                    .or_insert_with(|| JsValue::Array(Rc::new(RefCell::new(Vec::new()))));
                if let JsValue::Array(items) = listeners {
                    items.borrow_mut().push(listener);
                }
            }
            Ok(JsValue::Undefined)
        }),
    );
    let remove_signal = signal.clone();
    signal.borrow_mut().insert(
        "removeEventListener".into(),
        native("AbortSignal.removeEventListener", Some(2), move |args| {
            let event_type = args.first().unwrap_or(&JsValue::Undefined).display();
            let listener = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            if event_type == "abort" {
                if let Some(JsValue::Array(items)) = remove_signal.borrow().get("__listeners:abort")
                {
                    items.borrow_mut().retain(|item| *item != listener);
                }
            }
            Ok(JsValue::Undefined)
        }),
    );
    let dispatch_signal = signal.clone();
    signal.borrow_mut().insert(
        "dispatchEvent".into(),
        native("AbortSignal.dispatchEvent", Some(1), move |args| {
            let event = args.first().cloned().unwrap_or(JsValue::Undefined);
            dispatch_abort_signal_event(&dispatch_signal, event)
        }),
    );
}

fn dispatch_abort_signal_event(
    signal: &Rc<RefCell<HashMap<String, JsValue>>>,
    event: JsValue,
) -> Result<JsValue, String> {
    let Some(event_type) = event_type(&event) else {
        return Ok(JsValue::Bool(true));
    };
    if event_type != "abort" {
        return Ok(JsValue::Bool(true));
    }
    let this_value = JsValue::Object(signal.clone());
    let event = normalize_event(event, "abort", this_value.clone(), this_value.clone());
    let listeners = signal
        .borrow()
        .get("__listeners:abort")
        .and_then(|value| match value {
            JsValue::Array(items) => Some(items.borrow().clone()),
            _ => None,
        })
        .unwrap_or_default();
    for listener in listeners {
        call_dom_listener(listener, this_value.clone(), event.clone())?;
    }
    if let Some(handler) = signal.borrow().get("onabort").cloned() {
        if !matches!(handler, JsValue::Null | JsValue::Undefined) {
            call_dom_listener(handler, this_value, event.clone())?;
        }
    }
    Ok(JsValue::Bool(!event_flag(&event, "defaultPrevented")))
}

fn percent_decode(input: &str) -> String {
    let mut out = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = u8::from_str_radix(&input[i + 1..i + 3], 16) {
                out.push(hex as char);
                i += 3;
                continue;
            }
        }
        out.push(if bytes[i] == b'+' {
            ' '
        } else {
            bytes[i] as char
        });
        i += 1;
    }
    out
}

fn percent_encode(input: &str) -> String {
    input
        .bytes()
        .flat_map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                vec![b as char]
            }
            b' ' => vec!['+'],
            _ => format!("%{:02X}", b).chars().collect(),
        })
        .collect()
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0b0000_0011) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[(((b1 & 0b0000_1111) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(b2 & 0b0011_1111) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn base64_decode(input: &str) -> Result<String, String> {
    let mut bits = 0u32;
    let mut bit_count = 0u8;
    let mut out = Vec::new();
    for ch in input.chars().filter(|ch| !ch.is_whitespace()) {
        if ch == '=' {
            break;
        }
        let value = match ch {
            'A'..='Z' => ch as u8 - b'A',
            'a'..='z' => ch as u8 - b'a' + 26,
            '0'..='9' => ch as u8 - b'0' + 52,
            '+' => 62,
            '/' => 63,
            _ => return Err("atob: invalid base64 input".into()),
        } as u32;
        bits = (bits << 6) | value;
        bit_count += 6;
        if bit_count >= 8 {
            bit_count -= 8;
            out.push(((bits >> bit_count) & 0xff) as u8);
        }
    }
    String::from_utf8(out).map_err(|_| "atob: decoded bytes are not UTF-8".into())
}

pub fn compatibility_report_to_value(_args: &[Value]) -> Result<Value, String> {
    let features = [
        "document",
        "window",
        "self",
        "selectors",
        "attributes",
        "textContent",
        "innerHTML",
        "parentNode",
        "childNodes",
        "siblingNavigation",
        "createElement",
        "attachShadow",
        "shadowRoot",
        "append",
        "remove",
        "events",
        "this",
        "typeof",
        "functionExpressions",
        "forLoops",
        "location",
        "document.cookie",
        "navigator",
        "setTimeout",
        "clearTimeout",
        "setInterval",
        "clearInterval",
        "queueMicrotask",
        "requestAnimationFrame",
        "cancelAnimationFrame",
        "deterministicTimers",
        "localStorage",
        "sessionStorage",
        "Storage.getItem",
        "Storage.setItem",
        "Storage.removeItem",
        "Storage.clear",
        "Storage.key",
        "Storage.length",
        "getComputedStyle",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "accessibilityTree",
        "ariaRoles",
        "focusableElements",
        "formData",
        "formSubmit",
        "submitEvents",
        "selectionStart",
        "setSelectionRange",
        "typeText",
        "Selection",
        "Range",
        "contenteditable",
        "DOMContentLoaded",
        "loadEvent",
        "history.pushState",
        "history.replaceState",
        "history.back",
        "popstate",
        "MutationObserver",
        "IntersectionObserver",
        "ResizeObserver",
        "ReportingObserver",
        "URL",
        "URLSearchParams",
        "AbortController",
        "Request",
        "Headers",
        "XMLHttpRequest",
        "WebSocket",
        "EventSource",
        "atob",
        "btoa",
    ];
    Ok(Value::List(Rc::new(RefCell::new(
        features
            .into_iter()
            .map(|s| Value::Str(Rc::new(s.into())))
            .collect(),
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_scripts_can_read_document_and_console_log() {
        let html = "<main id='app'><h1>Hello</h1><script>console.log(document.getElementById('app').children.length); document.querySelector('h1').textContent;</script></main>";
        let result = run_html_scripts(html).unwrap();
        assert_eq!(result.value, JsValue::String("Hello".into()));
        assert_eq!(result.console, vec!["2".to_string()]);
    }

    #[test]
    fn script_text_content_is_readable_without_eager_dom_strings() {
        let source = "let x=1;".repeat(4096);
        let html = format!("<script id='bundle'>{source}</script><main></main>");
        let result = eval_with_dom(
            &html,
            "document.getElementById('bundle').textContent.length;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::Number(source.len() as f64));
    }

    #[test]
    fn eval_with_dom_exposes_selectors_and_attributes() {
        let result = eval_with_dom("<p class='note' id='x'>Hi</p>", "let p=document.querySelector('.note'); p.setAttribute('data-ok','yes'); p.getAttribute('id') + ':' + p.textContent;").unwrap();
        assert_eq!(result.value, JsValue::String("x:Hi".into()));
        match &result.document.children[0] {
            Node::Element(el) => assert_eq!(el.attrs.get("data-ok"), Some(&"yes".to_string())),
            Node::Text(_) => panic!("expected element"),
        }
    }

    #[test]
    fn dom_property_assignment_and_mutation_apis_update_document() {
        let result = eval_with_dom(
            "<main id='app'><p>old</p></main>",
            "let app=document.getElementById('app'); let p=document.querySelector('p'); p.textContent='new'; let span=document.createElement('span'); span.textContent='!'; app.appendChild(span); document.getElementById('app').children.length;",
        ).unwrap();
        assert_eq!(result.value, JsValue::Number(2.0));
        let text = result
            .document
            .children
            .iter()
            .map(browser::text_content)
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.contains("new"));
        assert!(text.contains("!"));
    }

    #[test]
    fn event_listeners_property_handlers_this_and_event_target_work() {
        let result = eval_with_dom(
            "<button id='go'>old</button>",
            "let btn=document.getElementById('go'); let seen=''; btn.addEventListener('click', function(e){ seen=e.type + ':' + e.target.id + ':' + this.id; this.textContent='clicked'; }); btn.click(); seen + ':' + document.getElementById('go').textContent;",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("click:go:go:clicked".into()));

        let result = eval_with_dom(
            "<button id='go'>old</button>",
            "let btn=document.getElementById('go'); btn.onclick=function(e){ this.setAttribute('data-clicked', e.type); }; btn.dispatchEvent({type:'click'}); document.getElementById('go').getAttribute('data-clicked');",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("click".into()));
    }

    #[test]
    fn remove_event_listener_and_typeof_work() {
        let result = eval_with_dom(
            "<button>ok</button>",
            "let count=0; function inc(){ count=count+1; } let b=document.querySelector('button'); b.addEventListener('click', inc); b.removeEventListener('click', inc); b.click(); count + ':' + typeof missingName + ':' + typeof document;",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("0:undefined:object".into()));
    }

    #[test]
    fn location_and_navigator_globals_are_available() {
        let result = eval_with_dom(
            "<main></main>",
            "navigator.userAgent.length > 0 && location.pathname == '/' && window.location.origin == 'http://localhost' && window.navigator.language == 'en-US';",
        ).unwrap();
        assert_eq!(result.value, JsValue::Bool(true));
    }

    #[test]
    fn window_self_and_storage_globals_are_available() {
        let result = eval_with_dom(
            "<main></main>",
            "window.window === window && self === window && window.localStorage === localStorage && window.sessionStorage === sessionStorage;",
        ).unwrap();
        assert_eq!(result.value, JsValue::Bool(true));
    }

    #[test]
    fn classic_for_loop_supports_node_list_iteration() {
        let result = eval_with_dom(
            "<ul><li>A</li><li>B</li><li>C</li></ul>",
            "let items=document.querySelectorAll('li'); let text=''; for (let i=0; i<items.length; i=i+1) { text = text + items[i].textContent; } text;",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("ABC".into()));
    }

    #[test]
    fn set_timeout_callbacks_drain_after_script_by_deadline_then_registration() {
        let result = eval_with_dom(
            "<button id='go'>old</button>",
            "let order=''; setTimeout(function(){ order=order+'A'; document.getElementById('go').textContent='done'; }, 50); setTimeout(function(){ order=order+'B'; }, 0); order='sync'; order;",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(browser::text_content(&result.document.children[0]), "done");

        let result = eval_with_dom(
            "<main></main>",
            "let order=''; setTimeout(function(){ order=order+'A'; }, 10); setTimeout(function(){ order=order+'B'; }, 0); setTimeout(function(){ order=order+'C'; console.log(order); }, 10); 'sync';",
        ).unwrap();
        assert_eq!(result.console, vec!["BAC".to_string()]);
    }

    #[test]
    fn clear_timeout_cancels_pending_callback_and_timeout_args_work() {
        let result = eval_with_dom(
            "<main></main>",
            "let seen=''; let first=setTimeout(function(){ seen='bad'; }, 0); clearTimeout(first); window.setTimeout(function(a,b){ seen=a+b; console.log(this.navigator.language + ':' + seen); }, 0, 'O', 'K'); 'sync';",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(result.console, vec!["en-US:OK".to_string()]);
    }

    #[test]
    fn set_interval_repeats_until_cleared_with_window_this_and_args() {
        let result = eval_with_dom(
            "<main></main>",
            "let count=0; let id=setInterval(function(a,b){ count=count+1; console.log((this === window) + ':' + a + b + ':' + count); if (count === 3) { clearInterval(id); } }, 0, 'O', 'K'); 'sync';",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(
            result.console,
            vec![
                "true:OK:1".to_string(),
                "true:OK:2".to_string(),
                "true:OK:3".to_string()
            ]
        );
    }

    #[test]
    fn set_interval_reschedules_by_delay_between_timeout_deadlines() {
        let result = eval_with_dom(
            "<main></main>",
            "let order=''; let count=0; let id=setInterval(function(){ count=count+1; order=order+'I'; if (count === 2) { clearInterval(id); } }, 10); setTimeout(function(){ order=order+'T'; }, 15); setTimeout(function(){ console.log(order); }, 25); 'sync';",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(result.console, vec!["ITI".to_string()]);
    }

    #[test]
    fn clear_interval_cancels_pending_interval() {
        let result = eval_with_dom(
            "<main></main>",
            "let seen='ok'; let id=setInterval(function(){ seen='bad'; }, 0); clearInterval(id); setTimeout(function(){ console.log(seen); }, 0); 'sync';",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(result.console, vec!["ok".to_string()]);
    }

    #[test]
    fn uncleared_interval_hits_deterministic_drain_limit() {
        let error = match eval_with_dom("<main></main>", "setInterval(function(){}, 0); 'sync';") {
            Ok(_) => panic!("expected uncleared interval to hit drain limit"),
            Err(error) => error,
        };
        assert!(error.contains("setInterval: exceeded deterministic drain limit"));
    }

    #[test]
    fn local_storage_implements_minimal_storage_api() {
        let result = eval_with_dom(
            "<main></main>",
            "localStorage.setItem('a', 1); localStorage.setItem('b', 'two'); localStorage.setItem('a', 'one'); let before=localStorage.length + ':' + localStorage.key(0) + ':' + localStorage.getItem('a') + ':' + localStorage.getItem('missing'); localStorage.removeItem('b'); let after=localStorage.length + ':' + localStorage.key(1); localStorage.clear(); before + '|' + after + '|' + localStorage.length + ':' + localStorage.getItem('a');",
        ).unwrap();
        assert_eq!(
            result.value,
            JsValue::String("2:a:one:null|1:null|0:null".into())
        );
    }

    #[test]
    fn local_storage_dispatches_storage_events_for_set_remove_clear() {
        let result = eval_with_dom(
            "<main></main>",
            "let seen=[]; window.addEventListener('storage', function(e){ seen.push(e.type + ':' + e.key + ':' + e.oldValue + ':' + e.newValue + ':' + e.url + ':' + (e.storageArea === localStorage) + ':' + (e.target === window) + ':' + (e.currentTarget === window) + ':' + (this === window)); }); localStorage.setItem('a','1'); localStorage.setItem('a','2'); localStorage.removeItem('a'); localStorage.setItem('b','3'); localStorage.clear(); seen.join('|');",
        )
        .unwrap();
        assert_eq!(
            result.value,
            JsValue::String(
                "storage:a:null:1:http://localhost/:true:true:true:true|storage:a:1:2:http://localhost/:true:true:true:true|storage:a:2:null:http://localhost/:true:true:true:true|storage:b:null:3:http://localhost/:true:true:true:true|storage:null:null:null:http://localhost/:true:true:true:true".into()
            )
        );
    }

    #[test]
    fn local_storage_skips_storage_event_for_same_value_set() {
        let result = eval_with_dom(
            "<main></main>",
            "let count=0; window.addEventListener('storage', function(){ count=count+1; }); localStorage.setItem('same','x'); localStorage.setItem('same','x'); count + ':' + localStorage.length + ':' + localStorage.getItem('same');",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("1:1:x".into()));
    }

    #[test]
    fn window_onstorage_receives_local_storage_event() {
        let result = eval_with_dom(
            "<main></main>",
            "let seen=''; window.onstorage=function(e){ seen=e.key + ':' + e.newValue + ':' + (this === window); }; localStorage.setItem('mode','edit'); seen;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("mode:edit:true".into()));
    }

    #[test]
    fn session_storage_is_separate_from_local_storage_and_per_eval() {
        let result = eval_with_dom(
            "<main></main>",
            "localStorage.setItem('shared', 'local'); sessionStorage.setItem('shared', 'session'); localStorage.getItem('shared') + ':' + sessionStorage.getItem('shared') + ':' + localStorage.length + ':' + sessionStorage.length;",
        ).unwrap();
        assert_eq!(result.value, JsValue::String("local:session:1:1".into()));

        let result = eval_with_dom(
            "<main></main>",
            "localStorage.getItem('shared') === null && sessionStorage.length === 0;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::Bool(true));
    }

    #[test]
    fn stateful_eval_round_trips_browser_state_network_and_html() {
        let result = eval_with_dom_state(
            "<main id='app'><span>old</span></main>",
            "let before=document.cookie + ':' + localStorage.getItem('token') + ':' + sessionStorage.getItem('tab'); document.cookie='sid=new'; document.cookie='theme=light'; localStorage.setItem('token', 'new-token'); localStorage.setItem('mode', 'edit'); sessionStorage.removeItem('tab'); sessionStorage.setItem('step', '2'); document.querySelector('span').textContent='updated'; fetch('/api/state'); let xhr=XMLHttpRequest(); xhr.open('post','/api/xhr'); xhr.send('body'); before;",
            BrowserJsState {
                cookies: vec![("sid".into(), "old".into())],
                set_cookies: Vec::new(),
                local_storage: vec![("token".into(), "old-token".into())],
                session_storage: vec![("tab".into(), "first".into())],
                ..BrowserJsState::default()
            },
        )
        .unwrap();

        assert_eq!(
            result.value,
            JsValue::String("sid=old:old-token:first".into())
        );
        assert_eq!(
            result.state.cookies,
            vec![
                ("sid".to_string(), "new".to_string()),
                ("theme".to_string(), "light".to_string()),
            ]
        );
        assert_eq!(
            result.state.local_storage,
            vec![
                ("token".to_string(), "new-token".to_string()),
                ("mode".to_string(), "edit".to_string()),
            ]
        );
        assert_eq!(
            result.state.session_storage,
            vec![("step".to_string(), "2".to_string())]
        );
        assert_eq!(
            result.network,
            vec![
                BrowserJsNetworkEvent {
                    method: "GET".into(),
                    url: "http://localhost/api/state".into(),
                    status: Some(200),
                    route_result: None,
                },
                BrowserJsNetworkEvent {
                    method: "POST".into(),
                    url: "http://localhost/api/xhr".into(),
                    status: Some(200),
                    route_result: None,
                },
            ]
        );
        assert!(result.html.contains("<span>updated</span>"));
    }

    #[test]
    fn computed_styles_geometry_and_offsets_are_exposed() {
        let result = eval_with_dom(
            "<style>#box { width: 12px; height: 4px; color: red; }</style><main><div id='box' style='margin-top: 2px'>Hi</div></main>",
            "let box=document.getElementById('box'); let style=getComputedStyle(box); let rect=box.getBoundingClientRect(); style.color + ':' + style.getPropertyValue('margin-top') + ':' + rect.width + ':' + rect.height + ':' + box.offsetWidth + ':' + box.offsetHeight;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("red:2px:12:4:12:4".into()));
    }

    #[test]
    fn accessibility_tree_exposes_roles_names_and_focusability() {
        let result = eval_with_dom(
            "<main><button aria-label='Save'></button><a href='/x'>Read</a><img alt='Logo'></main>",
            "let tree=getAccessibilityTree(); let main=tree[0]; let button=main.children[0]; let link=main.children[1]; let img=main.children[2]; main.role + ':' + button.role + ':' + button.name + ':' + button.focusable + ':' + link.role + ':' + img.name;",
        )
        .unwrap();
        assert_eq!(
            result.value,
            JsValue::String("main:button:Save:true:link:Logo".into())
        );
    }

    #[test]
    fn forms_collect_values_and_submit_events_can_cancel() {
        let result = eval_with_dom(
            "<form id='f' action='/save' method='post'><input name='q' value='rust'><input type='checkbox' name='ok' checked><input type='checkbox' name='skip'><textarea name='body'>Hi</textarea></form>",
            "let form=document.getElementById('f'); let data=form.collectFormData(); let submitted=form.requestSubmit(); form.method + ':' + submitted.action + ':' + submitted.method + ':' + data.get('q') + ':' + data.get('ok') + ':' + data.get('skip') + ':' + submitted.data.get('body');",
        )
        .unwrap();
        assert_eq!(
            result.value,
            JsValue::String("post:/save:post:rust:on:null:Hi".into())
        );

        let result = eval_with_dom(
            "<form id='f'><input name='q' value='rust'></form>",
            "let form=document.getElementById('f'); let seen=''; form.addEventListener('submit', function(e){ seen=e.type; e.preventDefault(); }); let submitted=form.requestSubmit(); seen + ':' + submitted;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("submit:false".into()));
    }

    #[test]
    fn navigation_lifecycle_history_and_location_are_available() {
        let result = eval_with_dom(
            "<main></main>",
            "let events=''; document.addEventListener('DOMContentLoaded', function(e){ events=events+'D'; }); window.onload=function(){ events=events+'L'; console.log(events); }; events;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String(String::new()));
        assert_eq!(result.console, vec!["DL".to_string()]);

        let result = eval_with_dom(
            "<main></main>",
            "let pops=0; window.addEventListener('popstate', function(){ pops=pops+1; }); history.pushState({page:1}, '', '/first?x=1#top'); let first=location.pathname + location.search + location.hash + ':' + history.length; history.pushState({}, '', '/second'); history.back(); first + '|' + location.pathname + ':' + pops;",
        )
        .unwrap();
        assert_eq!(
            result.value,
            JsValue::String("/first?x=1#top:2|/first:1".into())
        );
    }

    #[test]
    fn fetch_api_returns_response_with_thenable_promise() {
        let result = eval_with_dom(
            "<main></main>",
            "let seen = ''; fetch('/api/test').then(function(r){ seen = r.status + ':' + r.ok + ':' + r.statusText; console.log(seen); }); seen;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String(String::new()));
        assert_eq!(result.console, vec!["200:true:OK".to_string()]);
    }

    #[test]
    fn fetch_response_text_and_json_work() {
        let result = eval_with_dom(
            "<main></main>",
            "fetch('/api/data').then(function(r){ r.json().then(function(data){ console.log(r.url + ':' + r.status + ':' + r.ok + ':' + data.url); }); }); 'sync';",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(
            result.console,
            vec!["http://localhost/api/data:200:true:http://localhost/api/data".to_string()]
        );

        let result = eval_with_dom(
            "<main></main>",
            "fetch('/api/x').then(function(r){ r.text().then(function(text){ console.log(text.length > 0); }); }); 'sync';",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(result.console, vec!["true".to_string()]);
    }

    #[test]
    fn fetch_handles_404_and_data_urls() {
        let result = eval_with_dom(
            "<main></main>",
            "fetch('/not-found').then(function(r){ console.log(r.status + ':' + r.ok); }); 'sync';",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(result.console, vec!["404:false".to_string()]);

        let result = eval_with_dom(
            "<main></main>",
            "fetch('data:text/plain,hello').then(function(r){ console.log(r.url + ':' + r.ok); }); 'sync';",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(
            result.console,
            vec!["data:text/plain,hello:true".to_string()]
        );
    }

    #[test]
    fn fetch_accepts_request_init_headers_body_and_abort_signal() {
        let result = eval_with_dom(
            "<main></main>",
            "let headers=Headers({Accept:'application/json'}); headers.set('X-Test','yes'); let req=Request('/api/post', {method:'post', headers:{'x-test':'yes'}, body:'payload'}); fetch(req).then(function(r){ console.log(r.method + ':' + r.headers.get('x-test') + ':' + r.bodyUsed + ':' + headers.get('x-test')); }); let ac=AbortController(); ac.abort(); fetch('/api/cancel', {signal:ac.signal}).catch(function(e){ console.log(e); }); 'sync';",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(
            result.console,
            vec!["POST:yes:true:yes".to_string(), "AbortError".to_string()]
        );
    }

    #[test]
    fn mutation_observer_can_be_constructed_and_used() {
        let result = eval_with_dom(
            "<div id='target'></div>",
            "let called = false; let obs = MutationObserver(function(mutations){ called = true; }); let target = document.getElementById('target'); obs.observe(target, {childList: true}); let records = obs.takeRecords(); typeof obs === 'object' && typeof records === 'object';",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::Bool(true));
    }

    #[test]
    fn mutation_observer_disconnect_works() {
        let result = eval_with_dom(
            "<main></main>",
            "let obs = MutationObserver(function(){}); obs.disconnect(); 'ok';",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("ok".into()));
    }

    #[test]
    fn intersection_observer_fires_callback_on_observe() {
        let result = eval_with_dom(
            "<div id='box'>Hi</div>",
            "let seen = ''; let obs = IntersectionObserver(function(entries){ seen = entries[0].isIntersecting + ':' + entries[0].intersectionRatio; }); let box = document.getElementById('box'); obs.observe(box); seen;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("true:1".into()));
    }

    #[test]
    fn intersection_observer_disconnect_and_unobserve_work() {
        let result = eval_with_dom(
            "<main></main>",
            "let obs = IntersectionObserver(function(){}); obs.disconnect(); obs.unobserve(null); let records = obs.takeRecords(); records.length;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::Number(0.0));
    }

    #[test]
    fn microtasks_animation_frames_and_timers_have_deterministic_order() {
        let result = eval_with_dom(
            "<main></main>",
            "let order=''; setTimeout(function(){ order=order+'T'; console.log(order); }, 0); requestAnimationFrame(function(){ order=order+'R'; }); queueMicrotask(function(){ order=order+'M'; }); order=order+'S'; 'sync';",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(result.console, vec!["SMRT".to_string()]);
    }

    #[test]
    fn dom_create_insert_replace_clone_and_fragments_are_detached_until_inserted() {
        let result = eval_with_dom(
            "<main id='app'><p>A</p></main>",
            "let main=document.getElementById('app'); let span=document.createElement('span'); span.textContent='X'; let before=main.children.length; main.insertBefore(span, main.children[0]); let frag=document.createDocumentFragment(); let em=document.createElement('em'); em.textContent='Y'; frag.appendChild(em); document.getElementById('app').appendChild(frag); let clone=document.querySelector('span').cloneNode(true); clone.textContent='Z'; document.getElementById('app').replaceChild(clone, document.getElementById('app').children[1]); before + ':' + document.getElementById('app').children.length + ':' + document.getElementById('app').textContent;",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("1:3:XZY".into()));
    }

    #[test]
    fn open_shadow_root_supports_append_query_and_text() {
        let result = eval_with_dom(
            "<div id='host'></div>",
            "let host=document.getElementById('host'); let root=host.attachShadow({mode:'open'}); let span=document.createElement('span'); span.setAttribute('id','inside'); span.textContent='shadow'; root.appendChild(span); let queried=host.shadowRoot.querySelector('#inside').textContent; let fresh=document.getElementById('host').shadowRoot.textContent; queried + ':' + fresh + ':' + document.querySelector('#inside');",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("shadow:shadow:null".into()));
        assert!(!result.html.contains("shadow"));
    }

    #[test]
    fn closed_shadow_root_is_not_exposed() {
        let result = eval_with_dom(
            "<div id='host'></div>",
            "let host=document.getElementById('host'); let root=host.attachShadow({mode:'closed'}); (root === null) + ':' + (host.shadowRoot === null);",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("false:true".into()));
    }

    #[test]
    fn event_capture_bubble_focus_and_radio_defaults_work() {
        let result = eval_with_dom(
            "<div id='p'><input id='a'><input id='b'><button id='c'>Go</button></div>",
            "let p=document.getElementById('p'); let c=document.getElementById('c'); let order=''; p.addEventListener('click', function(e){ order=order+'C'+e.eventPhase; }, {capture:true}); p.addEventListener('click', function(e){ order=order+'B'+e.eventPhase; }); c.addEventListener('click', function(e){ order=order+'T'+e.eventPhase; }); c.click(); let focus=''; document.getElementById('a').addEventListener('focus', function(){ focus=focus+'aF'; }); document.getElementById('a').addEventListener('blur', function(){ focus=focus+'aB'; }); document.getElementById('b').addEventListener('focus', function(){ focus=focus+'bF'; }); document.getElementById('a').focus(); document.getElementById('b').focus(); order + ':' + focus;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("C1T2B3:aFaBbF".into()));

        let result = eval_with_dom(
            "<form><input id='r1' type='radio' name='pick' checked><input id='r2' type='radio' name='pick'></form>",
            "let changes=0; document.getElementById('r2').addEventListener('change', function(){ changes=changes+1; }); document.getElementById('r2').click(); document.getElementById('r1').checked + ':' + document.getElementById('r2').checked + ':' + changes;",
        )
        .unwrap();
        assert_eq!(result.value, JsValue::String("false:true:1".into()));
    }

    #[test]
    fn dom_traversal_and_full_selectors_work() {
        let result = eval_with_dom(
            "<main id='app'><p id='a'>A</p><p id='b' class='lead' data-kind='x'>B</p><p id='c'>C</p></main>",
            "let b=document.querySelector('#app > p.lead[data-kind=\"x\"]'); let app=document.getElementById('app'); b.parentNode.id + ':' + b.previousSibling.textContent + ':' + b.nextSibling.id + ':' + app.firstChild.id + ':' + app.lastChild.id + ':' + app.childNodes.length + ':' + app.contains(b) + ':' + b.ownerDocument.nodeType;",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("app:A:c:a:c:3:true:9".into()));
    }

    #[test]
    fn input_selection_type_text_and_document_cookie_work() {
        let result = eval_with_dom(
            "<input id='q' value='abcd'>",
            "document.cookie='sid=abc; Path=/'; document.cookie='theme=dark'; let q=document.getElementById('q'); q.setSelectionRange(1,3); q.typeText('XX'); let fresh=document.getElementById('q'); fresh.value + ':' + fresh.selectionStart + ':' + fresh.selectionEnd + ':' + document.cookie;",
        )
        .unwrap();

        assert_eq!(
            result.value,
            JsValue::String("aXXd:3:3:sid=abc; theme=dark".into())
        );
    }

    #[test]
    fn xhr_and_resize_observer_are_available() {
        let result = eval_with_dom(
            "<style>#box { width: 12px; height: 5px }</style><div id='box'></div>",
            "let xhr=XMLHttpRequest(); xhr.onreadystatechange=function(){ if (xhr.readyState == 4) { console.log('rs:' + xhr.status); } }; xhr.onload=function(){ console.log('load:' + xhr.responseURL + ':' + xhr.getResponseHeader('content-type')); }; xhr.open('post','/api/x'); xhr.setRequestHeader('X-Test','yes'); xhr.send('body'); let box=document.getElementById('box'); let ro=ResizeObserver(function(entries){ console.log('resize:' + entries[0].contentRect.width + ':' + entries[0].borderBoxSize[0].inlineSize); }); ro.observe(box); 'sync';",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(
            result.console,
            vec![
                "resize:12:12".to_string(),
                "rs:200".to_string(),
                "load:http://localhost/api/x:application/json".to_string(),
            ]
        );
    }

    #[test]
    fn websocket_open_close_callbacks_are_deterministic() {
        let result = eval_with_dom(
            "<main><p id='out'></p></main>",
            "let out=document.getElementById('out'); let ws=WebSocket('ws://agent'); ws.onopen=function(){ out.textContent='open:' + ws.readyState + ':' + ws.url; ws.close(); }; ws.onclose=function(){ out.textContent=out.textContent + ':close:' + ws.readyState; }; 'sync';",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(
            browser::text_content(&result.document.children[0]),
            "open:1:ws://agent:close:3"
        );
    }

    #[test]
    fn event_source_message_can_be_injected_through_runtime() {
        let mut runtime =
            BrowserJsRuntime::new("<main><p id='out'></p></main>", BrowserJsState::default())
                .unwrap();
        runtime
            .eval("let es=EventSource('/events'); es.onmessage=function(e){ document.getElementById('out').textContent=e.data; };")
            .unwrap();
        let id = runtime.realtime_connections()[0].id;
        let result = runtime.inject_event_source_message(id, "hello").unwrap();

        assert!(result.html.contains("<p id=\"out\">hello</p>"));
    }

    #[test]
    fn mutation_observer_records_are_delivered_after_script() {
        let result = eval_with_dom(
            "<div id='target'></div>",
            "let target=document.getElementById('target'); let obs=MutationObserver(function(records){ console.log(records.length + ':' + records[0].type); }); obs.observe(target, {childList:true}); target.appendChild(document.createElement('span')); 'sync';",
        )
        .unwrap();

        assert_eq!(result.value, JsValue::String("sync".into()));
        assert_eq!(result.console, vec!["1:childList".to_string()]);
    }

    #[test]
    fn url_abort_and_base64_compatibility_apis_work() {
        let result = eval_with_dom(
            "<main></main>",
            "let u=URL('/p?q=one#h', 'https://example.test/base/page'); let params=URLSearchParams('a=1&b=two'); params.set('a','3'); let ac=AbortController(); ac.abort('done'); u.origin + ':' + u.pathname + ':' + u.searchParams.get('q') + ':' + params.get('a') + ':' + ac.signal.aborted + ':' + ac.signal.reason + ':' + atob(btoa('hi'));",
        )
        .unwrap();

        assert_eq!(
            result.value,
            JsValue::String("https://example.test:/p:one:3:true:done:hi".into())
        );
    }

    #[test]
    fn compatibility_report_lists_storage_apis() {
        let report = compatibility_report_to_value(&[]).unwrap();
        let Value::List(items) = report else {
            panic!("expected list");
        };
        let features = items
            .borrow()
            .iter()
            .map(|v| match v {
                Value::Str(s) => s.to_string(),
                other => other.to_string(),
            })
            .collect::<Vec<_>>();
        assert!(features.contains(&"localStorage".to_string()));
        assert!(features.contains(&"sessionStorage".to_string()));
        assert!(features.contains(&"Storage.length".to_string()));
    }
}

#[cfg(test)]
#[path = "browser_js_canvas_tests.rs"]
mod canvas_tests;

#[cfg(test)]
#[path = "browser_js_observer_tests.rs"]
mod observer_tests;

#[cfg(test)]
#[path = "browser_js_channels_tests.rs"]
mod channels_tests;

#[cfg(test)]
#[path = "browser_js_dom_tests.rs"]
mod dom_compat_tests;

#[cfg(test)]
#[path = "browser_js_cssom_tests/mod.rs"]
mod cssom_tests;

#[cfg(test)]
#[path = "browser_js_performance_tests.rs"]
mod performance_tests;
