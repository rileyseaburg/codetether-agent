import Foundation

/// Builds the agent prompt suffix that points the agent at uploaded attachment paths.
enum AttachmentPrompt {
    static func build(_ text: String, imagePaths: [String], documentPaths: [String]) throws -> String {
        var prompt = text
        if !imagePaths.isEmpty {
            let encoded = String(decoding: try JSONEncoder().encode(imagePaths), as: UTF8.self)
            prompt += "\n\nUser attached image files: \(encoded)\nUse the image tool to inspect these exact files before answering."
        }
        if !documentPaths.isEmpty {
            let encoded = String(decoding: try JSONEncoder().encode(documentPaths), as: UTF8.self)
            prompt += "\n\nUser attached PDF documents: \(encoded)\nRead these exact PDF files before answering."
        }
        return prompt
    }
}
