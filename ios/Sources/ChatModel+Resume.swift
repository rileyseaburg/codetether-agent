import Foundation

extension ChatModel {
    func clear() {
        conversationID = UUID()
        activeTurnID = nil
        requestTask?.cancel(); requestTask = nil
        sessionCreation?.cancel(); sessionCreation = nil; sessionCreationID = nil
        busy = false; loading = false
        messages = []; tools = []; attachments = []
        sessionID = nil; draft = ""; error = nil; replyForSpeech = nil
        activeAssistantItem = nil; completedItems = []; currentReplyID = nil
        agentStatus = ""
        UserDefaults.standard.removeObject(forKey: "agent.session")
    }
    func resume(_ id: String) async {
        guard UUID(uuidString: id) != nil else { return }
        clear()
        sessionID = id
        UserDefaults.standard.set(id, forKey: "agent.session")
        await restoreSession()
    }
}