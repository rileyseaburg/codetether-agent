use tetherscript::browser::{self, DisplayCommand, Rgba};
use tetherscript::browser_agent::{BrowserPage, ComputedStyle, Locator};

#[test]
fn react_style_default_module_render_mutates_root() {
    let html = "<div id='root'></div><script type='module' src='/src/main.jsx'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);
    page.register_script_resource(
        "/src/main.jsx",
        "import React from './react.js';\nimport { createRoot } from './react-dom-client.js';\nfunction App(){ return React.createElement('h1', {id:'title'}, 'Hello React'); }\ncreateRoot(document.getElementById('root')).render(React.createElement(App));",
    );
    page.register_script_resource(
        "/src/react.js",
        "function createElement(type, props, child){ return {type:type, props:props || {}, child:child}; }\nexport default { createElement: createElement };",
    );
    page.register_script_resource(
        "/src/react-dom-client.js",
        "export function createRoot(root){ return {render:function(v){ if(typeof v.type === 'function'){ v=v.type(); } let el=document.createElement(v.type); if(v.props && v.props.id){ el.setAttribute('id', v.props.id); } el.textContent=v.child; root.appendChild(el); }}; }",
    );

    page.run_scripts().unwrap();

    assert!(page
        .session
        .html
        .contains("<h1 id=\"title\">Hello React</h1>"));
    let layout = browser::layout_document(&page.session.document, &page.session.css, 160);
    let display = browser::build_display_list(&layout);
    assert!(display.iter().any(|command| matches!(
        command,
        DisplayCommand::Text { text, .. } if text == "Hello React"
    )));
    let image = page.render_raster().unwrap();
    assert!(image.pixels.chunks_exact(4).any(|pixel| pixel != white()));
}

#[test]
fn react_render_exports_computed_layout_evidence() {
    let html = "<style>#title{color:green;width:120px;height:20px}</style>\
        <div id='root'></div><script type='module' src='/src/main.jsx'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);
    page.register_script_resource(
        "/src/main.jsx",
        "import React from './react.js';\
         import { createRoot } from './react-dom-client.js';\
         function App(){ return React.createElement('h1', {id:'title'}, 'Checkout Ready'); }\
         createRoot(document.getElementById('root')).render(React.createElement(App));",
    );
    page.register_script_resource(
        "/src/react.js",
        "function createElement(type, props, child){ return {type:type, props:props || {}, child:child}; }\n\
         export default { createElement: createElement };",
    );
    page.register_script_resource(
        "/src/react-dom-client.js",
        "export function createRoot(root){ return {render:function(v){\
         if(typeof v.type === 'function'){ v=v.type(); }\
         let el=document.createElement(v.type);\
         if(v.props && v.props.id){ el.setAttribute('id', v.props.id); }\
         el.textContent=v.child; root.appendChild(el); }}; }",
    );

    page.run_scripts().unwrap();

    let style: ComputedStyle = page.computed_style(&Locator::css("#title")).unwrap();
    assert_eq!(style.get("color"), Some("green"));
    let report = page.production_debug_report();
    let title = report
        .visual_elements
        .iter()
        .find(|item| item.selector_candidates.contains(&"#title".into()))
        .unwrap();
    assert_eq!((title.bounds.width, title.bounds.height), (120, 20));
    assert!(title.visible);
    assert_eq!(title.computed_styles.get("color").unwrap(), "green");
}

fn white() -> &'static [u8] {
    &[Rgba::WHITE.r, Rgba::WHITE.g, Rgba::WHITE.b, Rgba::WHITE.a]
}
