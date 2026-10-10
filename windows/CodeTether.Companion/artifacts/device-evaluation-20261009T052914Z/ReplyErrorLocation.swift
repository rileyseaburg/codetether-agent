import XCTest

/// Identify error ownership without exporting drafts or performing a request.
final class ReplyErrorLocation: XCTestCase {
    struct Observation: Encodable {
        let foreground: Bool
        let sessionError: [String]
        let sessionErrorExists: Bool
        let replyError: [String]
        let orderedKnownLabels: [String]
    }

    func testErrorLocation() throws {
        let app = XCUIApplication(bundleIdentifier: "run.codetether.ios")
        XCTAssertEqual(app.state, .runningForeground)
        guard app.state == .runningForeground else { return }
        let sessionError = app.staticTexts["screen-error"]
        let replyError = app.staticTexts["screen-reply-error"]
        let headings = ["Type a reply on Windows", "Ask about the screen", "Screen analysis",
                        "Live analysis stream", "Analysis stream disconnected", "Stream paused. Reconnect or Stop."]
        let labels = app.staticTexts.allElementsBoundByIndex.map(\.label)
        let known = labels.flatMap { label in
            headings.contains(label) ? [label] : ReplyErrors.matching([label])
        }
        let result = Observation(foreground: true,
            sessionError: sessionError.exists ? ReplyErrors.matching([sessionError.label]) : [],
            sessionErrorExists: sessionError.exists,
            replyError: replyError.exists ? ReplyErrors.matching([replyError.label]) : [],
            orderedKnownLabels: known)
        let data = try JSONEncoder().encode(result)
        let directory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
        let destination = directory.appendingPathComponent("reply-observation-05.json")
        guard !FileManager.default.fileExists(atPath: destination.path) else {
            throw NSError(domain: "ReplyObservationEvidenceExists", code: 1)
        }
        try data.write(to: destination, options: .atomic)
        print("REPLY_ERROR_LOCATION " + String(decoding: data, as: UTF8.self))
        // Deliberately leave owner application, pairing and Windows untouched.
    }
}