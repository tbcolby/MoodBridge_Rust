import SwiftUI

struct DashboardView: View {
    @ObservedObject var viewModel: DashboardViewModel
    
    var body: some View {
        ScrollView {
            LazyVStack(spacing: 20) {
                // Header
                VStack(alignment: .leading, spacing: 8) {
                    Text("Dashboard")
                        .font(.largeTitle.bold())
                    Text("Welcome to MoodBridge Legal Tech Dashboard")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal)
                
                // Quick Stats
                if let data = viewModel.dashboardData {
                    LazyVGrid(columns: Array(repeating: GridItem(.flexible()), count: 4), spacing: 16) {
                        StatCard(
                            title: "Total Projects",
                            value: "\(data.totalProjects)",
                            icon: "folder.fill",
                            color: .blue
                        )
                        
                        StatCard(
                            title: "Active Tasks",
                            value: "\(data.activeTasks)",
                            icon: "checkmark.circle.fill",
                            color: .orange
                        )
                        
                        StatCard(
                            title: "Completed Tasks",
                            value: "\(data.completedTasks)",
                            icon: "checkmark.circle.fill",
                            color: .green
                        )
                        
                        StatCard(
                            title: "Total Time",
                            value: viewModel.formatDuration(data.totalTimeSpent * 3600),
                            icon: "clock.fill",
                            color: .purple
                        )
                    }
                    .padding(.horizontal)
                }
                
                // Recent Activity & Quick Actions
                HStack(alignment: .top, spacing: 20) {
                    // Recent Activity
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Recent Activity")
                            .font(.title2.bold())
                        
                        if let data = viewModel.dashboardData {
                            ForEach(data.recentActivity.prefix(5)) { activity in
                                ActivityRow(activity: activity)
                            }
                        } else {
                            Text("Loading activity...")
                                .foregroundColor(.secondary)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
                    .background(Color(.controlBackgroundColor))
                    .cornerRadius(12)
                    
                    // Quick Actions
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Quick Actions")
                            .font(.title2.bold())
                        
                        VStack(spacing: 12) {
                            QuickActionButton(
                                title: "New Project",
                                icon: "plus.circle.fill",
                                color: .blue
                            ) {
                                // TODO: Show new project sheet
                            }
                            
                            QuickActionButton(
                                title: "New Task",
                                icon: "plus.square.fill",
                                color: .green
                            ) {
                                // TODO: Show new task sheet
                            }
                            
                            QuickActionButton(
                                title: "Start Timer",
                                icon: "play.circle.fill",
                                color: .orange
                            ) {
                                // TODO: Start work session
                            }
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
                    .background(Color(.controlBackgroundColor))
                    .cornerRadius(12)
                }
                .padding(.horizontal)
                
                // Active Work Sessions
                if !viewModel.activeWorkSessions.isEmpty {
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Active Work Sessions")
                            .font(.title2.bold())
                        
                        ForEach(viewModel.activeWorkSessions) { session in
                            ActiveWorkSessionCard(session: session, viewModel: viewModel)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.horizontal)
                }
            }
            .padding(.vertical)
        }
        .refreshable {
            await viewModel.loadInitialData()
        }
    }
}

struct StatCard: View {
    let title: String
    let value: String
    let icon: String
    let color: Color
    
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
            }
        }
        .frame(maxWidth: .infinity)
        .padding()
        .background(Color(.controlBackgroundColor))
        .cornerRadius(12)
    }
}

struct ActivityRow: View {
    let activity: ActivityItem
    
    var body: some View {
        HStack(spacing: 12) {
            Circle()
                .fill(Color.blue)
                .frame(width: 8, height: 8)
            
            VStack(alignment: .leading, spacing: 2) {
                Text(activity.description)
                    .font(.system(size: 14))
                Text(activity.timestamp)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
        }
        .padding(.vertical, 4)
    }
}

struct QuickActionButton: View {
    let title: String
    let icon: String
    let color: Color
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: icon)
                    .foregroundColor(color)
                Text(title)
                    .font(.system(size: 14, weight: .medium))
                Spacer()
            }
            .padding()
            .background(Color(.controlBackgroundColor))
            .cornerRadius(8)
        }
        .buttonStyle(.plain)
    }
}

struct ActiveWorkSessionCard: View {
    let session: WorkSession
    @ObservedObject var viewModel: DashboardViewModel
    
    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text("Task ID: \(session.taskId)")
                    .font(.headline)
                Text("Started: \(session.startTime)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            Button("End Session") {
                Task {
                    await viewModel.endWorkSession(sessionId: session.id)
                }
            }
            .buttonStyle(.borderedProminent)
        }
        .padding()
        .background(Color(.controlBackgroundColor))
        .cornerRadius(8)
    }
}
