import SwiftUI

/// Stop local audio before resetting the server conversation.
extension ChatView {
    func startNewConversation() async {
        voiceInput.stop(); voiceOutput.stop()
        await chat.newConversation()
    }
}