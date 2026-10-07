import Foundation

/// Loads PDFs chosen in the document picker into the chat as attachments.
enum DocumentImport {
    @MainActor
    static func handle(_ result: Result<[URL], any Error>, chat: ChatModel) {
        guard case .success(let urls) = result else {
            chat.error = "That PDF could not be attached. Try a smaller file."
            return
        }
        for url in urls.prefix(max(0, UserImage.maximumAttachments - chat.attachments.count)) {
            guard url.startAccessingSecurityScopedResource() else { continue }
            defer { url.stopAccessingSecurityScopedResource() }
            do { chat.attachDocument(try Data(contentsOf: url)) }
            catch { chat.error = "That PDF could not be attached. Try a smaller file." }
        }
    }
}
