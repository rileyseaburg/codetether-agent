import Foundation

extension ScreenModel {
    var canSendReply: Bool { replyBlockReason == nil }

    /// Queues manually requested text, never the entire analysis or question.
    func sendReply(_ text: String? = nil) {
        guard canSendReply, let owned = session,
              let sender = client as? ScreenReplyNetworking else { return }
        let body = ScreenReply(text: text ?? replies.draft)
        guard ScreenTypingText.valid(body.text) else {
            replies.error = "Use one nonblank line of up to 2,000 characters; no Enter, tabs or control characters."; return
        }
        replies.cancel()
        replies.submitting = true
        let ticket = replies.generation
        replies.task = Task { [weak self] in
            guard let self else { return }
            defer {
                if replies.generation == ticket { replies.submitting = false; replies.task = nil }
            }
            do {
                _ = try await sender.send(body, session: owned.id, token: credential())
                guard !Task.isCancelled, replies.generation == ticket, session?.id == owned.id else { return }
                if text == nil, replies.draft == body.text { replies.draft = "" }
                replies.notice = "Queued for Windows, not confirmed as typed. Check the Windows target field."
            } catch {
                guard !Task.isCancelled, replies.generation == ticket, session?.id == owned.id else { return }
                replies.error = ScreenFailure.message(error)
                replies.notice = "No automatic retry was sent. Check Windows before resending."
            }
        }
    }
}
