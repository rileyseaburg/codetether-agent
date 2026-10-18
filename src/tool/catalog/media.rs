//! Native media tool constructors.
use super::*;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::WebFetch => Arc::new(webfetch::WebFetchTool::new()) as Arc<dyn Tool>,
        ToolKind::WebSearch => Arc::new(websearch::WebSearchTool::new()) as Arc<dyn Tool>,
        ToolKind::BrowserCtl => Arc::new(browserctl::BrowserCtlTool::new()) as Arc<dyn Tool>,
        ToolKind::Voice => Arc::new(voice::VoiceTool::new()) as Arc<dyn Tool>,
        ToolKind::VoiceInput => Arc::new(voice_input::VoiceInputTool::new()) as Arc<dyn Tool>,
        ToolKind::VoiceStream => Arc::new(voice_stream::VoiceStreamTool::new()) as Arc<dyn Tool>,
        ToolKind::K8s => Arc::new(k8s_tool::K8sTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
