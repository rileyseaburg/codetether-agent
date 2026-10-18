//! Construct media tools for the ordered catalog.
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::WebFetch => Arc::new(codetether_webfetch::WebFetchTool::new()) as Arc<dyn Tool>,
        ToolKind::WebSearch => {
            Arc::new(codetether_websearch::WebSearchTool::new()) as Arc<dyn Tool>
        }
        ToolKind::BrowserCtl => {
            Arc::new(codetether_browser_tool::BrowserCtlTool::new()) as Arc<dyn Tool>
        }
        ToolKind::Voice => Arc::new(codetether_voice_tool::VoiceTool::new()) as Arc<dyn Tool>,
        ToolKind::VoiceInput => {
            Arc::new(codetether_voice_input_tool::VoiceInputTool::new()) as Arc<dyn Tool>
        }
        ToolKind::VoiceStream => {
            Arc::new(codetether_voice_stream_tool::VoiceStreamTool::new()) as Arc<dyn Tool>
        }
        ToolKind::K8s => Arc::new(codetether_k8s_tool::K8sTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
