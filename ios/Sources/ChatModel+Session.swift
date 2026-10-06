import Foundation

extension ChatModel {
    private struct CreateSession: Encodable { let title: String; let agent = "build" }
    func runAgentTurn(_ text: String) async throws -> AgentReply {
        guard let token = try TokenStore.read() else { throw ClientError.missingToken }
        if sessionID == nil {
            let session: AgentSession = try await client.post("api/session", token: token, body: CreateSession(title: "iPhone: " + text.prefix(60)))
            sessionID = session.id
            UserDefaults.standard.set(session.id, forKey: "agent.session")
        }
        var paths: [String] = []
        for index in attachments.indices {
            if attachments[index].serverPath == nil {
                let uploaded: UploadedImage = try await client.post("mobile/attachments", token: token,
                    body: ImageUpload(data: attachments[index].data.base64EncodedString()))
                attachments[index].serverPath = uploaded.path
            }
            if let path = attachments[index].serverPath { paths.append(path) }
        }
        var prompt = text
        if !paths.isEmpty {
            let encoded = String(decoding: try JSONEncoder().encode(paths), as: UTF8.self)
            prompt += "\n\nUser attached image files: \(encoded)\nUse the image tool to inspect these exact files before answering."
        }
        agentStatus = "Agent working…"
        let reply = try await agent.prompt(sessionID: sessionID!, message: prompt) { self.receive($0) }
        if let snapshot: AgentSession = try? await client.get("api/session/\(reply.session_id)", token: token) {
            tools = snapshot.toolNames
            generatedImages = snapshot.imagePaths
        }
        agentStatus = "Agent finished"
        AgentReceipt.write(session: reply.session_id, tools: tools)
        return reply
    }
    func restoreSession() async {
        guard let id = sessionID else { return }
        do {
            guard let token = try TokenStore.read() else { throw ClientError.missingToken }
            let snapshot: AgentSession = try await client.get("api/session/\(id)", token: token)
            messages = snapshot.transcript
            tools = snapshot.toolNames
            generatedImages = snapshot.imagePaths
        } catch { self.error = "Could not restore the conversation. Use New chat to start again." }
    }
}
