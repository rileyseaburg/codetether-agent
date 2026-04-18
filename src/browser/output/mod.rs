mod ack;
mod content;
mod eval;
mod screenshot;
mod snapshot;
mod tabs;
mod toggle;

pub use ack::Ack;
pub use content::{HtmlContent, TextContent};
pub use eval::EvalOutput;
pub use screenshot::ScreenshotData;
pub use snapshot::PageSnapshot;
pub use tabs::TabList;
pub use toggle::ToggleOutput;

#[allow(dead_code)]
pub enum BrowserOutput {
    Ack(Ack),
    Eval(EvalOutput),
    Html(HtmlContent),
    Json(serde_json::Value),
    Screenshot(ScreenshotData),
    Snapshot(PageSnapshot),
    Tabs(TabList),
    Text(TextContent),
    Toggle(ToggleOutput),
}
