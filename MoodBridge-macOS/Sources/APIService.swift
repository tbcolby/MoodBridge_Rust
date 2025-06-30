import Foundation
import Alamofire

class APIService: ObservableObject {
    static let shared = APIService()
    private let baseURL = "http://127.0.0.1:8000"
    
    private init() {}
    
    // MARK: - Health Check
    func healthCheck() async throws -> HealthResponse {
        return try await AF.request("\(baseURL)/api/health")
            .validate()
            .serializingDecodable(HealthResponse.self)
            .value
    }
    
    // MARK: - Dashboard Data
    func getDashboardData() async throws -> DashboardData {
        return try await AF.request("\(baseURL)/api/dashboard-data")
            .validate()
            .serializingDecodable(DashboardData.self)
            .value
    }
    
    // MARK: - Projects
    func getProjects() async throws -> [Project] {
        return try await AF.request("\(baseURL)/api/projects")
            .validate()
            .serializingDecodable([Project].self)
            .value
    }
    
    func createProject(_ project: CreateProjectRequest) async throws -> Project {
        return try await AF.request("\(baseURL)/api/projects",
                                   method: .post,
                                   parameters: project,
                                   encoder: JSONParameterEncoder.default)
            .validate()
            .serializingDecodable(Project.self)
            .value
    }
    
    func getProject(id: String) async throws -> Project {
        return try await AF.request("\(baseURL)/api/projects/\(id)")
            .validate()
            .serializingDecodable(Project.self)
            .value
    }
    
    // MARK: - Tasks
    func getTasks() async throws -> [MBTask] {
        return try await AF.request("\(baseURL)/api/tasks")
            .validate()
            .serializingDecodable([MBTask].self)
            .value
    }
    
    func createTask(_ task: CreateTaskRequest) async throws -> MBTask {
        return try await AF.request("\(baseURL)/api/tasks",
                                   method: .post,
                                   parameters: task,
                                   encoder: JSONParameterEncoder.default)
            .validate()
            .serializingDecodable(MBTask.self)
            .value
    }
    
    func updateTask(id: String, _ task: UpdateTaskRequest) async throws -> MBTask {
        return try await AF.request("\(baseURL)/api/tasks/\(id)",
                                   method: .put,
                                   parameters: task,
                                   encoder: JSONParameterEncoder.default)
            .validate()
            .serializingDecodable(MBTask.self)
            .value
    }
    
    // MARK: - Work Sessions
    func startWorkSession(taskId: String) async throws -> WorkSession {
        return try await AF.request("\(baseURL)/api/work-sessions/\(taskId)/start",
                                   method: .post)
            .validate()
            .serializingDecodable(WorkSession.self)
            .value
    }
    
    func endWorkSession(sessionId: String) async throws -> WorkSession {
        return try await AF.request("\(baseURL)/api/work-sessions/\(sessionId)/end",
                                   method: .put)
            .validate()
            .serializingDecodable(WorkSession.self)
            .value
    }
    
    // MARK: - Analytics
    func getTaskAnalytics() async throws -> TaskAnalytics {
        return try await AF.request("\(baseURL)/api/task-analytics")
            .validate()
            .serializingDecodable(TaskAnalytics.self)
            .value
    }
    
    func getProjectDashboard() async throws -> ProjectDashboard {
        return try await AF.request("\(baseURL)/api/project-dashboard")
            .validate()
            .serializingDecodable(ProjectDashboard.self)
            .value
    }
}
