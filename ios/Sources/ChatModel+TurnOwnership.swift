import Foundation

/// Guard every asynchronous boundary and clear state only for the owning turn.
extension ChatModel {
    func requireTurn(_ id: UUID) throws {
        try Task.checkCancellation()
        guard activeTurnID == id else { throw CancellationError() }
    }

    func finishTurn(_ id: UUID) {
        guard activeTurnID == id else { return }
        busy = false
        requestTask = nil
        activeAssistantItem = nil
        activeTurnID = nil
    }
}
