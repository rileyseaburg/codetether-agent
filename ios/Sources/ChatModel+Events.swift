import Foundation

extension ChatModel {
    func receive(_ event: AgentFrame.Event) {
        if event.kind == "item.started", event.payload?.item_type == "assistant_text", let id = event.payload?.item_id {
            activeAssistantItem = id
        }
        if event.kind == "item.completed", let id = event.payload?.item_id, id == activeAssistantItem,
           let text = event.payload?.text, !text.isEmpty, rememberCompletedItem(id) {
            upsertReply(TranscriptPresentation.displayText(text))
            replyForSpeech = TranscriptPresentation.displayText(text)
        }
        if event.kind == "tool.started", agentStatus != "Working…" {
            agentStatus = "Working…"
        }
        if event.kind == "tool.completed", let name = event.payload?.name {
            if tools.count < 64 && !tools.contains(name) { tools.append(name) }
        }
        if event.kind == "tool.metadata", let path = event.payload?.metadata?.saved_path {
            if !generatedImages.contains(path) { generatedImages.append(path) }
        }
    }
}
