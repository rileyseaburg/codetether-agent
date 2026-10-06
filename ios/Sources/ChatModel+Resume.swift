import Foundation

extension ChatModel {
    func clear() {
        guard !busy else { return }
        messages = []; tools = []; attachments = []; generatedImages = []
        sessionID = nil; draft = ""; error = nil; replyForSpeech = nil
        activeAssistantItem = nil; completedItems = []; currentReplyID = nil
        UserDefaults.standard.removeObject(forKey: "agent.session")
    }
    func resume(_ id: String) async {
        guard !busy, UUID(uuidString: id) != nil else { return }
        sessionID = id
        UserDefaults.standard.set(id, forKey: "agent.session")
        messages = []
        attachments = []
        generatedImages = []
        tools = []
        error = nil
        draft = ""
        currentReplyID = nil; activeAssistantItem = nil; completedItems = []
        await restoreSession()
    }
}
