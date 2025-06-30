import SwiftUI

struct AnalyticsView: View {
    @ObservedObject var viewModel: DashboardViewModel
    
    var body: some View {
        ScrollView {
            LazyVStack(spacing: 20) {
                // Header
                VStack(alignment: .leading, spacing: 8) {
                    Text("Analytics")
                        .font(.largeTitle.bold())
                    Text("Productivity insights and metrics")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal)
                
                // Task Analytics Overview
                if let analytics = viewModel.taskAnalytics {
                    LazyVGrid(columns: Array(repeating: GridItem(.flexible()), count: 3), spacing: 16) {
                        AnalyticsCard(
                            title: "Total Tasks",
                            value: "\(analytics.totalTasks)",
                            icon: "list.bullet",
                            color: .blue,
                            trend: nil
                        )
                        
                        AnalyticsCard(
                            title: "Completion Rate",
                            value: "\(Int((Double(analytics.completedTasks) / Double(max(1, analytics.totalTasks))) * 100))%",
                            icon: "chart.pie.fill",
                            color: .green,
                            trend: nil
                        )
                        
                        AnalyticsCard(
                            title: "Avg. Completion Time",
                            value: viewModel.formatDuration(analytics.averageCompletionTime),
                            icon: "clock.fill",
                            color: .orange,
                            trend: nil
                        )
                    }
                    .padding(.horizontal)
                    
                    // Productivity Trend Chart
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Productivity Trend")
                            .font(.title2.bold())
                        
                        ProductivityChart(data: analytics.productivityTrend)
                    }
                    .padding()
                    .background(Color(.controlBackgroundColor))
                    .cornerRadius(12)
                    .padding(.horizontal)
                }
                
                // Project Dashboard
                if let projectDashboard = viewModel.projectDashboard {
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Project Overview")
                            .font(.title2.bold())
                        
                        LazyVGrid(columns: [
                            GridItem(.flexible()),
                            GridItem(.flexible())
                        ], spacing: 12) {
                            ForEach(projectDashboard.projects) { project in
                                ProjectAnalyticsCard(project: project, viewModel: viewModel)
                            }
                        }
                    }
                    .padding()
                    .background(Color(.controlBackgroundColor))
                    .cornerRadius(12)
                    .padding(.horizontal)
                    
                    // Recent Tasks
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Recent Task Activity")
                            .font(.title2.bold())
                        
                        ForEach(projectDashboard.recentTasks.prefix(5)) { task in
                            RecentTaskRow(task: task, viewModel: viewModel)
                        }
                    }
                    .padding()
                    .background(Color(.controlBackgroundColor))
                    .cornerRadius(12)
                    .padding(.horizontal)
                }
            }
            .padding(.vertical)
        }
        .refreshable {
            await viewModel.loadAnalytics()
        }
        .task {
            await viewModel.loadAnalytics()
        }
    }
}

struct AnalyticsCard: View {
    let title: String
    let value: String
    let icon: String
    let color: Color
    let trend: String?
    
    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 30))
                .foregroundColor(color)
            
            VStack(spacing: 4) {
                Text(value)
                    .font(.title.bold())
                Text(title)
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                
                if let trend = trend {
                    Text(trend)
                        .font(.caption2)
                        .foregroundColor(.green)
                }
            }
        }
        .frame(maxWidth: .infinity)
        .padding()
        .background(Color(.controlBackgroundColor))
        .cornerRadius(12)
    }
}

struct ProductivityChart: View {
    let data: [ProductivityPoint]
    
    var body: some View {
        VStack(alignment: .leading) {
            if data.isEmpty {
                Text("No productivity data available")
                    .foregroundColor(.secondary)
                    .frame(height: 150)
            } else {
                // Simple bar chart representation
                HStack(alignment: .bottom, spacing: 4) {
                    ForEach(data.suffix(7)) { point in
                        VStack(spacing: 4) {
                            Rectangle()
                                .fill(LinearGradient(
                                    colors: [.blue, .purple],
                                    startPoint: .bottom,
                                    endPoint: .top
                                ))
                                .frame(width: 30, height: CGFloat(point.hoursWorked * 10))
                                .cornerRadius(4)
                            
                            Text("\(point.tasksCompleted)")
                                .font(.caption2)
                                .foregroundColor(.secondary)
                        }
                    }
                }
                .frame(height: 150)
                
                Text("Tasks completed per day (last 7 days)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
}

struct ProjectAnalyticsCard: View {
    let project: ProjectSummary
    @ObservedObject var viewModel: DashboardViewModel
    
    var completionRate: Double {
        guard project.taskCount > 0 else { return 0 }
        return Double(project.completedTasks) / Double(project.taskCount)
    }
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text(project.name)
                    .font(.headline)
                    .lineLimit(1)
                
                Spacer()
                
                Text(project.status.rawValue)
                    .font(.caption)
                    .padding(.horizontal, 6)
                    .padding(.vertical, 2)
                    .background(statusColor.opacity(0.2))
                    .foregroundColor(statusColor)
                    .cornerRadius(4)
            }
            
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text("Progress")
                        .font(.caption.bold())
                    Spacer()
                    Text("\(project.completedTasks)/\(project.taskCount)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                
                ProgressView(value: completionRate)
                    .progressViewStyle(LinearProgressViewStyle(tint: .blue))
            }
            
            HStack {
                Label("\(project.totalHours, specifier: "%.1f")h", systemImage: "clock.fill")
                    .font(.caption)
                    .foregroundColor(.secondary)
                
                Spacer()
            }
        }
        .padding()
        .background(Color(.controlBackgroundColor))
        .cornerRadius(8)
    }
    
    private var statusColor: Color {
        switch project.status {
        case .planning:
            return .orange
        case .active:
            return .blue
        case .onHold:
            return .yellow
        case .completed:
            return .green
        case .cancelled:
            return .red
        }
    }
}

struct RecentTaskRow: View {
    let task: MBTask
    @ObservedObject var viewModel: DashboardViewModel
    
    var body: some View {
        HStack(spacing: 12) {
            Circle()
                .fill(viewModel.statusColor(for: task.status))
                .frame(width: 8, height: 8)
            
            VStack(alignment: .leading, spacing: 2) {
                Text(task.title)
                    .font(.system(size: 14, weight: .medium))
                    .strikethrough(task.status == .completed)
                
                Text(task.status.rawValue)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            Text(task.priority.rawValue)
                .font(.caption)
                .padding(.horizontal, 6)
                .padding(.vertical, 2)
                .background(viewModel.priorityColor(for: task.priority).opacity(0.2))
                .foregroundColor(viewModel.priorityColor(for: task.priority))
                .cornerRadius(4)
        }
        .padding(.vertical, 4)
    }
}
