import Foundation

extension AgentSession {
    var imagePaths: [String] {
        let texts = (messages ?? []).flatMap(\.content).compactMap { part in
            part.type == "tool_result" ? part.content : (part.type == "text" ? part.text : nil)
        }
        return Array(Set(texts.flatMap(ImageReferencePaths.extract))).sorted()
    }
}
