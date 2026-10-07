import Foundation

/// Extract references from one server message, never from the whole session.
extension AgentSession.Message {
    var imagePaths: [String] {
        let texts = content.compactMap { part in
            part.type == "tool_result" ? part.content : (part.type == "text" ? part.text : nil)
        }
        return texts.flatMap(ImageReferencePaths.extract)
    }
}