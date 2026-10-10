import Foundation

/// The same availability rules drive buttons and their visible explanations.
extension ScreenModel {
    var sessionBlockReason: String? {
        if creating { return "Wait for the session to be created." }
        if stopping { return "Wait for Stop to finish." }
        guard let session else { return "Pair Windows to send a message." }
        if (session.expiry ?? .distantPast) <= Date() {
            return "This session expired. Create a new session and pair Windows again."
        }
        if !active { return "Open the Screen tab to reconnect." }
        if !connected || retryBlocked { return "Reconnect the session above to send. Your draft is kept." }
        if !response.isPaired { return "Enter the pairing code in the Windows companion first." }
        if response.status == "paused" { return "Resume sharing in the Windows companion first." }
        return nil
    }

    var questionBlockReason: String? {
        if let reason = sessionBlockReason { return reason }
        if !(client is ScreenQuestionNetworking) { return "Screen questions are unavailable in this client." }
        if questions.submitting || replies.submitting { return "Sending your message…" }
        if !response.canRequestFrame || questions.waitingForFrame {
            return "Wait for the current screen request to finish."
        }
        return nil
    }

    var replyBlockReason: String? {
        if let reason = sessionBlockReason { return reason }
        if !(client is ScreenReplyNetworking) { return "Windows typing is unavailable in this client." }
        if questions.submitting || replies.submitting { return "Sending your message…" }
        return nil
    }

    /// A failed automatic retry must not disable the manual recovery action.
    var canReconnect: Bool {
        active && !creating && !stopping && !connected
            && (session?.expiry ?? .distantPast) > Date()
    }

    var composerBusy: Bool { questions.submitting || replies.submitting }
}