import Foundation

extension ChatModel {
    /// Voice uses the same cancellable, conversation-owned request path as Chat.
    func sendAsync(_ text: String) async -> String? {
        guard let turn = beginTurn(text, owner: .voice) else { return nil }
        let request = Task { await executeTurn(turn) }
        requestTask = request
        return await withTaskCancellationHandler {
            await request.value
        } onCancel: { request.cancel() }
    }
}
