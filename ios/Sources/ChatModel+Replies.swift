import Foundation

extension ChatModel {
    /// One stable reply bubble per turn; completed event replay cannot grow history.
    func upsertReply(_ text: String) {
        appendReplyImages(ImageReferencePaths.extract(text))
        guard !text.isEmpty else { return }
        if let id = currentReplyID, let index = messages.firstIndex(where: { $0.id == id }) {
            if messages[index].content != text {
                messages[index] = ChatMessage(id: id, role: "assistant", content: text,
                                             imagePaths: messages[index].imagePaths)
            }
        } else {
            let message = ChatMessage(role: "assistant", content: text)
            currentReplyID = message.id
            messages.append(message)
        }
    }
    func rememberCompletedItem(_ id: String) -> Bool {
        guard !completedItems.contains(id) else { return false }
        if completedItems.count == 128 { completedItems.removeFirst() }
        completedItems.append(id)
        return true
    }
}