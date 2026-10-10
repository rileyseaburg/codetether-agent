import XCTest

/// One observation-only run: never launch, type, tap Send, reconnect, or stop.
final class ReplyObservation: XCTestCase {
    struct Sample: Encodable {
        let step: Int
        let foreground: Bool
        let replyVisible: Bool
        let replySendEnabled: Bool
        let errorsAndNotices: [String]
    }

    func testExistingReplyError() throws {
        let app = XCUIApplication(bundleIdentifier: "run.codetether.ios")
        var samples: [Sample] = []
        for step in 0..<7 {
            let foreground = app.state == .runningForeground
            guard foreground else {
                samples.append(Sample(step: step, foreground: false, replyVisible: false,
                    replySendEnabled: false, errorsAndNotices: []))
                break
            }
            let reply = app.descendants(matching: .any).matching(identifier: "screen-reply").firstMatch
            let button = app.buttons["screen-reply-send"]
            let visible = reply.exists && reply.isHittable
            let labels = app.staticTexts.allElementsBoundByIndex.map(\.label)
            samples.append(Sample(step: step, foreground: true, replyVisible: visible,
                replySendEnabled: button.exists && button.isEnabled,
                errorsAndNotices: ReplyErrors.matching(labels)))
            // One further scroll after revealing the input exposes its error/footer.
            if step > 0 && samples[step - 1].replyVisible { break }
            // Gesture stays in the form margin, away from editor selection and Send.
            let start = app.coordinate(withNormalizedOffset: CGVector(dx: 0.94, dy: 0.80))
            let end = app.coordinate(withNormalizedOffset: CGVector(dx: 0.94, dy: 0.28))
            start.press(forDuration: 0.05, thenDragTo: end)
        }
        let result = try JSONEncoder().encode(samples)
        let directory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
        let destination = directory.appendingPathComponent("reply-observation-04.json")
        guard !FileManager.default.fileExists(atPath: destination.path) else {
            throw NSError(domain: "ReplyObservationEvidenceExists", code: 1)
        }
        try result.write(to: destination, options: .atomic)
        print("REPLY_OBSERVATION " + String(decoding: result, as: UTF8.self))
        // No owner app lifecycle or relay operation is performed on teardown.
    }
}