import Foundation

@MainActor
final class ConnectionModel: ObservableObject {
    @Published var version: ServerVersion?
    @Published var agents: [AgentProfile] = []
    @Published var busy = false
    @Published var error: String?
    @Published var checkedAt: Date?
    private let client = ServerClient()

    func start() async {
        do { try TokenBootstrap.consume() }
        catch { self.error = "Secure token setup failed. Use Settings to enter it again."; return }
        await refresh()
    }

    func refresh() async {
        guard !busy else { return }
        busy = true
        error = nil
        version = nil
        agents = []
        checkedAt = nil
        defer { busy = false }
        do {
            guard let token = try TokenStore.read() else { throw ClientError.missingToken }
            let info: ServerVersion = try await client.get("api/version", token: token)
            let catalog: [AgentProfile] = try await client.get("api/agent", token: token)
            version = info
            agents = catalog.filter { !$0.hidden }
            checkedAt = Date()
            VerificationReceipt.write(version: info.version, agentCount: agents.count)
        } catch let failure as ClientError { error = failure.localizedDescription }
        catch { self.error = "Cannot reach the server. Check your connection and try again." }
    }

    func disconnect() throws {
        try TokenStore.remove()
        version = nil
        agents = []
        checkedAt = nil
        error = nil
    }
}
