import SwiftUI
import Combine

@MainActor
class DashboardViewModel: ObservableObject {
    @Published var isConnected = false
    @Published var isLoading = false
    @Published var errorMessage: String?
    
    // Dashboard Data
    @Published var dashboardData: DashboardData?
    
    // Projects
    @Published var projects: [Project] = []
    @Published var selectedProject: Project?
    
    // Tasks
    @Published var tasks: [MBTask] = []
    @Published var selectedTask: MBTask?
    
    // Work Sessions
    @Published var activeWorkSessions: [WorkSession] = []
    
    // Analytics
    @Published var taskAnalytics: TaskAnalytics?
    @Published var projectDashboard: ProjectDashboard?
    
    private let apiService = APIService.shared
    
    func loadInitialData() async {
        isLoading = true
        errorMessage = nil
        
        await checkConnection()
        
        if isConnected {
            await loadDashboardData()
            await loadProjects()
            await loadTasks()
            await loadAnalytics()
        }
        
        isLoading = false
    }
    
    func checkConnection() async {
        do {
            let health = try await apiService.healthCheck()
            isConnected = health.status.lowercased() == "ok" || health.status.lowercased() == "healthy"
            errorMessage = nil
        } catch {
            isConnected = false
            errorMessage = "Failed to connect to MoodBridge API: \(error.localizedDescription)"
        }
    }
    
    func loadDashboardData() async {
        do {
            dashboardData = try await apiService.getDashboardData()
        } catch {
            errorMessage = "Failed to load dashboard: \(error.localizedDescription)"
        }
    }
    
    func loadProjects() async {
        do {
            projects = try await apiService.getProjects()
        } catch {
            errorMessage = "Failed to load projects: \(error.localizedDescription)"
        }
    }
    
    func loadTasks() async {
        do {
            tasks = try await apiService.getTasks()
        } catch {
            errorMessage = "Failed to load tasks: \(error.localizedDescription)"
        }
    }
    
    func loadAnalytics() async {
        do {
            taskAnalytics = try await apiService.getTaskAnalytics()
            projectDashboard = try await apiService.getProjectDashboard()
        } catch {
            errorMessage = "Failed to load analytics: \(error.localizedDescription)"
        }
    }
    
    // MARK: - Project Actions
    func createProject(name: String, description: String?, priority: Priority) async {
        do {
            let request = CreateProjectRequest(name: name, description: description, priority: priority)
            let newProject = try await apiService.createProject(request)
            projects.append(newProject)
        } catch {
            errorMessage = "Failed to create project: \(error.localizedDescription)"
        }
    }
    
    // MARK: - Task Actions
    func createTask(projectId: String, title: String, description: String?, priority: Priority, estimatedHours: Double?, dueDate: String?) async {
        do {
            let request = CreateTaskRequest(
                projectId: projectId,
                title: title,
                description: description,
                priority: priority,
                estimatedHours: estimatedHours,
                dueDate: dueDate
            )
            let newTask = try await apiService.createTask(request)
            tasks.append(newTask)
        } catch {
            errorMessage = "Failed to create task: \(error.localizedDescription)"
        }
    }
    
    func updateTaskStatus(taskId: String, status: TaskStatus) async {
        do {
            let request = UpdateTaskRequest(
                title: nil,
                description: nil,
                status: status,
                priority: nil,
                estimatedHours: nil,
                dueDate: nil
            )
            let updatedTask = try await apiService.updateTask(id: taskId, request)
            
            if let index = tasks.firstIndex(where: { $0.id == taskId }) {
                tasks[index] = updatedTask
            }
        } catch {
            errorMessage = "Failed to update task: \(error.localizedDescription)"
        }
    }
    
    // MARK: - Work Session Actions
    func startWorkSession(taskId: String) async {
        do {
            let session = try await apiService.startWorkSession(taskId: taskId)
            activeWorkSessions.append(session)
        } catch {
            errorMessage = "Failed to start work session: \(error.localizedDescription)"
        }
    }
    
    func endWorkSession(sessionId: String) async {
        do {
            _ = try await apiService.endWorkSession(sessionId: sessionId)
            activeWorkSessions.removeAll { $0.id == sessionId }
        } catch {
            errorMessage = "Failed to end work session: \(error.localizedDescription)"
        }
    }
    
    // MARK: - Utility Methods
    func formatDuration(_ seconds: Double) -> String {
        let hours = Int(seconds) / 3600
        let minutes = (Int(seconds) % 3600) / 60
        
        if hours > 0 {
            return "\(hours)h \(minutes)m"
        } else {
            return "\(minutes)m"
        }
    }
    
    func priorityColor(for priority: Priority) -> Color {
        switch priority {
        case .low:
            return .green
        case .medium:
            return .orange
        case .high:
            return .red
        case .urgent:
            return .purple
        }
    }
    
    func statusColor(for status: TaskStatus) -> Color {
        switch status {
        case .todo:
            return .gray
        case .inProgress:
            return .blue
        case .review:
            return .orange
        case .completed:
            return .green
        case .cancelled:
            return .red
        }
    }
}
