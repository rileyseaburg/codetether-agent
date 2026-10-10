import XCTest

extension ManualBridge {
    func perform(_ action: String) -> String {
        guard app.state == .runningForeground || app.state == .runningBackground else {
            return "Owner app is not running; refusing to launch"
        }
        if action == "inspect" { return "Observed only" }
        if action == "screen" {
            app.activate()
            let tab = app.tabBars.buttons["Screen"]
            guard tab.exists else { return "Screen tab absent" }
            tab.tap(); return "Opened existing Screen tab"
        }
        guard app.state == .runningForeground else { return "Owner app is not foreground" }
        if action == "up" { app.swipeUp(); return "Scrolled down page" }
        if action == "down" { app.swipeDown(); return "Scrolled up page" }
        let question = app.descendants(matching: .any).matching(identifier: "screen-question").firstMatch
        if action == "prepare" {
            guard !prepared, !submitted, question.exists, question.isHittable else { return "Cannot prepare" }
            let draft = question.value as? String ?? ""
            guard draft.isEmpty || draft == "What should AI check on screen now?" else {
                return "Existing draft preserved"
            }
            prepared = true
            question.tap()
            question.typeText("Type exactly CT-LIVE-052914 into the already focused Notepad document only. Do not click, press Enter, or submit. If the focused field is not Notepad, do not type.")
            return "Prepared harmless marker request; not sent"
        }
        if action == "send" {
            let ask = app.buttons["screen-ask"]
            guard prepared, !submitted, ask.exists, ask.isHittable, ask.isEnabled else { return "Cannot send" }
            submitted = true
            ask.tap()
            return "Sent exactly one owner request"
        }
        return "Unsupported action"
    }
}
