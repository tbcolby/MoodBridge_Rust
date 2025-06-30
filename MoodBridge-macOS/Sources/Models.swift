import Foundation

// MARK: - Health Response
struct HealthResponse: Codable {
    let status: String
    let timestamp: String
}

// MARK: - Dashboard Data
struct DashboardData: Codable {
    let totalProjects: Int
    let activeTasks: Int
    let completedTasks: Int
    let totalTimeSpent: Double
    let recentActivity: [ActivityItem]
    
    enum CodingKeys: String, CodingKey {
        case totalProjects = "total_projects"
        case activeTasks = "active_tasks"
        case completedTasks = "completed_tasks"
        case totalTimeSpent = "total_time_spent"
        case recentActivity = "recent_activity"
    }
}

struct ActivityItem: Codable, Identifiable {
    let id: String
    let type: String
    let description: String
    let timestamp: String
}

// MARK: - Project Models
struct Project: Codable, Identifiable {
    let id: String
    let name: String
    let description: String?
    let status: ProjectStatus
    let priority: Priority
    let createdAt: String
    let updatedAt: String
    
    enum CodingKeys: String, CodingKey {
        case id, name, description, status, priority
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }
}

struct CreateProjectRequest: Codable {
    let name: String
    let description: String?
    let priority: Priority
}

enum ProjectStatus: String, Codable, CaseIterable {
    case planning = "Planning"
    case active = "Active"
    case onHold = "OnHold"
    case completed = "Completed"
    case cancelled = "Cancelled"
}

enum Priority: String, Codable, CaseIterable {
    case low = "Low"
    case medium = "Medium"
    case high = "High"
    case urgent = "Urgent"
}

// MARK: - Task Models
struct MBTask: Codable, Identifiable {
    let id: String
    let projectId: String
    let title: String
    let description: String?
    let status: TaskStatus
    let priority: Priority
    let estimatedHours: Double?
    let actualHours: Double?
    let dueDate: String?
    let createdAt: String
    let updatedAt: String
    
    enum CodingKeys: String, CodingKey {
        case id, title, description, status, priority
        case projectId = "project_id"
        case estimatedHours = "estimated_hours"
        case actualHours = "actual_hours"
        case dueDate = "due_date"
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }
}

struct CreateTaskRequest: Codable {
    let projectId: String
    let title: String
    let description: String?
    let priority: Priority
    let estimatedHours: Double?
    let dueDate: String?
    
    enum CodingKeys: String, CodingKey {
        case title, description, priority
        case projectId = "project_id"
        case estimatedHours = "estimated_hours"
        case dueDate = "due_date"
    }
}

struct UpdateTaskRequest: Codable {
    let title: String?
    let description: String?
    let status: TaskStatus?
    let priority: Priority?
    let estimatedHours: Double?
    let dueDate: String?
    
    
    enum CodingKeys: String, CodingKey {
        case title, description, status, priority
        case estimatedHours = "estimated_hours"
        case dueDate = "due_date"
    }
}

enum TaskStatus: String, Codable, CaseIterable {
    case todo = "Todo"
    case inProgress = "InProgress"
    case review = "Review"
    case completed = "Completed"
    case cancelled = "Cancelled"
}

// MARK: - Work Session Models
struct WorkSession: Codable, Identifiable {
    let id: String
    let taskId: String
    let startTime: String
    let endTime: String?
    let duration: Double?
    let notes: String?
    
    enum CodingKeys: String, CodingKey {
        case id, notes, duration
        case taskId = "task_id"
        case startTime = "start_time"
        case endTime = "end_time"
    }
}

// MARK: - Analytics Models
struct TaskAnalytics: Codable {
    let totalTasks: Int
    let completedTasks: Int
    let activeTasks: Int
    let averageCompletionTime: Double
    let productivityTrend: [ProductivityPoint]
    
    enum CodingKeys: String, CodingKey {
        case totalTasks = "total_tasks"
        case completedTasks = "completed_tasks"
        case activeTasks = "active_tasks"
        case averageCompletionTime = "average_completion_time"
        case productivityTrend = "productivity_trend"
    }
}

struct ProductivityPoint: Codable, Identifiable {
    let id = UUID()
    let date: String
    let tasksCompleted: Int
    let hoursWorked: Double
    
    enum CodingKeys: String, CodingKey {
        case date
        case tasksCompleted = "tasks_completed"
        case hoursWorked = "hours_worked"
    }
}

struct ProjectDashboard: Codable {
    let projects: [ProjectSummary]
    let recentTasks: [MBTask]
    let activeWorkSessions: [WorkSession]
}

struct ProjectSummary: Codable, Identifiable {
    let id: String
    let name: String
    let status: ProjectStatus
    let taskCount: Int
    let completedTasks: Int
    let totalHours: Double
    
    enum CodingKeys: String, CodingKey {
        case id, name, status
        case taskCount = "task_count"
        case completedTasks = "completed_tasks"
        case totalHours = "total_hours"
    }
}
