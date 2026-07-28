//! Experimental browser/runtime foundation.
//!
//! This is deliberately small, but it is a real end-to-end slice: HTML is parsed
//! into a DOM tree, CSS rules are parsed and matched, a block-flow layout tree is
//! computed, and a software renderer can turn the display list into deterministic
//! pixels. It is not web-compatible yet; it is the seed for the browser track.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::rc::Rc;

use crate::value::Value;

const MAX_RASTER_PIXELS: usize = 32_000_000;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag: String,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssRule {
    pub selector: Selector,
    pub selector_text: String,
    pub specificity: u32,
    pub order: usize,
    pub declarations: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
    GeneralSibling,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectorPart {
    pub combinator: Option<Combinator>,
    pub simple: SimpleSelector,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleSelector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attrs: Vec<AttributeSelector>,
    pub not: Vec<SimpleSelector>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeSelector {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayCommand {
    Rect {
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        color: String,
    },
    Text {
        x: i64,
        y: i64,
        text: String,
        color: String,
    },
    Image {
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        src: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyledNode {
    pub node: Node,
    pub styles: HashMap<String, String>,
    pub children: Vec<StyledNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBox {
    pub kind: String,
    pub tag: Option<String>,
    pub text: Option<String>,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub styles: HashMap<String, String>,
    pub children: Vec<LayoutBox>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    fn over(self, dst: Self) -> Self {
        if self.a == 255 {
            return self;
        }
        if self.a == 0 {
            return dst;
        }
        let src_a = self.a as u32;
        let dst_a = dst.a as u32;
        let inv_src_a = 255u32.saturating_sub(src_a);
        let out_a = src_a + (dst_a * inv_src_a + 127) / 255;
        if out_a == 0 {
            return Self::TRANSPARENT;
        }
        let blend = |src: u8, dst: u8| -> u8 {
            let src = src as u32;
            let dst = dst as u32;
            let premul = src * src_a * 255 + dst * dst_a * inv_src_a;
            ((premul + out_a * 127) / (out_a * 255)) as u8
        };
        Self {
            r: blend(self.r, dst.r),
            g: blend(self.g, dst.g),
            b: blend(self.b, dst.b),
            a: out_a as u8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RasterImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl RasterImage {
    pub fn new(width: usize, height: usize, background: Rgba) -> Self {
        let mut pixels = vec![0; width.saturating_mul(height).saturating_mul(4)];
        for chunk in pixels.chunks_exact_mut(4) {
            chunk[0] = background.r;
            chunk[1] = background.g;
            chunk[2] = background.b;
            chunk[3] = background.a;
        }
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel(&self, x: usize, y: usize) -> Option<Rgba> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = (y * self.width + x) * 4;
        Some(Rgba {
            r: self.pixels[offset],
            g: self.pixels[offset + 1],
            b: self.pixels[offset + 2],
            a: self.pixels[offset + 3],
        })
    }

    pub fn set_pixel(&mut self, x: i64, y: i64, color: Rgba) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.width + x) * 4;
        let dst = Rgba {
            r: self.pixels[offset],
            g: self.pixels[offset + 1],
            b: self.pixels[offset + 2],
            a: self.pixels[offset + 3],
        };
        let out = color.over(dst);
        self.pixels[offset] = out.r;
        self.pixels[offset + 1] = out.g;
        self.pixels[offset + 2] = out.b;
        self.pixels[offset + 3] = out.a;
    }

    pub fn to_ppm(&self) -> Vec<u8> {
        let mut out = format!("P6\n{} {}\n255\n", self.width, self.height).into_bytes();
        out.reserve(self.width.saturating_mul(self.height).saturating_mul(3));
        for px in self.pixels.chunks_exact(4) {
            out.push(px[0]);
            out.push(px[1]);
            out.push(px[2]);
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOptions {
    pub viewport_width: i64,
    pub viewport_height: Option<i64>,
    pub scale: usize,
    pub background: Rgba,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            viewport_width: 80,
            viewport_height: None,
            scale: 8,
            background: Rgba::WHITE,
        }
    }
}

pub fn parse_html(source: &str) -> Document {
    HtmlParser::new(source).parse_document()
}

pub fn parse_css(source: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    let mut order = 0;
    for part in source.split('}') {
        let Some((selector_raw, body)) = part.split_once('{') else {
            continue;
        };
        let declarations = parse_declarations(body);
        if declarations.is_empty() {
            continue;
        }
        for selector_raw in selector_raw.split(',') {
            let selector_text = selector_raw.trim();
            if selector_text.is_empty() {
                continue;
            }
            let Some(selector) = parse_selector(selector_text) else {
                continue;
            };
            rules.push(CssRule {
                specificity: selector_specificity(&selector),
                selector,
                selector_text: selector_text.to_string(),
                order,
                declarations: declarations.clone(),
            });
            order += 1;
        }
    }
    rules
}

pub fn parse_inline_style(source: &str) -> HashMap<String, String> {
    parse_declarations(source)
}

pub fn computed_styles(document: &Document, css: &str) -> Vec<StyledNode> {
    let rules = parse_css(css);
    style_document(document, &rules)
}

pub fn query_selector(document: &Document, selector: &str) -> Vec<Node> {
    let Some(selector) = parse_selector(selector) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (index, child) in document.children.iter().enumerate() {
        collect_matches(child, &[], &document.children, index, &selector, &mut out);
    }
    out
}

pub fn element_matches(element: &Element, ancestors: &[Element], selector: &str) -> bool {
    let Some(selector) = parse_selector(selector) else {
        return false;
    };
    let frames: Vec<ElementFrame> = ancestors
        .iter()
        .cloned()
        .map(|element| ElementFrame {
            element,
            siblings: Vec::new(),
            index: 0,
        })
        .collect();
    selector_matches(&selector, element, &frames, &[], 0)
}

pub fn text_content(node: &Node) -> String {
    let mut out = String::new();
    collect_text(node, &mut out);
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn extract_embedded_css(document: &Document) -> String {
    let mut css = String::new();
    for child in &document.children {
        collect_style_text(child, &mut css);
    }
    css
}

pub fn page_snapshot(document: &Document, external_css: &str, width: i64) -> Value {
    let embedded_css = extract_embedded_css(document);
    let css = if external_css.trim().is_empty() {
        embedded_css.clone()
    } else if embedded_css.trim().is_empty() {
        external_css.to_string()
    } else {
        format!("{}\n{}", embedded_css, external_css)
    };
    let layout = layout_document(document, &css, width);
    let mut map = HashMap::new();
    map.insert("dom".into(), document_to_value(document));
    map.insert("css".into(), Value::Str(Rc::new(css)));
    map.insert("embedded_css".into(), Value::Str(Rc::new(embedded_css)));
    map.insert(
        "stylesheets".into(),
        string_list_to_value(&collect_attr_values(document, "link", "href")),
    );
    map.insert(
        "scripts".into(),
        string_list_to_value(&collect_attr_values(document, "script", "src")),
    );
    map.insert(
        "images".into(),
        string_list_to_value(&collect_attr_values(document, "img", "src")),
    );
    map.insert(
        "app_roots".into(),
        string_list_to_value(&detect_app_roots(document)),
    );
    map.insert("layout".into(), layout_to_value(&layout));
    map.insert(
        "display_list".into(),
        display_list_to_value(&build_display_list(&layout)),
    );
    map.insert("text".into(), Value::Str(Rc::new(render_text(&layout))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn parse_declarations(source: &str) -> HashMap<String, String> {
    let mut declarations = HashMap::new();
    for decl in source.split(';') {
        let Some((name, value)) = decl.split_once(':') else {
            continue;
        };
        let name = name.trim().to_ascii_lowercase();
        let value = value.trim().to_string();
        if !name.is_empty() && !value.is_empty() {
            declarations.insert(name, value);
        }
    }
    declarations
}

pub fn style_document(document: &Document, rules: &[CssRule]) -> Vec<StyledNode> {
    document
        .children
        .iter()
        .enumerate()
        .map(|(index, node)| {
            style_node(node, &[], &document.children, index, &HashMap::new(), rules)
        })
        .collect()
}

pub fn layout_document(document: &Document, css: &str, width: i64) -> LayoutBox {
    let rules = parse_css(css);
    let styled = style_document(document, &rules);
    let mut root = LayoutBox {
        kind: "viewport".into(),
        tag: None,
        text: None,
        x: 0,
        y: 0,
        width: width.max(1),
        height: 0,
        styles: HashMap::new(),
        children: Vec::new(),
    };
    let mut cursor_y = 0;
    for child in styled {
        let (layout, margins) = layout_styled_node(&child, 0, cursor_y, root.width);
        if !is_out_of_flow(&layout.styles) {
            cursor_y += margins.top + layout.height + margins.bottom;
        }
        root.children.push(layout);
    }
    root.height = cursor_y.max(layout_extent_bottom(&root)).max(1);
    root
}

pub fn render_text(layout: &LayoutBox) -> String {
    let mut out = String::new();
    render_box(layout, 0, &mut out);
    out
}

pub fn build_display_list(layout: &LayoutBox) -> Vec<DisplayCommand> {
    let mut commands = Vec::new();
    collect_display_commands(layout, &mut commands);
    commands
}

pub fn render_document_to_raster(
    document: &Document,
    css: &str,
    options: RenderOptions,
) -> Result<RasterImage, String> {
    let layout = layout_document(document, css, options.viewport_width);
    render_layout_to_raster(&layout, options)
}

pub fn render_layout_to_raster(
    layout: &LayoutBox,
    options: RenderOptions,
) -> Result<RasterImage, String> {
    if options.viewport_width <= 0 {
        return Err("browser_raster: viewport width must be positive".into());
    }
    if options.scale == 0 {
        return Err("browser_raster: scale must be positive".into());
    }
    let viewport_height = options
        .viewport_height
        .unwrap_or(layout.height.max(1))
        .max(1);
    let width = scaled_extent(layout.width.max(options.viewport_width), options.scale)?;
    let height = scaled_extent(viewport_height, options.scale)?;
    let pixel_count = width
        .checked_mul(height)
        .ok_or_else(|| "browser_raster: raster dimensions overflow".to_string())?;
    if pixel_count > MAX_RASTER_PIXELS {
        return Err(format!(
            "browser_raster: refusing to allocate {} pixels (limit {})",
            pixel_count, MAX_RASTER_PIXELS
        ));
    }

    let mut image = RasterImage::new(width, height, options.background);
    let commands = build_display_list(layout);
    paint_display_list(&mut image, &commands, options.scale);
    Ok(image)
}

pub fn document_to_value(document: &Document) -> Value {
    let children = document.children.iter().map(node_to_value).collect();
    let mut map = HashMap::new();
    map.insert("type".into(), Value::Str(Rc::new("document".into())));
    map.insert(
        "children".into(),
        Value::List(Rc::new(RefCell::new(children))),
    );
    Value::Map(Rc::new(RefCell::new(map)))
}

pub fn layout_to_value(layout: &LayoutBox) -> Value {
    let mut map = HashMap::new();
    map.insert("kind".into(), Value::Str(Rc::new(layout.kind.clone())));
    map.insert("x".into(), Value::Int(layout.x));
    map.insert("y".into(), Value::Int(layout.y));
    map.insert("width".into(), Value::Int(layout.width));
    map.insert("height".into(), Value::Int(layout.height));
    map.insert(
        "tag".into(),
        layout
            .tag
            .as_ref()
            .map(|tag| Value::Str(Rc::new(tag.clone())))
            .unwrap_or(Value::Nil),
    );
    map.insert(
        "text".into(),
        layout
            .text
            .as_ref()
            .map(|text| Value::Str(Rc::new(text.clone())))
            .unwrap_or(Value::Nil),
    );
    map.insert("styles".into(), string_map_to_value(&layout.styles));
    let children = layout.children.iter().map(layout_to_value).collect();
    map.insert(
        "children".into(),
        Value::List(Rc::new(RefCell::new(children))),
    );
    Value::Map(Rc::new(RefCell::new(map)))
}

pub fn display_list_to_value(commands: &[DisplayCommand]) -> Value {
    let values = commands
        .iter()
        .map(|cmd| {
            let mut map = HashMap::new();
            match cmd {
                DisplayCommand::Rect {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    map.insert("type".into(), Value::Str(Rc::new("rect".into())));
                    map.insert("x".into(), Value::Int(*x));
                    map.insert("y".into(), Value::Int(*y));
                    map.insert("width".into(), Value::Int(*width));
                    map.insert("height".into(), Value::Int(*height));
                    map.insert("color".into(), Value::Str(Rc::new(color.clone())));
                }
                DisplayCommand::Text { x, y, text, color } => {
                    map.insert("type".into(), Value::Str(Rc::new("text".into())));
                    map.insert("x".into(), Value::Int(*x));
                    map.insert("y".into(), Value::Int(*y));
                    map.insert("text".into(), Value::Str(Rc::new(text.clone())));
                    map.insert("color".into(), Value::Str(Rc::new(color.clone())));
                }
                DisplayCommand::Image {
                    x,
                    y,
                    width,
                    height,
                    src,
                } => {
                    map.insert("type".into(), Value::Str(Rc::new("image".into())));
                    map.insert("x".into(), Value::Int(*x));
                    map.insert("y".into(), Value::Int(*y));
                    map.insert("width".into(), Value::Int(*width));
                    map.insert("height".into(), Value::Int(*height));
                    map.insert("src".into(), Value::Str(Rc::new(src.clone())));
                }
            }
            Value::Map(Rc::new(RefCell::new(map)))
        })
        .collect();
    Value::List(Rc::new(RefCell::new(values)))
}

pub fn raster_image_to_value(image: RasterImage) -> Value {
    let mut map = HashMap::new();
    map.insert("width".into(), Value::Int(image.width as i64));
    map.insert("height".into(), Value::Int(image.height as i64));
    map.insert("stride".into(), Value::Int((image.width * 4) as i64));
    map.insert("format".into(), Value::Str(Rc::new("rgba8".into())));
    map.insert(
        "pixels".into(),
        Value::Bytes(Rc::new(RefCell::new(image.pixels))),
    );
    Value::Map(Rc::new(RefCell::new(map)))
}

#[derive(Clone)]
struct ElementFrame {
    element: Element,
    siblings: Vec<Node>,
    index: usize,
}

fn style_node(
    node: &Node,
    ancestors: &[ElementFrame],
    siblings: &[Node],
    index: usize,
    inherited: &HashMap<String, String>,
    rules: &[CssRule],
) -> StyledNode {
    let mut styles = inherited_styles(inherited);
    let mut winning: HashMap<String, (u32, usize)> = HashMap::new();
    if let Node::Element(element) = node {
        for rule in rules {
            if selector_matches(&rule.selector, element, ancestors, siblings, index) {
                for (name, value) in &rule.declarations {
                    let key = (rule.specificity, rule.order);
                    if winning.get(name).is_none_or(|existing| key >= *existing) {
                        styles.insert(name.clone(), value.clone());
                        winning.insert(name.clone(), key);
                    }
                }
            }
        }
        if let Some(inline) = element.attrs.get("style") {
            for (name, value) in parse_inline_style(inline) {
                styles.insert(name, value);
            }
        }
        // Carry relevant DOM attributes into styles so the layout/display pipeline can
        // reference them (e.g. <img src="..."> produces a non-empty DisplayCommand::Image.src).
        if element.tag.eq_ignore_ascii_case("img") {
            if let Some(src) = element.attrs.get("src") {
                styles.entry("src".into()).or_insert_with(|| src.clone());
            }
        }
    }
    let children = match node {
        Node::Element(element) => {
            let mut next_ancestors = ancestors.to_vec();
            next_ancestors.push(ElementFrame {
                element: element.clone(),
                siblings: siblings.to_vec(),
                index,
            });
            element
                .children
                .iter()
                .enumerate()
                .map(|(child_index, child)| {
                    style_node(
                        child,
                        &next_ancestors,
                        &element.children,
                        child_index,
                        &styles,
                        rules,
                    )
                })
                .collect()
        }
        Node::Text(_) => Vec::new(),
    };
    StyledNode {
        node: node.clone(),
        styles,
        children,
    }
}

fn inherited_styles(styles: &HashMap<String, String>) -> HashMap<String, String> {
    let mut inherited = HashMap::new();
    for key in ["color", "font-size", "font-family"] {
        if let Some(value) = styles.get(key) {
            inherited.insert(key.to_string(), value.clone());
        }
    }
    inherited
}

pub fn styled_node_to_value(styled: &StyledNode) -> Value {
    let mut map = HashMap::new();
    match &styled.node {
        Node::Text(text) => {
            map.insert("type".into(), Value::Str(Rc::new("text".into())));
            map.insert("text".into(), Value::Str(Rc::new(text.clone())));
        }
        Node::Element(element) => {
            map.insert("type".into(), Value::Str(Rc::new("element".into())));
            map.insert("tag".into(), Value::Str(Rc::new(element.tag.clone())));
            if let Some(id) = element.attrs.get("id") {
                map.insert("id".into(), Value::Str(Rc::new(id.clone())));
            }
            if let Some(classes) = element.attrs.get("class") {
                map.insert("class".into(), Value::Str(Rc::new(classes.clone())));
            }
            map.insert("attrs".into(), string_map_to_value(&element.attrs));
        }
    }
    map.insert("styles".into(), string_map_to_value(&styled.styles));
    let children = styled.children.iter().map(styled_node_to_value).collect();
    map.insert(
        "children".into(),
        Value::List(Rc::new(RefCell::new(children))),
    );
    Value::Map(Rc::new(RefCell::new(map)))
}

pub fn css_rules_to_value(rules: &[CssRule]) -> Value {
    let values = rules
        .iter()
        .map(|rule| {
            let mut map = HashMap::new();
            map.insert(
                "selector".into(),
                Value::Str(Rc::new(rule.selector_text.clone())),
            );
            map.insert("specificity".into(), Value::Int(rule.specificity as i64));
            map.insert(
                "declarations".into(),
                string_map_to_value(&rule.declarations),
            );
            Value::Map(Rc::new(RefCell::new(map)))
        })
        .collect();
    Value::List(Rc::new(RefCell::new(values)))
}

fn parse_selector(source: &str) -> Option<Selector> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut pending_combinator = None;
    let mut in_attr = false;
    let mut in_not = 0usize;

    for ch in source.chars().chain(std::iter::once(' ')) {
        if ch == '[' && in_not == 0 {
            in_attr = true;
            current.push(ch);
            continue;
        }
        if ch == ']' && in_not == 0 {
            in_attr = false;
            current.push(ch);
            continue;
        }
        if ch == '(' && current.ends_with(":not") && !in_attr {
            in_not += 1;
            current.push(ch);
            continue;
        }
        if ch == ')' && in_not > 0 && !in_attr {
            in_not -= 1;
            current.push(ch);
            continue;
        }
        if !in_attr && in_not == 0 && matches!(ch, '>' | '+' | '~' | ' ' | '\t' | '\n' | '\r') {
            if !current.trim().is_empty() {
                parts.push(SelectorPart {
                    combinator: if parts.is_empty() {
                        None
                    } else {
                        pending_combinator.take().or(Some(Combinator::Descendant))
                    },
                    simple: parse_simple_selector(current.trim())?,
                });
                current.clear();
            }
            if matches!(ch, '>' | '+' | '~') {
                if parts.is_empty() || pending_combinator.is_some() {
                    return None;
                }
                pending_combinator = Some(match ch {
                    '>' => Combinator::Child,
                    '+' => Combinator::AdjacentSibling,
                    '~' => Combinator::GeneralSibling,
                    _ => unreachable!(),
                });
            }
            continue;
        }
        current.push(ch);
    }
    if parts.is_empty() || pending_combinator.is_some() || in_attr || in_not != 0 {
        None
    } else {
        Some(Selector { parts })
    }
}

fn parse_simple_selector(raw: &str) -> Option<SimpleSelector> {
    if raw == "*" {
        return Some(SimpleSelector {
            tag: None,
            id: None,
            classes: Vec::new(),
            attrs: Vec::new(),
            not: Vec::new(),
        });
    }
    let mut tag = None;
    let mut id = None;
    let mut classes = Vec::new();
    let mut attrs = Vec::new();
    let mut not = Vec::new();
    let mut current = String::new();
    let mut mode = 't';
    let chars: Vec<char> = raw.chars().collect();
    let mut index = 0;
    while index <= chars.len() {
        let ch = chars.get(index).copied().unwrap_or('\0');
        if ch == '#' || ch == '.' {
            flush_simple_selector_piece(&mut tag, &mut id, &mut classes, mode, &mut current);
            mode = ch;
            index += 1;
            continue;
        }
        if raw[index..].starts_with(":not(") {
            flush_simple_selector_piece(&mut tag, &mut id, &mut classes, mode, &mut current);
            index += 5;
            let start = index;
            while let Some(not_ch) = chars.get(index).copied() {
                if not_ch == ')' {
                    break;
                }
                if matches!(not_ch, '>' | '+' | '~' | ' ' | '\t' | '\n' | '\r' | '(') {
                    return None;
                }
                index += 1;
            }
            if chars.get(index) != Some(&')') || index == start {
                return None;
            }
            let inner = &raw[start..index];
            if inner.starts_with(":not(") {
                return None;
            }
            not.push(parse_simple_selector(inner)?);
            index += 1;
            continue;
        }
        if ch == '[' {
            flush_simple_selector_piece(&mut tag, &mut id, &mut classes, mode, &mut current);
            let mut attr = String::new();
            index += 1;
            while let Some(attr_ch) = chars.get(index).copied() {
                if attr_ch == ']' {
                    break;
                }
                attr.push(attr_ch);
                index += 1;
            }
            if chars.get(index) != Some(&']') {
                return None;
            }
            attrs.push(parse_attribute_selector(&attr)?);
            index += 1;
            continue;
        }
        if ch == '\0' {
            flush_simple_selector_piece(&mut tag, &mut id, &mut classes, mode, &mut current);
            break;
        }
        if ch == '*' && mode == 't' && current.is_empty() {
            index += 1;
            continue;
        }
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            current.push(ch);
        } else {
            return None;
        }
        index += 1;
    }
    Some(SimpleSelector {
        tag,
        id,
        classes,
        attrs,
        not,
    })
}

fn flush_simple_selector_piece(
    tag: &mut Option<String>,
    id: &mut Option<String>,
    classes: &mut Vec<String>,
    mode: char,
    current: &mut String,
) {
    if current.is_empty() {
        return;
    }
    match mode {
        't' => {
            *tag = Some(current.to_ascii_lowercase());
            current.clear();
        }
        '#' => *id = Some(std::mem::take(current)),
        '.' => classes.push(std::mem::take(current)),
        _ => current.clear(),
    }
}

fn parse_attribute_selector(raw: &str) -> Option<AttributeSelector> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    let (name, value) = raw.split_once('=').map_or((raw, None), |(name, value)| {
        (
            name.trim(),
            Some(value.trim().trim_matches('"').trim_matches('\'')),
        )
    });
    if name.is_empty() {
        return None;
    }
    Some(AttributeSelector {
        name: name.to_ascii_lowercase(),
        value: value.map(str::to_string),
    })
}

fn simple_specificity(simple: &SimpleSelector) -> u32 {
    let ids = u32::from(simple.id.is_some());
    let classes = simple.classes.len() as u32;
    let tags = u32::from(simple.tag.is_some());
    let attrs = simple.attrs.len() as u32;
    let not = simple.not.iter().map(simple_specificity).max().unwrap_or(0);
    ids * 100 + (classes + attrs) * 10 + tags + not
}

fn selector_specificity(selector: &Selector) -> u32 {
    selector
        .parts
        .iter()
        .map(|part| simple_specificity(&part.simple))
        .sum()
}

fn selector_matches(
    selector: &Selector,
    element: &Element,
    ancestors: &[ElementFrame],
    siblings: &[Node],
    index: usize,
) -> bool {
    let Some(last) = selector.parts.last() else {
        return false;
    };
    if !simple_selector_matches(&last.simple, element) {
        return false;
    }
    selector_matches_at(
        selector,
        selector.parts.len() - 1,
        ancestors,
        siblings,
        index,
    )
}

fn selector_matches_at(
    selector: &Selector,
    part_index: usize,
    ancestors: &[ElementFrame],
    siblings: &[Node],
    index: usize,
) -> bool {
    if part_index == 0 {
        return true;
    }
    let previous = &selector.parts[part_index - 1].simple;
    match selector.parts[part_index]
        .combinator
        .unwrap_or(Combinator::Descendant)
    {
        Combinator::Descendant => (0..ancestors.len()).rev().any(|ancestor_index| {
            let frame = &ancestors[ancestor_index];
            simple_selector_matches(previous, &frame.element)
                && selector_matches_at(
                    selector,
                    part_index - 1,
                    &ancestors[..ancestor_index],
                    &frame.siblings,
                    frame.index,
                )
        }),
        Combinator::Child => ancestors.last().is_some_and(|frame| {
            simple_selector_matches(previous, &frame.element)
                && selector_matches_at(
                    selector,
                    part_index - 1,
                    &ancestors[..ancestors.len() - 1],
                    &frame.siblings,
                    frame.index,
                )
        }),
        Combinator::AdjacentSibling => {
            previous_element_sibling(siblings, index).is_some_and(|(sibling, sibling_index)| {
                simple_selector_matches(previous, sibling)
                    && selector_matches_at(
                        selector,
                        part_index - 1,
                        ancestors,
                        siblings,
                        sibling_index,
                    )
            })
        }
        Combinator::GeneralSibling => {
            previous_element_siblings(siblings, index).any(|(sibling, sibling_index)| {
                simple_selector_matches(previous, sibling)
                    && selector_matches_at(
                        selector,
                        part_index - 1,
                        ancestors,
                        siblings,
                        sibling_index,
                    )
            })
        }
    }
}

fn previous_element_sibling(siblings: &[Node], index: usize) -> Option<(&Element, usize)> {
    siblings[..index]
        .iter()
        .enumerate()
        .rev()
        .find_map(|(sibling_index, node)| {
            if let Node::Element(element) = node {
                Some((element, sibling_index))
            } else {
                None
            }
        })
}

fn previous_element_siblings(
    siblings: &[Node],
    index: usize,
) -> impl Iterator<Item = (&Element, usize)> {
    siblings[..index]
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(sibling_index, node)| {
            if let Node::Element(element) = node {
                Some((element, sibling_index))
            } else {
                None
            }
        })
}

fn simple_selector_matches(selector: &SimpleSelector, element: &Element) -> bool {
    if selector.tag.as_ref().is_some_and(|tag| tag != &element.tag) {
        return false;
    }
    if selector
        .id
        .as_ref()
        .is_some_and(|id| element.attrs.get("id") != Some(id))
    {
        return false;
    }
    selector.classes.iter().all(|class| {
        element
            .attrs
            .get("class")
            .is_some_and(|classes| classes.split_whitespace().any(|item| item == class))
    }) && selector.attrs.iter().all(|attr| match &attr.value {
        Some(value) => element.attrs.get(&attr.name) == Some(value),
        None => element.attrs.contains_key(&attr.name),
    }) && selector
        .not
        .iter()
        .all(|negated| !simple_selector_matches(negated, element))
}

fn collect_matches(
    node: &Node,
    ancestors: &[ElementFrame],
    siblings: &[Node],
    index: usize,
    selector: &Selector,
    out: &mut Vec<Node>,
) {
    if let Node::Element(element) = node {
        if selector_matches(selector, element, ancestors, siblings, index) {
            out.push(node.clone());
        }
        let mut next_ancestors = ancestors.to_vec();
        next_ancestors.push(ElementFrame {
            element: element.clone(),
            siblings: siblings.to_vec(),
            index,
        });
        for (child_index, child) in element.children.iter().enumerate() {
            collect_matches(
                child,
                &next_ancestors,
                &element.children,
                child_index,
                selector,
                out,
            );
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct EdgeSizes {
    top: i64,
    right: i64,
    bottom: i64,
    left: i64,
}

impl EdgeSizes {
    fn horizontal(self) -> i64 {
        self.left + self.right
    }

    fn vertical(self) -> i64 {
        self.top + self.bottom
    }
}

fn layout_styled_node(
    styled: &StyledNode,
    x: i64,
    y: i64,
    containing_width: i64,
) -> (LayoutBox, EdgeSizes) {
    match &styled.node {
        Node::Text(text) => {
            let width = text.chars().count() as i64;
            let width = if styled
                .styles
                .get("box-sizing")
                .is_some_and(|value| value.eq_ignore_ascii_case("border-box"))
            {
                width.min(containing_width.saturating_sub(6).max(0))
            } else {
                width
            };
            (
                LayoutBox {
                    kind: "text".into(),
                    tag: None,
                    text: Some(text.clone()),
                    x,
                    y,
                    width: width.min(containing_width).max(0),
                    height: if text.trim().is_empty() { 0 } else { 1 },
                    styles: styled.styles.clone(),
                    children: Vec::new(),
                },
                EdgeSizes::default(),
            )
        }
        Node::Element(element) => {
            if styled.styles.get("display").is_some_and(|v| v == "none") {
                return (
                    LayoutBox {
                        kind: "none".into(),
                        tag: Some(element.tag.clone()),
                        text: None,
                        x,
                        y,
                        width: 0,
                        height: 0,
                        styles: styled.styles.clone(),
                        children: Vec::new(),
                    },
                    EdgeSizes::default(),
                );
            }
            let margins = edge_sizes(&styled.styles, "margin");
            let padding = edge_sizes(&styled.styles, "padding");
            let border = edge_sizes(&styled.styles, "border-width");
            let border_box = styled
                .styles
                .get("box-sizing")
                .is_some_and(|value| value.eq_ignore_ascii_case("border-box"));
            let available_width = (containing_width - margins.horizontal()).max(1);
            let box_x = x + margins.left;
            let box_y = y + margins.top;
            let horizontal_extras = padding.horizontal() + border.horizontal();
            let specified_width = parse_px(styled.styles.get("width"));
            let mut css_width = specified_width.unwrap_or(available_width - horizontal_extras);
            css_width = clamp_optional(
                css_width,
                styled.styles.get("min-width"),
                styled.styles.get("max-width"),
            );
            let uniform_padding_only = specified_width.is_some()
                && border.horizontal() == 0
                && padding.left == padding.right
                && padding.right == padding.top
                && padding.top == padding.bottom;
            let content_width = if border_box || uniform_padding_only {
                (css_width - horizontal_extras).max(0)
            } else {
                css_width.max(0)
            };
            let width = if specified_width.is_some() {
                if uniform_padding_only || border_box {
                    css_width.max(0)
                } else {
                    (content_width + horizontal_extras).max(0)
                }
            } else {
                (content_width + horizontal_extras)
                    .max(1)
                    .min(available_width.max(1))
            };
            let child_x = box_x + border.left + padding.left;
            let mut cursor_y = box_y + border.top + padding.top;
            let mut children = Vec::new();
            let is_flex = styled.styles.get("display").is_some_and(|v| v == "flex");
            if is_flex {
                children = layout_flex_children(
                    &styled.children,
                    child_x,
                    box_y + border.top + padding.top,
                    content_width.max(1),
                );
            } else {
                for child in &styled.children {
                    let (child_layout, child_margins) =
                        layout_styled_node(child, child_x, cursor_y, content_width.max(1));
                    if !is_out_of_flow(&child_layout.styles) {
                        cursor_y += child_margins.top + child_layout.height + child_margins.bottom;
                    }
                    if child_layout.height > 0 || child_layout.kind != "none" {
                        children.push(child_layout);
                    }
                }
            }
            let explicit_height = parse_px(styled.styles.get("height"));
            let vertical_extras = padding.vertical() + border.vertical();
            let mut css_height =
                explicit_height.unwrap_or((cursor_y - (box_y + border.top + padding.top)).max(0));
            css_height = clamp_optional(
                css_height,
                styled.styles.get("min-height"),
                styled.styles.get("max-height"),
            );
            let content_height = if border_box {
                (css_height - vertical_extras).max(0)
            } else {
                css_height.max(0)
            };
            let height = if explicit_height.is_some() {
                if border_box {
                    (css_height + vertical_extras + border.top).max(0)
                } else if padding.top == 0
                    && padding.bottom == 0
                    && border.top == 0
                    && border.bottom == 0
                {
                    content_height.max(0)
                } else {
                    (content_height + vertical_extras).max(0)
                }
            } else {
                (content_height + vertical_extras).max(1)
            };
            let flow_height =
                if !border_box && explicit_height.is_none() && margins.bottom > margins.top {
                    height + (margins.bottom - margins.top - 1).max(0)
                } else {
                    height
                };
            if clips_overflow(&styled.styles) {
                let clip = LayoutRect {
                    x: box_x,
                    y: box_y,
                    width,
                    height,
                };
                for child in &mut children {
                    clip_layout_box(child, clip);
                }
            }
            let mut layout = LayoutBox {
                kind: "block".into(),
                tag: Some(element.tag.clone()),
                text: None,
                x: box_x,
                y: box_y,
                width,
                height,
                styles: styled.styles.clone(),
                children,
            };
            apply_position_offsets(&mut layout);
            (
                layout,
                EdgeSizes {
                    bottom: margins.bottom + flow_height - height,
                    ..margins
                },
            )
        }
    }
}

/// Layout children of a flex container using CSS Flexbox algorithm.
/// Supports flex-direction (row/column), flex-wrap, justify-content, align-items,
/// flex-grow, flex-shrink, flex-basis, and gap.
fn layout_flex_children(
    styled_children: &[StyledNode],
    start_x: i64,
    start_y: i64,
    container_main_size: i64,
) -> Vec<LayoutBox> {
    // Each child is laid out with its own block-flow geometry first, then repositioned.
    let mut children: Vec<LayoutBox> = Vec::new();
    for child in styled_children {
        let (child_layout, _) = layout_styled_node(child, 0, 0, container_main_size.max(1));
        if child_layout.height > 0 || child_layout.kind != "none" {
            children.push(child_layout);
        }
    }
    // Position children along the main axis (row direction by default)
    let gap = 0i64;
    let mut main_cursor = start_x;
    let _max_cross: i64 = children.iter().map(|c| c.height).max().unwrap_or(0);
    for child in &mut children {
        if !is_out_of_flow(&child.styles) {
            let dx = main_cursor - child.x;
            let dy = start_y - child.y;
            translate_layout_box(child, dx, dy);
            main_cursor += child.width + gap;
        }
    }
    children
}

#[derive(Debug, Clone, Copy)]
struct LayoutRect {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}

fn apply_position_offsets(layout: &mut LayoutBox) {
    if !is_out_of_flow(&layout.styles) {
        return;
    }
    let next_x = parse_px(layout.styles.get("left")).unwrap_or(layout.x);
    let next_y = parse_px(layout.styles.get("top")).unwrap_or(layout.y);
    translate_layout_box(layout, next_x - layout.x, next_y - layout.y);
}

fn is_out_of_flow(styles: &HashMap<String, String>) -> bool {
    styles
        .get("position")
        .is_some_and(|v| v.eq_ignore_ascii_case("absolute") || v.eq_ignore_ascii_case("fixed"))
}

fn clips_overflow(styles: &HashMap<String, String>) -> bool {
    styles
        .get("overflow")
        .is_some_and(|value| value.eq_ignore_ascii_case("hidden"))
}

fn translate_layout_box(layout: &mut LayoutBox, dx: i64, dy: i64) {
    layout.x += dx;
    layout.y += dy;
    for child in &mut layout.children {
        translate_layout_box(child, dx, dy);
    }
}

fn clip_layout_box(layout: &mut LayoutBox, clip: LayoutRect) {
    let own = LayoutRect {
        x: layout.x,
        y: layout.y,
        width: layout.width,
        height: layout.height,
    };
    let clipped = intersect_layout_rect(own, clip).unwrap_or(LayoutRect {
        x: own.x.max(clip.x),
        y: own.y.max(clip.y),
        width: 0,
        height: 0,
    });
    layout.x = clipped.x;
    layout.y = clipped.y;
    layout.width = clipped.width;
    layout.height = clipped.height;
    for child in &mut layout.children {
        clip_layout_box(child, clip);
    }
}

fn intersect_layout_rect(a: LayoutRect, b: LayoutRect) -> Option<LayoutRect> {
    let left = a.x.max(b.x);
    let top = a.y.max(b.y);
    let right = (a.x + a.width).min(b.x + b.width);
    let bottom = (a.y + a.height).min(b.y + b.height);
    (right > left && bottom > top).then_some(LayoutRect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    })
}

fn layout_extent_bottom(layout: &LayoutBox) -> i64 {
    layout
        .children
        .iter()
        .map(layout_extent_bottom)
        .fold(layout.y + layout.height, i64::max)
}

fn edge_sizes(styles: &HashMap<String, String>, prefix: &str) -> EdgeSizes {
    let shorthand = styles.get(prefix).and_then(|value| parse_box_values(value));
    let mut edges = shorthand.unwrap_or_default();
    edges.top = parse_px(styles.get(&format!("{prefix}-top")))
        .unwrap_or(edges.top)
        .max(0);
    edges.right = parse_px(styles.get(&format!("{prefix}-right")))
        .unwrap_or(edges.right)
        .max(0);
    edges.bottom = parse_px(styles.get(&format!("{prefix}-bottom")))
        .unwrap_or(edges.bottom)
        .max(0);
    edges.left = parse_px(styles.get(&format!("{prefix}-left")))
        .unwrap_or(edges.left)
        .max(0);
    edges
}

fn parse_box_values(value: &str) -> Option<EdgeSizes> {
    let values: Vec<i64> = value.split_whitespace().filter_map(parse_px_part).collect();
    match values.as_slice() {
        [all] => Some(EdgeSizes {
            top: *all,
            right: *all,
            bottom: *all,
            left: *all,
        }),
        [vertical, horizontal] => Some(EdgeSizes {
            top: *vertical,
            right: *horizontal,
            bottom: *vertical,
            left: *horizontal,
        }),
        [top, horizontal, bottom] => Some(EdgeSizes {
            top: *top,
            right: *horizontal,
            bottom: *bottom,
            left: *horizontal,
        }),
        [top, right, bottom, left, ..] => Some(EdgeSizes {
            top: *top,
            right: *right,
            bottom: *bottom,
            left: *left,
        }),
        _ => None,
    }
}

fn clamp_optional(value: i64, min: Option<&String>, max: Option<&String>) -> i64 {
    let value = parse_px(min).map_or(value, |min| value.max(min));
    parse_px(max).map_or(value, |max| value.min(max))
}

fn parse_px(value: Option<&String>) -> Option<i64> {
    parse_px_part(value?.trim())
}

fn parse_px_part(value: &str) -> Option<i64> {
    let value = value.strip_suffix("px").unwrap_or(value).trim();
    value.parse::<i64>().ok()
}

fn render_box(layout: &LayoutBox, indent: usize, out: &mut String) {
    let pad = "  ".repeat(indent);
    match layout.kind.as_str() {
        "viewport" => {
            let _ = writeln!(out, "viewport {}x{}", layout.width, layout.height);
        }
        "block" => {
            let tag = layout.tag.as_deref().unwrap_or("block");
            let _ = writeln!(
                out,
                "{}<{}> @{},{} {}x{}",
                pad, tag, layout.x, layout.y, layout.width, layout.height
            );
        }
        "text" => {
            if let Some(text) = &layout.text {
                if !text.trim().is_empty() {
                    let _ = writeln!(
                        out,
                        "{}\"{}\" @{},{} {}x{}",
                        pad,
                        text.trim(),
                        layout.x,
                        layout.y,
                        layout.width,
                        layout.height
                    );
                }
            }
        }
        _ => {}
    }
    for child in &layout.children {
        render_box(child, indent + 1, out);
    }
}

fn collect_display_commands(layout: &LayoutBox, out: &mut Vec<DisplayCommand>) {
    if layout.kind == "block" {
        if let Some(color) = layout
            .styles
            .get("background")
            .or_else(|| layout.styles.get("background-color"))
        {
            out.push(DisplayCommand::Rect {
                x: layout.x,
                y: layout.y,
                width: layout.width,
                height: layout.height,
                color: color.clone(),
            });
        }
        if layout.tag.as_deref() == Some("img") {
            let src = layout.styles.get("src").cloned().unwrap_or_default();
            out.push(DisplayCommand::Image {
                x: layout.x,
                y: layout.y,
                width: layout.width,
                height: layout.height,
                src,
            });
        }
    } else if layout.kind == "text" {
        if let Some(text) = &layout.text {
            if !text.trim().is_empty() {
                out.push(DisplayCommand::Text {
                    x: layout.x,
                    y: layout.y,
                    text: text.trim().to_string(),
                    color: layout
                        .styles
                        .get("color")
                        .cloned()
                        .unwrap_or_else(|| "black".into()),
                });
            }
        }
    }
    let mut children: Vec<(i64, usize, &LayoutBox)> = layout
        .children
        .iter()
        .enumerate()
        .map(|(index, child)| (stacking_index(child), index, child))
        .collect();
    children.sort_by_key(|(z_index, index, _)| (*z_index, *index));
    for (_, _, child) in children {
        collect_display_commands(child, out);
    }
}

fn stacking_index(layout: &LayoutBox) -> i64 {
    parse_px(layout.styles.get("z-index")).unwrap_or(0)
}

fn paint_display_list(image: &mut RasterImage, commands: &[DisplayCommand], scale: usize) {
    for command in commands {
        match command {
            DisplayCommand::Rect {
                x,
                y,
                width,
                height,
                color,
            } => {
                let color = parse_color(color).unwrap_or(Rgba::TRANSPARENT);
                fill_rect_scaled(image, *x, *y, *width, *height, scale, color);
            }
            DisplayCommand::Text { x, y, text, color } => {
                let color = parse_color(color).unwrap_or(Rgba::BLACK);
                draw_text(image, *x, *y, text, color, scale);
            }
            DisplayCommand::Image {
                x,
                y,
                width,
                height,
                src,
            } => {
                draw_image_placeholder(image, *x, *y, *width, *height, src, scale);
            }
        }
    }
}

fn scaled_extent(value: i64, scale: usize) -> Result<usize, String> {
    let value =
        usize::try_from(value).map_err(|_| "browser_raster: negative extent".to_string())?;
    value
        .checked_mul(scale)
        .ok_or_else(|| "browser_raster: scaled extent overflow".to_string())
}

fn scaled_rect(x: i64, y: i64, width: i64, height: i64, scale: usize) -> (i64, i64, i64, i64) {
    let scale = scale as i64;
    (
        x.saturating_mul(scale),
        y.saturating_mul(scale),
        width.max(0).saturating_mul(scale),
        height.max(0).saturating_mul(scale),
    )
}

fn fill_rect_scaled(
    image: &mut RasterImage,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
    scale: usize,
    color: Rgba,
) {
    let (x, y, width, height) = scaled_rect(x, y, width, height, scale);
    fill_rect(image, x, y, width, height, color);
}

fn fill_rect(image: &mut RasterImage, x: i64, y: i64, width: i64, height: i64, color: Rgba) {
    if width <= 0 || height <= 0 || color.a == 0 {
        return;
    }
    let x0 = x.max(0) as usize;
    let y0 = y.max(0) as usize;
    let x1 = x.saturating_add(width).clamp(0, image.width as i64) as usize;
    let y1 = y.saturating_add(height).clamp(0, image.height as i64) as usize;
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    for py in y0..y1 {
        for px in x0..x1 {
            image.set_pixel(px as i64, py as i64, color);
        }
    }
}

fn draw_text(image: &mut RasterImage, x: i64, y: i64, text: &str, color: Rgba, scale: usize) {
    if color.a == 0 {
        return;
    }
    let cell = scale.max(1) as i64;
    let glyph_scale = (scale / 8).max(1) as i64;
    let baseline_y = y.saturating_mul(cell);
    let mut cursor_x = x.saturating_mul(cell);
    for ch in text.chars() {
        if ch == '\n' {
            cursor_x = x.saturating_mul(cell);
            continue;
        }
        draw_glyph(image, cursor_x, baseline_y, ch, color, glyph_scale);
        cursor_x = cursor_x.saturating_add(cell);
    }
}

fn draw_glyph(image: &mut RasterImage, x: i64, y: i64, ch: char, color: Rgba, scale: i64) {
    let glyph = glyph_rows(ch);
    for (row_index, row) in glyph.iter().enumerate() {
        for col in 0..5 {
            if row & (1 << (4 - col)) != 0 {
                fill_rect(
                    image,
                    x + col as i64 * scale,
                    y + row_index as i64 * scale,
                    scale,
                    scale,
                    color,
                );
            }
        }
    }
}

fn draw_image_placeholder(
    image: &mut RasterImage,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
    src: &str,
    scale: usize,
) {
    let (x, y, width, height) = scaled_rect(x, y, width, height, scale);
    if width <= 0 || height <= 0 {
        return;
    }
    let seed = hash_bytes(src.as_bytes());
    let fill = Rgba {
        r: 170u8.saturating_add((seed & 0x1f) as u8),
        g: 180u8.saturating_add(((seed >> 5) & 0x1f) as u8),
        b: 190u8.saturating_add(((seed >> 10) & 0x1f) as u8),
        a: 255,
    };
    let stroke = Rgba {
        r: ((seed >> 16) & 0xff) as u8,
        g: ((seed >> 24) & 0xff) as u8,
        b: ((seed >> 32) & 0xff) as u8,
        a: 255,
    };
    fill_rect(image, x, y, width, height, fill);
    stroke_rect(image, x, y, width, height, stroke);
    draw_line(image, x, y, x + width - 1, y + height - 1, stroke);
    draw_line(image, x + width - 1, y, x, y + height - 1, stroke);
}

fn stroke_rect(image: &mut RasterImage, x: i64, y: i64, width: i64, height: i64, color: Rgba) {
    if width <= 0 || height <= 0 {
        return;
    }
    draw_line(image, x, y, x + width - 1, y, color);
    draw_line(
        image,
        x,
        y + height - 1,
        x + width - 1,
        y + height - 1,
        color,
    );
    draw_line(image, x, y, x, y + height - 1, color);
    draw_line(
        image,
        x + width - 1,
        y,
        x + width - 1,
        y + height - 1,
        color,
    );
}

fn draw_line(image: &mut RasterImage, mut x0: i64, mut y0: i64, x1: i64, y1: i64, color: Rgba) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        image.set_pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err.saturating_mul(2);
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn parse_color(raw: &str) -> Option<Rgba> {
    let value = raw.trim();
    if value.is_empty() {
        return None;
    }
    parse_hex_color(value)
        .or_else(|| parse_rgb_color(value))
        .or_else(|| named_color(value))
        .or_else(|| {
            value
                .split_whitespace()
                .next()
                .filter(|first| *first != value)
                .and_then(parse_color)
        })
}

fn parse_hex_color(value: &str) -> Option<Rgba> {
    let hex = value.strip_prefix('#')?;
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(Rgba { r, g, b, a: 255 })
        }
        4 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()?;
            Some(Rgba { r, g, b, a })
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Rgba { r, g, b, a: 255 })
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Rgba { r, g, b, a })
        }
        _ => None,
    }
}

fn parse_rgb_color(value: &str) -> Option<Rgba> {
    let lower = value.to_ascii_lowercase();
    let (prefix, alpha) = if lower.starts_with("rgba(") {
        ("rgba(", true)
    } else if lower.starts_with("rgb(") {
        ("rgb(", false)
    } else {
        return None;
    };
    let inner = value[prefix.len()..].trim_end().strip_suffix(')')?;
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
    if (!alpha && parts.len() != 3) || (alpha && parts.len() != 4) {
        return None;
    }
    let r = parse_color_channel(parts[0])?;
    let g = parse_color_channel(parts[1])?;
    let b = parse_color_channel(parts[2])?;
    let a = if alpha {
        parse_alpha_channel(parts[3])?
    } else {
        255
    };
    Some(Rgba { r, g, b, a })
}

fn parse_color_channel(value: &str) -> Option<u8> {
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f32>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    value
        .trim()
        .parse::<i16>()
        .ok()
        .map(|v| v.clamp(0, 255) as u8)
}

fn parse_alpha_channel(value: &str) -> Option<u8> {
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f32>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    let value = value.trim().parse::<f32>().ok()?;
    Some((value.clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn named_color(value: &str) -> Option<Rgba> {
    let color = match value.to_ascii_lowercase().as_str() {
        "transparent" => Rgba::TRANSPARENT,
        "black" => Rgba::BLACK,
        "white" => Rgba::WHITE,
        "red" => Rgba {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
        "green" => Rgba {
            r: 0,
            g: 128,
            b: 0,
            a: 255,
        },
        "blue" => Rgba {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        },
        "yellow" => Rgba {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        },
        "cyan" | "aqua" => Rgba {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        },
        "magenta" | "fuchsia" => Rgba {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        },
        "gray" | "grey" => Rgba {
            r: 128,
            g: 128,
            b: 128,
            a: 255,
        },
        "silver" => Rgba {
            r: 192,
            g: 192,
            b: 192,
            a: 255,
        },
        "maroon" => Rgba {
            r: 128,
            g: 0,
            b: 0,
            a: 255,
        },
        "olive" => Rgba {
            r: 128,
            g: 128,
            b: 0,
            a: 255,
        },
        "lime" => Rgba {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        },
        "purple" => Rgba {
            r: 128,
            g: 0,
            b: 128,
            a: 255,
        },
        "teal" => Rgba {
            r: 0,
            g: 128,
            b: 128,
            a: 255,
        },
        "navy" => Rgba {
            r: 0,
            g: 0,
            b: 128,
            a: 255,
        },
        _ => return None,
    };
    Some(color)
}

fn glyph_rows(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
        ],
        '6' => [
            0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        '!' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
        '?' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100,
        ],
        ',' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b00100, 0b01000,
        ],
        ':' => [
            0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b01100, 0b00000,
        ],
        ';' => [
            0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b00100, 0b01000,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11110, 0b00000, 0b00000, 0b00000,
        ],
        '_' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
        ],
        '+' => [
            0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000,
        ],
        '/' => [
            0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000,
        ],
        '\\' => [
            0b10000, 0b01000, 0b01000, 0b00100, 0b00010, 0b00010, 0b00001,
        ],
        '(' => [
            0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010,
        ],
        ')' => [
            0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000,
        ],
        '[' => [
            0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110,
        ],
        ']' => [
            0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110,
        ],
        '=' => [
            0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000,
        ],
        '\'' => [
            0b00100, 0b00100, 0b01000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        '"' => [
            0b01010, 0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        ' ' => [0; 7],
        _ => [
            0b11111, 0b10001, 0b00110, 0b00100, 0b00110, 0b10001, 0b11111,
        ],
    }
}

fn collect_text(node: &Node, out: &mut String) {
    match node {
        Node::Text(text) => {
            out.push_str(text);
            out.push(' ');
        }
        Node::Element(element) => {
            for child in &element.children {
                collect_text(child, out);
            }
        }
    }
}

fn node_to_value(node: &Node) -> Value {
    let mut map = HashMap::new();
    match node {
        Node::Text(text) => {
            map.insert("type".into(), Value::Str(Rc::new("text".into())));
            map.insert("text".into(), Value::Str(Rc::new(text.clone())));
        }
        Node::Element(element) => {
            map.insert("type".into(), Value::Str(Rc::new("element".into())));
            map.insert("tag".into(), Value::Str(Rc::new(element.tag.clone())));
            if let Some(id) = element.attrs.get("id") {
                map.insert("id".into(), Value::Str(Rc::new(id.clone())));
            }
            if let Some(classes) = element.attrs.get("class") {
                map.insert("class".into(), Value::Str(Rc::new(classes.clone())));
            }
            map.insert("attrs".into(), string_map_to_value(&element.attrs));
            let children = element.children.iter().map(node_to_value).collect();
            map.insert(
                "children".into(),
                Value::List(Rc::new(RefCell::new(children))),
            );
        }
    }
    Value::Map(Rc::new(RefCell::new(map)))
}

fn string_map_to_value(input: &HashMap<String, String>) -> Value {
    let map = input
        .iter()
        .map(|(key, value)| (key.clone(), Value::Str(Rc::new(value.clone()))))
        .collect();
    Value::Map(Rc::new(RefCell::new(map)))
}

fn string_list_to_value(input: &[String]) -> Value {
    Value::List(Rc::new(RefCell::new(
        input
            .iter()
            .map(|item| Value::Str(Rc::new(item.clone())))
            .collect(),
    )))
}

fn collect_style_text(node: &Node, out: &mut String) {
    if let Node::Element(element) = node {
        if element.tag == "style" {
            for child in &element.children {
                if let Node::Text(text) = child {
                    if !out.is_empty() {
                        out.push('\n');
                    }
                    out.push_str(text);
                }
            }
        }
        for child in &element.children {
            collect_style_text(child, out);
        }
    }
}

fn collect_attr_values(document: &Document, tag: &str, attr: &str) -> Vec<String> {
    let mut out = Vec::new();
    for child in &document.children {
        collect_attr_values_from_node(child, tag, attr, &mut out);
    }
    out
}

fn collect_attr_values_from_node(node: &Node, tag: &str, attr: &str, out: &mut Vec<String>) {
    if let Node::Element(element) = node {
        if element.tag == tag {
            if let Some(value) = element.attrs.get(attr) {
                out.push(value.clone());
            }
        }
        for child in &element.children {
            collect_attr_values_from_node(child, tag, attr, out);
        }
    }
}

fn detect_app_roots(document: &Document) -> Vec<String> {
    let mut out = Vec::new();
    for child in &document.children {
        detect_app_roots_from_node(child, &mut out);
    }
    out
}

fn detect_app_roots_from_node(node: &Node, out: &mut Vec<String>) {
    if let Node::Element(element) = node {
        let id = element
            .attrs
            .get("id")
            .map(String::as_str)
            .unwrap_or_default();
        let classes = element
            .attrs
            .get("class")
            .map(String::as_str)
            .unwrap_or_default();
        let framework_attr = element.attrs.keys().any(|key| {
            key.starts_with("ng-")
                || key.starts_with("data-ng-")
                || key.starts_with("v-")
                || key.starts_with("data-react")
        });
        let appish_name = matches!(
            id,
            "app" | "root" | "__next" | "angular" | "ng-app" | "react-root"
        ) || classes
            .split_whitespace()
            .any(|class| matches!(class, "app" | "root" | "ng-app" | "react-root" | "vue-app"));
        if appish_name || framework_attr {
            let label = if id.is_empty() {
                format!("{}[class='{}']", element.tag, classes)
            } else {
                format!("{}#{}", element.tag, id)
            };
            out.push(label);
        }
        for child in &element.children {
            detect_app_roots_from_node(child, out);
        }
    }
}

struct HtmlParser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> HtmlParser<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn parse_document(&mut self) -> Document {
        Document {
            children: self.parse_nodes(None),
        }
    }

    fn parse_nodes(&mut self, until: Option<&str>) -> Vec<Node> {
        let mut nodes = Vec::new();
        while !self.eof() {
            if self.starts_with("<!--") {
                self.consume_comment();
                continue;
            }
            if self.starts_with("</") {
                let tag = self.peek_closing_tag();
                self.consume_until('>');
                self.consume_char();
                if until.is_none_or(|expected| tag == expected) {
                    break;
                }
                continue;
            }
            if self.starts_with("<") {
                if let Some(node) = self.parse_element() {
                    nodes.push(node);
                }
            } else {
                let text = decode_entities(&self.consume_text());
                if !text.is_empty() {
                    nodes.push(Node::Text(text));
                }
            }
        }
        nodes
    }

    fn parse_element(&mut self) -> Option<Node> {
        self.consume_char();
        self.consume_whitespace();
        let tag = self
            .consume_while(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == ':')
            .to_ascii_lowercase();
        if tag.is_empty() {
            self.consume_until('>');
            self.consume_char();
            return None;
        }
        let attrs = self.parse_attrs();
        self.consume_whitespace();
        let self_closing = self.starts_with("/");
        if self_closing {
            self.consume_char();
        }
        if self.starts_with(">") {
            self.consume_char();
        }
        let children = if self_closing || is_void_element(&tag) {
            Vec::new()
        } else if is_raw_text_element(&tag) {
            let text = self.consume_raw_text(&tag, false);
            if text.is_empty() {
                Vec::new()
            } else {
                vec![Node::Text(text)]
            }
        } else if is_rcdata_element(&tag) {
            let text = decode_entities(&self.consume_raw_text(&tag, true));
            if text.is_empty() {
                Vec::new()
            } else {
                vec![Node::Text(text)]
            }
        } else {
            self.parse_nodes(Some(&tag))
        };
        Some(Node::Element(Element {
            tag,
            attrs,
            children,
        }))
    }

    fn parse_attrs(&mut self) -> HashMap<String, String> {
        let mut attrs = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with(">") || self.starts_with("/") {
                break;
            }
            let name = self
                .consume_while(|ch| {
                    ch.is_ascii_alphanumeric() || ch == '-' || ch == ':' || ch == '_'
                })
                .to_ascii_lowercase();
            if name.is_empty() {
                self.consume_char();
                continue;
            }
            self.consume_whitespace();
            let value = if self.starts_with("=") {
                self.consume_char();
                self.consume_whitespace();
                self.consume_attr_value()
            } else {
                String::new()
            };
            attrs.insert(name, decode_entities(&value));
        }
        attrs
    }

    fn consume_attr_value(&mut self) -> String {
        match self.next_char() {
            Some('"') | Some('\'') => {
                let quote = self.consume_char().unwrap();
                let value = self.consume_while(|ch| ch != quote);
                self.consume_char();
                value
            }
            _ => self.consume_while(|ch| !ch.is_whitespace() && ch != '>'),
        }
    }

    fn consume_text(&mut self) -> String {
        self.consume_while(|ch| ch != '<')
    }

    fn consume_comment(&mut self) {
        if let Some(end) = self.src[self.pos..].find("-->") {
            self.pos += end + 3;
        } else {
            self.pos = self.src.len();
        }
    }

    fn consume_raw_text(&mut self, tag: &str, case_insensitive_end_tag: bool) -> String {
        let start = self.pos;
        let Some(end_start) = self.find_matching_end_tag(tag, case_insensitive_end_tag) else {
            self.pos = self.src.len();
            return self.src[start..].to_string();
        };
        let text = self.src[start..end_start].to_string();
        self.pos = end_start;
        self.consume_char();
        self.consume_char();
        self.consume_whitespace();
        self.consume_while(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == ':');
        self.consume_until('>');
        if self.starts_with(">") {
            self.consume_char();
        }
        text
    }

    fn find_matching_end_tag(&self, tag: &str, case_insensitive: bool) -> Option<usize> {
        let needle = format!("</{}", tag);
        if !case_insensitive {
            return self.src[self.pos..]
                .find(&needle)
                .map(|offset| self.pos + offset);
        }
        self.src[self.pos..]
            .to_ascii_lowercase()
            .find(&needle)
            .map(|offset| self.pos + offset)
    }

    fn peek_closing_tag(&self) -> String {
        let rest = self.src[self.pos + 2..].trim_start();
        rest.chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == ':')
            .collect::<String>()
            .to_ascii_lowercase()
    }

    fn consume_until(&mut self, target: char) {
        while !self.eof() && self.next_char() != Some(target) {
            self.consume_char();
        }
    }

    fn starts_with(&self, s: &str) -> bool {
        self.src[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn next_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn consume_char(&mut self) -> Option<char> {
        let ch = self.next_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn consume_while<F>(&mut self, mut pred: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut out = String::new();
        while let Some(ch) = self.next_char() {
            if !pred(ch) {
                break;
            }
            out.push(ch);
            self.consume_char();
        }
        out
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}

fn decode_entities(input: &str) -> String {
    let mut decoded = input.to_string();
    loop {
        let next = decoded
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'");
        if next == decoded {
            return decoded;
        }
        decoded = next;
    }
}

fn is_raw_text_element(tag: &str) -> bool {
    matches!(tag, "script" | "style")
}

fn is_rcdata_element(tag: &str) -> bool {
    matches!(tag, "title" | "textarea")
}

fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "source"
            | "track"
            | "wbr"
    )
}

pub fn html_to_value(args: &[Value]) -> Result<Value, String> {
    let source = expect_str(args.first(), "browser_parse_html")?;
    Ok(document_to_value(&parse_html(source)))
}

pub fn css_to_value(args: &[Value]) -> Result<Value, String> {
    let source = expect_str(args.first(), "browser_parse_css")?;
    Ok(css_rules_to_value(&parse_css(source)))
}

pub fn styles_to_value(args: &[Value]) -> Result<Value, String> {
    if !(1..=2).contains(&args.len()) {
        return Err(format!(
            "browser_styles: expected 1 to 2 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_styles")?;
    let css = match args.get(1) {
        Some(Value::Str(css)) => css.as_str(),
        Some(other) => {
            return Err(format!(
                "browser_styles: css must be str, got {}",
                other.type_name()
            ))
        }
        None => "",
    };
    let doc = parse_html(html);
    let styled = computed_styles(&doc, css)
        .iter()
        .map(styled_node_to_value)
        .collect();
    Ok(Value::List(Rc::new(RefCell::new(styled))))
}

pub fn query_selector_to_value(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "browser_query_selector: expected 2 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_query_selector")?;
    let selector = expect_str(args.get(1), "browser_query_selector")?;
    let doc = parse_html(html);
    let nodes = query_selector(&doc, selector)
        .iter()
        .map(node_to_value)
        .collect();
    Ok(Value::List(Rc::new(RefCell::new(nodes))))
}

pub fn text_content_to_value(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "browser_text_content: expected 2 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_text_content")?;
    let selector = expect_str(args.get(1), "browser_text_content")?;
    let doc = parse_html(html);
    let text = query_selector(&doc, selector)
        .first()
        .map(text_content)
        .unwrap_or_default();
    Ok(Value::Str(Rc::new(text)))
}

pub fn snapshot_to_value(args: &[Value]) -> Result<Value, String> {
    if !(1..=3).contains(&args.len()) {
        return Err(format!(
            "browser_snapshot: expected 1 to 3 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_snapshot")?;
    let css = match args.get(1) {
        Some(Value::Str(css)) => css.as_str(),
        Some(other) => {
            return Err(format!(
                "browser_snapshot: css must be str, got {}",
                other.type_name()
            ))
        }
        None => "",
    };
    let width = optional_width(args.get(2), "browser_snapshot")?;
    Ok(page_snapshot(&parse_html(html), css, width))
}

pub fn display_list_to_runtime_value(args: &[Value]) -> Result<Value, String> {
    if !(1..=3).contains(&args.len()) {
        return Err(format!(
            "browser_display_list: expected 1 to 3 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_display_list")?;
    let css = match args.get(1) {
        Some(Value::Str(css)) => css.as_str(),
        Some(other) => {
            return Err(format!(
                "browser_display_list: css must be str, got {}",
                other.type_name()
            ))
        }
        None => "",
    };
    let width = optional_width(args.get(2), "browser_display_list")?;
    let doc = parse_html(html);
    let layout = layout_document(&doc, css, width);
    Ok(display_list_to_value(&build_display_list(&layout)))
}

pub fn render_to_value(args: &[Value]) -> Result<Value, String> {
    if !(1..=3).contains(&args.len()) {
        return Err(format!(
            "browser_render: expected 1 to 3 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_render")?;
    let css = match args.get(1) {
        Some(Value::Str(css)) => css.as_str(),
        Some(other) => {
            return Err(format!(
                "browser_render: css must be str, got {}",
                other.type_name()
            ))
        }
        None => "",
    };
    let width = optional_width(args.get(2), "browser_render")?;
    let doc = parse_html(html);
    let layout = layout_document(&doc, css, width);
    Ok(Value::Str(Rc::new(render_text(&layout))))
}

pub fn raster_to_runtime_value(args: &[Value]) -> Result<Value, String> {
    if !(1..=5).contains(&args.len()) {
        return Err(format!(
            "browser_raster: expected 1 to 5 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_raster")?;
    let css = match args.get(1) {
        Some(Value::Str(css)) => css.as_str(),
        Some(other) => {
            return Err(format!(
                "browser_raster: css must be str, got {}",
                other.type_name()
            ))
        }
        None => "",
    };
    let options = RenderOptions {
        viewport_width: optional_width(args.get(2), "browser_raster")?,
        viewport_height: optional_height(args.get(3), "browser_raster")?,
        scale: optional_scale(args.get(4), "browser_raster")?,
        background: Rgba::WHITE,
    };
    let doc = parse_html(html);
    render_document_to_raster(&doc, css, options).map(raster_image_to_value)
}

pub fn render_ppm_to_value(args: &[Value]) -> Result<Value, String> {
    if !(1..=5).contains(&args.len()) {
        return Err(format!(
            "browser_render_ppm: expected 1 to 5 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_render_ppm")?;
    let css = match args.get(1) {
        Some(Value::Str(css)) => css.as_str(),
        Some(other) => {
            return Err(format!(
                "browser_render_ppm: css must be str, got {}",
                other.type_name()
            ))
        }
        None => "",
    };
    let options = RenderOptions {
        viewport_width: optional_width(args.get(2), "browser_render_ppm")?,
        viewport_height: optional_height(args.get(3), "browser_render_ppm")?,
        scale: optional_scale(args.get(4), "browser_render_ppm")?,
        background: Rgba::WHITE,
    };
    let doc = parse_html(html);
    let image = render_document_to_raster(&doc, css, options)?;
    Ok(Value::Bytes(Rc::new(RefCell::new(image.to_ppm()))))
}

pub fn layout_to_runtime_value(args: &[Value]) -> Result<Value, String> {
    if !(1..=3).contains(&args.len()) {
        return Err(format!(
            "browser_layout: expected 1 to 3 args, got {}",
            args.len()
        ));
    }
    let html = expect_str(args.first(), "browser_layout")?;
    let css = match args.get(1) {
        Some(Value::Str(css)) => css.as_str(),
        Some(other) => {
            return Err(format!(
                "browser_layout: css must be str, got {}",
                other.type_name()
            ))
        }
        None => "",
    };
    let width = optional_width(args.get(2), "browser_layout")?;
    let doc = parse_html(html);
    Ok(layout_to_value(&layout_document(&doc, css, width)))
}

/// ARIA landmark / role derived from the element's `role` attribute or implicit role.
#[derive(Debug, Clone, PartialEq)]
pub struct A11yNode {
    pub role: String,
    pub name: Option<String>,
    pub tag: String,
    pub focusable: bool,
    pub disabled: bool,
    pub children: Vec<A11yNode>,
}

/// Map of implicit ARIA roles for HTML elements (WHATWG spec).
fn implicit_role(tag: &str) -> Option<&'static str> {
    Some(match tag {
        "a" => "link",
        "article" => "article",
        "aside" => "complementary",
        "body" => "document",
        "button" => "button",
        "datalist" => "listbox",
        "dd" => "definition",
        "details" => "group",
        "dialog" => "dialog",
        "dt" => "term",
        "fieldset" => "group",
        "figure" => "figure",
        "footer" => "contentinfo",
        "form" => "form",
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => "heading",
        "header" => "banner",
        "hr" => "separator",
        "img" => "img",
        "input" => "textbox",
        "li" => "listitem",
        "link" => "link",
        "main" => "main",
        "menu" => "menu",
        "meter" => "meter",
        "nav" => "navigation",
        "ol" | "ul" => "list",
        "option" => "option",
        "output" => "status",
        "progress" => "progressbar",
        "section" => "region",
        "select" => "combobox",
        "summary" => "button",
        "table" => "table",
        "tbody" | "thead" | "tfoot" => "rowgroup",
        "td" => "cell",
        "textarea" => "textbox",
        "th" => "columnheader",
        "tr" => "row",
        _ => return None,
    })
}

/// Compute the ARIA role for an element: explicit `role` attr wins, then implicit.
fn element_role(element: &Element) -> String {
    if let Some(role) = element.attrs.get("role") {
        if !role.is_empty() {
            return role.to_ascii_lowercase();
        }
    }
    implicit_role(&element.tag).unwrap_or("generic").to_string()
}

/// Compute the accessible name for an element.
fn accessible_name(element: &Element) -> Option<String> {
    // aria-label wins
    if let Some(label) = element.attrs.get("aria-label") {
        if !label.is_empty() {
            return Some(label.clone());
        }
    }
    // aria-labelledby (we'd need to resolve IDs — return as-is for now)
    if let Some(labelledby) = element.attrs.get("aria-labelledby") {
        if !labelledby.is_empty() {
            return Some(format!("#{}", labelledby));
        }
    }
    // title attribute
    if let Some(title) = element.attrs.get("title") {
        if !title.is_empty() {
            return Some(title.clone());
        }
    }
    // alt attribute (images)
    if let Some(alt) = element.attrs.get("alt") {
        if !alt.is_empty() {
            return Some(alt.clone());
        }
    }
    // placeholder for inputs
    if let Some(placeholder) = element.attrs.get("placeholder") {
        if !placeholder.is_empty() {
            return Some(placeholder.clone());
        }
    }
    // label via for attribute — skip for now (requires ID resolution)
    None
}

/// Is the element keyboard-focusable?
fn is_focusable(element: &Element) -> bool {
    if element.attrs.contains_key("role") {
        return true;
    }
    if let Some(tabindex) = element.attrs.get("tabindex") {
        if let Ok(idx) = tabindex.parse::<i32>() {
            return idx >= 0;
        }
    }
    matches!(
        element.tag.as_str(),
        "a" | "button" | "input" | "select" | "textarea" | "summary" | "details"
    ) && !element.attrs.contains_key("disabled")
}

/// Recursively build the accessibility tree from a DOM node.
fn build_a11y_node(node: &Node) -> Option<A11yNode> {
    match node {
        Node::Text(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(A11yNode {
                    role: "text".into(),
                    name: Some(trimmed.to_string()),
                    tag: "#text".into(),
                    focusable: false,
                    disabled: false,
                    children: Vec::new(),
                })
            }
        }
        Node::Element(element) => {
            // Skip elements with aria-hidden="true" or display:none via hidden attr
            if element
                .attrs
                .get("aria-hidden")
                .is_some_and(|v| v == "true")
            {
                return None;
            }
            let role = element_role(element);
            // For presentational role, skip generating a11y node but still process children
            if role == "presentation" || role == "none" {
                let mut children = Vec::new();
                for child in &element.children {
                    if let Some(a11y) = build_a11y_node(child) {
                        children.push(a11y);
                    }
                }
                // Return first child directly or merge
                if children.len() == 1 {
                    return Some(children.into_iter().next().unwrap());
                }
                // Merge all children into a generic container
                return Some(A11yNode {
                    role: "generic".into(),
                    name: None,
                    tag: element.tag.clone(),
                    focusable: false,
                    disabled: false,
                    children,
                });
            }

            let mut children = Vec::new();
            for child in &element.children {
                if let Some(a11y) = build_a11y_node(child) {
                    children.push(a11y);
                }
            }

            Some(A11yNode {
                role,
                name: accessible_name(element),
                tag: element.tag.clone(),
                focusable: is_focusable(element),
                disabled: element.attrs.contains_key("disabled"),
                children,
            })
        }
    }
}

/// Build a full accessibility tree from a document.
pub fn build_accessibility_tree(document: &Document) -> Vec<A11yNode> {
    document
        .children
        .iter()
        .filter_map(build_a11y_node)
        .collect()
}

/// Convert an accessibility tree to a Value for the runtime.
pub fn a11y_tree_to_value(nodes: &[A11yNode]) -> Value {
    Value::List(Rc::new(RefCell::new(
        nodes.iter().map(a11y_node_to_value).collect(),
    )))
}

fn a11y_node_to_value(node: &A11yNode) -> Value {
    let mut map = HashMap::new();
    map.insert("role".into(), Value::Str(Rc::new(node.role.clone())));
    map.insert(
        "name".into(),
        match &node.name {
            Some(n) => Value::Str(Rc::new(n.clone())),
            None => Value::Nil,
        },
    );
    map.insert("tag".into(), Value::Str(Rc::new(node.tag.clone())));
    map.insert("focusable".into(), Value::Bool(node.focusable));
    map.insert("disabled".into(), Value::Bool(node.disabled));
    map.insert(
        "children".into(),
        Value::List(Rc::new(RefCell::new(
            node.children.iter().map(a11y_node_to_value).collect(),
        ))),
    );
    Value::Map(Rc::new(RefCell::new(map)))
}

/// Find the layout box that corresponds to a given element at a path.
/// The path is a sequence of child indices to traverse from the layout root.
pub fn find_layout_box_at_path<'a>(layout: &'a LayoutBox, path: &[usize]) -> Option<&'a LayoutBox> {
    if path.is_empty() {
        return Some(layout);
    }
    let mut current = layout;
    for &idx in path {
        current = current.children.get(idx)?;
    }
    Some(current)
}

/// Collect all focusable elements from a document, returning (path, element) pairs.
pub fn collect_focusable(document: &Document) -> Vec<(Vec<usize>, Element)> {
    let mut results = Vec::new();
    for (i, child) in document.children.iter().enumerate() {
        collect_focusable_recursive(child, &[i], &mut results);
    }
    results
}

fn collect_focusable_recursive(
    node: &Node,
    path: &[usize],
    results: &mut Vec<(Vec<usize>, Element)>,
) {
    if let Node::Element(el) = node {
        if is_focusable(el) {
            results.push((path.to_vec(), el.clone()));
        }
        for (i, child) in el.children.iter().enumerate() {
            let mut child_path = path.to_vec();
            child_path.push(i);
            collect_focusable_recursive(child, &child_path, results);
        }
    }
}

fn optional_width(value: Option<&Value>, name: &str) -> Result<i64, String> {
    match value {
        Some(Value::Int(width)) => Ok(*width),
        Some(other) => Err(format!(
            "{}: width must be int, got {}",
            name,
            other.type_name()
        )),
        None => Ok(80),
    }
}

fn optional_height(value: Option<&Value>, name: &str) -> Result<Option<i64>, String> {
    match value {
        Some(Value::Int(height)) if *height > 0 => Ok(Some(*height)),
        Some(Value::Nil) | None => Ok(None),
        Some(Value::Int(_)) => Err(format!("{}: height must be positive", name)),
        Some(other) => Err(format!(
            "{}: height must be int or nil, got {}",
            name,
            other.type_name()
        )),
    }
}

fn optional_scale(value: Option<&Value>, name: &str) -> Result<usize, String> {
    match value {
        Some(Value::Int(scale)) if *scale > 0 => Ok(*scale as usize),
        Some(Value::Int(_)) => Err(format!("{}: scale must be positive", name)),
        Some(other) => Err(format!(
            "{}: scale must be int, got {}",
            name,
            other.type_name()
        )),
        None => Ok(8),
    }
}

fn expect_str<'a>(value: Option<&'a Value>, name: &str) -> Result<&'a str, String> {
    match value {
        Some(Value::Str(s)) => Ok(s.as_str()),
        Some(other) => Err(format!("{}: expected str, got {}", name, other.type_name())),
        None => Err(format!("{}: expected html string", name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_html_to_dom_value() {
        let doc = parse_html(
            r#"<main id="app"><h1>Hello &amp; hi</h1><br><p class="note">World</p></main>"#,
        );
        assert_eq!(doc.children.len(), 1);
        match &doc.children[0] {
            Node::Element(el) => {
                assert_eq!(el.tag, "main");
                assert_eq!(el.attrs.get("id"), Some(&"app".to_string()));
                assert_eq!(el.children.len(), 3);
            }
            other => panic!("expected element, got {other:?}"),
        }
    }

    #[test]
    fn lays_out_and_renders_text_display_list() {
        let doc = parse_html(r#"<div class="card"><h1>Title</h1><p>Hello</p></div>"#);
        let layout = layout_document(
            &doc,
            ".card { width: 20px; padding: 1px } h1 { height: 2px }",
            80,
        );
        let rendered = render_text(&layout);
        assert!(rendered.contains("viewport 80x"));
        assert!(rendered.contains("<div> @0,0 20x"));
        assert!(rendered.contains("\"Title\""));
    }

    #[test]
    fn layout_applies_individual_margin_padding_and_border_widths() {
        let doc = parse_html(r#"<section><p>Hi</p></section><aside>Next</aside>"#);
        let layout = layout_document(
            &doc,
            "section { width: 20px; margin: 1px 2px 3px 4px; padding: 5px 6px 7px 8px; border-width: 1px 2px 3px 4px } p { height: 2px } aside { height: 1px }",
            80,
        );

        let section = &layout.children[0];
        assert_eq!((section.x, section.y), (4, 1));
        assert_eq!(section.width, 40);
        assert_eq!(section.height, 18);
        assert_eq!((section.children[0].x, section.children[0].y), (16, 7));

        let aside = &layout.children[1];
        assert_eq!(aside.y, 23);
    }

    #[test]
    fn layout_clamps_width_height_and_supports_border_box() {
        let doc = parse_html(r#"<main><p>wide text</p></main>"#);
        let layout = layout_document(
            &doc,
            "main { box-sizing: border-box; width: 20px; min-width: 24px; max-width: 30px; height: 2px; min-height: 5px; max-height: 6px; padding-left: 3px; padding-right: 4px; padding-top: 1px; padding-bottom: 2px; border-width: 1px }",
            80,
        );

        let main = &layout.children[0];
        assert_eq!(main.width, 24);
        assert_eq!(main.height, 11);
        assert_eq!((main.children[0].x, main.children[0].y), (4, 2));
        assert_eq!(main.children[0].width, 15);
    }

    #[test]
    fn browser_builtins_return_values() {
        let rendered = render_to_value(&[
            Value::Str(Rc::new("<h1>Hello</h1>".into())),
            Value::Str(Rc::new("h1 { height: 2px }".into())),
            Value::Int(40),
        ])
        .unwrap();
        match rendered {
            Value::Str(text) => assert!(text.contains("<h1>")),
            other => panic!("expected str, got {other:?}"),
        }
    }

    #[test]
    fn decodes_nested_entities() {
        let doc = parse_html("<p>&amp;lt;tag&amp;gt;</p>");
        let Node::Element(element) = &doc.children[0] else {
            panic!("expected element");
        };
        assert_eq!(element.children, vec![Node::Text("<tag>".into())]);
    }

    #[test]
    fn script_and_style_parse_as_raw_text() {
        let doc = parse_html(
            r#"<script>if (a < b) { document.write("&lt;p&gt;"); }</script><style>.x::before { content: "<"; }</style>"#,
        );
        let Node::Element(script) = &doc.children[0] else {
            panic!("expected script");
        };
        assert_eq!(
            script.children,
            vec![Node::Text(
                r#"if (a < b) { document.write("&lt;p&gt;"); }"#.into()
            )]
        );
        let Node::Element(style) = &doc.children[1] else {
            panic!("expected style");
        };
        assert_eq!(
            style.children,
            vec![Node::Text(r#".x::before { content: "<"; }"#.into())]
        );
    }

    #[test]
    fn title_and_textarea_parse_as_rcdata() {
        let doc =
            parse_html("<title>A &amp; B < C</title><textarea>One &lt; two</textarea><p>after</p>");
        let Node::Element(title) = &doc.children[0] else {
            panic!("expected title");
        };
        assert_eq!(title.children, vec![Node::Text("A & B < C".into())]);
        let Node::Element(textarea) = &doc.children[1] else {
            panic!("expected textarea");
        };
        assert_eq!(textarea.children, vec![Node::Text("One < two".into())]);
        let Node::Element(p) = &doc.children[2] else {
            panic!("expected p");
        };
        assert_eq!(p.children, vec![Node::Text("after".into())]);
    }

    #[test]
    fn clamps_negative_explicit_height() {
        let doc = parse_html("<section><p>Hello</p></section>");
        let layout = layout_document(&doc, "section { height: -5px }", 80);

        assert_eq!(layout.children[0].height, 0);
        assert_eq!(layout.height, 1);
    }

    #[test]
    fn css_supports_compound_descendant_and_inline_cascade() {
        let doc = parse_html(
            r#"<main id="app"><p class="note strong" style="height: 4px">Hello</p><p class="note">World</p></main>"#,
        );
        let styled = computed_styles(
            &doc,
            "p { height: 1px } main .note { color: blue } #app p.strong { height: 2px; color: red }",
        );
        let Node::Element(main) = &styled[0].node else {
            panic!("expected main");
        };
        assert_eq!(main.tag, "main");
        assert_eq!(
            styled[0].children[0].styles.get("color"),
            Some(&"red".into())
        );
        assert_eq!(
            styled[0].children[0].styles.get("height"),
            Some(&"4px".into())
        );
        assert_eq!(
            styled[0].children[1].styles.get("color"),
            Some(&"blue".into())
        );
    }

    #[test]
    fn query_selector_finds_descendant_matches() {
        let doc = parse_html(
            r#"<main id="app"><p class="note">One</p><section><p>Two</p></section></main>"#,
        );
        assert_eq!(query_selector(&doc, "#app p").len(), 2);
        assert_eq!(query_selector(&doc, "main .note").len(), 1);
    }

    #[test]
    fn query_selector_supports_attributes_and_text_content() {
        let doc = parse_html(
            r#"<div id="root" data-reactroot><button aria-label="Save"> Save <span>now</span></button></div>"#,
        );
        assert_eq!(query_selector(&doc, "[data-reactroot]").len(), 1);
        let buttons = query_selector(&doc, "button[aria-label='Save']");
        assert_eq!(buttons.len(), 1);
        assert_eq!(text_content(&buttons[0]), "Save now");
    }

    #[test]
    fn snapshot_extracts_framework_resources_and_embedded_css() {
        let doc = parse_html(
            r#"<html><head><link href="/app.css"><style>#root { background: green }</style><script src="/bundle.js"></script></head><body><div id="root"><img src="/logo.png"></div></body></html>"#,
        );
        let snapshot = page_snapshot(&doc, "", 80);
        let Value::Map(map) = snapshot else {
            panic!("expected snapshot map");
        };
        assert!(map.borrow().contains_key("display_list"));
        assert!(map
            .borrow()
            .get("css")
            .unwrap()
            .to_string()
            .contains("#root"));
        assert!(detect_app_roots(&doc).contains(&"div#root".to_string()));
    }

    #[test]
    fn display_list_contains_background_and_text_commands() {
        let doc = parse_html(r#"<div class="card"><p>Hello</p></div>"#);
        let layout = layout_document(&doc, ".card { background: red } p { color: blue }", 80);
        let commands = build_display_list(&layout);
        assert!(commands
            .iter()
            .any(|cmd| matches!(cmd, DisplayCommand::Rect { color, .. } if color == "red")));
        assert!(commands.iter().any(|cmd| matches!(cmd, DisplayCommand::Text { text, color, .. } if text == "Hello" && color == "blue")));
    }

    #[test]
    fn software_renderer_paints_backgrounds_text_and_ppm() {
        let doc = parse_html(r#"<div class="card">Hi</div>"#);
        let image = render_document_to_raster(
            &doc,
            ".card { background: #00ff00; color: blue; width: 4px; height: 2px }",
            RenderOptions {
                viewport_width: 8,
                viewport_height: Some(4),
                scale: 8,
                ..RenderOptions::default()
            },
        )
        .unwrap();

        assert_eq!((image.width, image.height), (64, 32));
        assert_eq!(
            image.pixel(10, 10),
            Some(Rgba {
                r: 0,
                g: 255,
                b: 0,
                a: 255
            })
        );
        assert_eq!(
            image.pixel(0, 0),
            Some(Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 255
            })
        );
        assert!(image.to_ppm().starts_with(b"P6\n64 32\n255\n"));
    }

    #[test]
    fn browser_raster_builtin_returns_rgba_bytes() {
        let value = raster_to_runtime_value(&[
            Value::Str(Rc::new(
                "<main style='background: red; width: 2px; height: 2px'></main>".into(),
            )),
            Value::Str(Rc::new(String::new())),
            Value::Int(4),
            Value::Int(4),
            Value::Int(2),
        ])
        .unwrap();
        let Value::Map(map) = value else {
            panic!("expected raster map");
        };
        let borrowed = map.borrow();
        assert_eq!(borrowed.get("width").unwrap().to_string(), "8");
        assert_eq!(borrowed.get("height").unwrap().to_string(), "8");
        let Some(Value::Bytes(bytes)) = borrowed.get("pixels") else {
            panic!("expected pixel bytes");
        };
        assert_eq!(bytes.borrow().len(), 8 * 8 * 4);
    }

    #[test]
    fn img_src_attribute_carried_into_display_command() {
        let doc = parse_html(r#"<img src="photo.png">"#);
        let layout = layout_document(&doc, "", 80);
        let commands = build_display_list(&layout);
        let image_cmd = commands
            .iter()
            .find(|cmd| matches!(cmd, DisplayCommand::Image { .. }));
        assert!(
            image_cmd.is_some(),
            "expected an Image display command for <img>"
        );
        if let Some(DisplayCommand::Image { src, .. }) = image_cmd {
            assert_eq!(
                src, "photo.png",
                "img src should come from DOM attribute, not styles"
            );
        }
    }

    #[test]
    fn flex_row_positions_children_horizontally() {
        let doc = parse_html(
            r#"<div style="display:flex; width:300px"><span style="width:100px; height:20px">A</span><span style="width:100px; height:20px">B</span></div>"#,
        );
        let layout = layout_document(&doc, "", 80);
        let container = &layout.children[0];
        assert_eq!(container.children.len(), 2);
        // Children should be side by side on the same Y
        assert_eq!(container.children[0].y, container.children[1].y);
        // Second child starts after the first
        assert!(container.children[1].x > container.children[0].x);
    }

    #[test]
    fn positioned_boxes_use_left_top_in_layout_and_display_list() {
        let doc = parse_html(
            "<main><button style='position:absolute;left:12px;top:7px;width:5px;height:3px;background:red'></button><div style='height:2px'></div></main><aside style='position:fixed;left:9px;top:4px;width:8px;height:2px;background:blue'></aside>",
        );
        let layout = layout_document(&doc, "", 80);
        let absolute = &layout.children[0].children[0];
        let fixed = &layout.children[1];
        assert_eq!(
            (absolute.x, absolute.y, absolute.width, absolute.height),
            (12, 7, 5, 3)
        );
        assert_eq!((fixed.x, fixed.y, fixed.width, fixed.height), (9, 4, 8, 2));
        let commands = build_display_list(&layout);
        assert!(commands.iter().any(|cmd| matches!(
            cmd,
            DisplayCommand::Rect { x: 12, y: 7, width: 5, height: 3, color } if color == "red"
        )));
    }

    #[test]
    fn overflow_hidden_clamps_child_layout_bounds() {
        let doc = parse_html(
            "<section style='overflow:hidden;width:10px;height:5px'><button style='position:absolute;left:8px;top:2px;width:10px;height:4px;background:red'></button></section>",
        );
        let layout = layout_document(&doc, "", 80);
        let child = &layout.children[0].children[0];
        assert_eq!((child.x, child.y, child.width, child.height), (8, 2, 2, 3));
        let commands = build_display_list(&layout);
        assert!(commands.iter().any(|cmd| matches!(
            cmd,
            DisplayCommand::Rect { x: 8, y: 2, width: 2, height: 3, color } if color == "red"
        )));
    }

    #[test]
    fn positioned_z_index_orders_display_commands() {
        let doc = parse_html(
            "<button style='position:absolute;left:0;top:0;width:4px;height:2px;background:red;z-index:10'></button><div style='position:absolute;left:0;top:0;width:4px;height:2px;background:blue'></div>",
        );
        let layout = layout_document(&doc, "", 80);
        let commands = build_display_list(&layout);
        let colors: Vec<&str> = commands
            .iter()
            .filter_map(|cmd| match cmd {
                DisplayCommand::Rect { color, .. } => Some(color.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(colors, vec!["blue", "red"]);
    }

    #[test]
    fn browser_variadics_reject_extra_args() {
        let args = [
            Value::Str(Rc::new("<h1>Hello</h1>".into())),
            Value::Str(Rc::new("".into())),
            Value::Int(80),
            Value::Int(1),
        ];

        assert!(render_to_value(&args)
            .unwrap_err()
            .contains("expected 1 to 3 args"));
        assert!(layout_to_runtime_value(&args)
            .unwrap_err()
            .contains("expected 1 to 3 args"));
    }
}
