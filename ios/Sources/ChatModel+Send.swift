import Foundation

extension ChatModel {
    func send() {
        guard let turn = beginTurn(draft, owner: .chat) else { return }
        requestTask = Task { await executeTurn(turn) }
    }
}