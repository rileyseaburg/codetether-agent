import XCTest

/// Only bounded status flags leave the phone; never credentials, drafts, or full answers.
struct ManualState: Encodable {
    let id: Int
    let message: String
    let foreground: Bool
    let questionVisible: Bool
    let askEnabled: Bool
    let submitted: Bool
    let visibleStatuses: [String]
    let markerPresent: Bool
    let typingQueued: Bool
    let errorVisible: Bool
    let observedFailures: [String]
    init(id: Int, message: String, bridge: ManualBridge) {
        self.id = id; self.message = message
        let app = bridge.app
        foreground = app.state == .runningForeground
        questionVisible = app.descendants(matching: .any).matching(identifier: "screen-question").firstMatch.isHittable
        let ask = app.buttons["screen-ask"]
        askEnabled = ask.exists && ask.isEnabled
        submitted = bridge.submitted
        let labels = app.staticTexts.allElementsBoundByIndex.map(\.label)
        let allowed = ["Ready", "Waiting for Windows", "Analyzing", "Paused", "Stopped", "Analysis error",
                       "Live analysis stream", "Analysis stream disconnected", "Waiting for Windows to capture…",
                       "Analyzing a fresh screenshot…", "Screen session is unavailable or expired."]
        let failures = ["Windows did not provide a fresh screenshot within 60 seconds.",
                        "Analysis failed. Check the selected vision model and try again.",
                        "Analysis interrupted or exceeded its limit. Request another capture.",
                        "Screen session is unavailable or expired."]
        observedFailures = failures.filter { failure in labels.contains { $0.contains(failure) } }
        visibleStatuses = labels.filter { allowed.contains($0) }
        markerPresent = labels.contains { $0.contains("CT-LIVE-052914") }
        typingQueued = labels.contains { $0.contains("Typing queued for Windows") }
        errorVisible = app.descendants(matching: .any).matching(identifier: "screen-error").firstMatch.exists
    }
}