import Foundation

extension ScreenModel {
    /// Clears owner state only after acknowledged revocation or session expiry.
    func finishSession(_ message: String) {
        invalidateStream()
        expiryTask?.cancel()
        expiryTask = nil
        questions.cancel(clearDraft: true)
        replies.cancel(clearDraft: true)
        session = nil
        activeModel = ""
        retryBlocked = false
        error = nil
        notice = message
    }
}
