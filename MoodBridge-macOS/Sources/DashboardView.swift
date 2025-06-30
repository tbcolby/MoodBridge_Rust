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
                            .font(.title2.bold())\n                        \n                        VStack(spacing: 12) {\n                            QuickActionButton(\n                                title: \"New Project\",\n                                icon: \"plus.circle.fill\",\n                                color: .blue\n                            ) {\n                                // TODO: Show new project sheet\n                            }\n                            \n                            QuickActionButton(\n                                title: \"New Task\",\n                                icon: \"plus.square.fill\",\n                                color: .green\n                            ) {\n                                // TODO: Show new task sheet\n                            }\n                            \n                            QuickActionButton(\n                                title: \"Start Timer\",\n                                icon: \"play.circle.fill\",\n                                color: .orange\n                            ) {\n                                // TODO: Start work session\n                            }\n                        }\n                    }\n                    .frame(maxWidth: .infinity, alignment: .leading)\n                    .padding()\n                    .background(Color(.controlBackgroundColor))\n                    .cornerRadius(12)\n                }\n                .padding(.horizontal)\n                \n                // Active Work Sessions\n                if !viewModel.activeWorkSessions.isEmpty {\n                    VStack(alignment: .leading, spacing: 16) {\n                        Text(\"Active Work Sessions\")\n                            .font(.title2.bold())\n                        \n                        ForEach(viewModel.activeWorkSessions) { session in\n                            ActiveWorkSessionCard(session: session, viewModel: viewModel)\n                        }\n                    }\n                    .frame(maxWidth: .infinity, alignment: .leading)\n                    .padding(.horizontal)\n                }\n            }\n            .padding(.vertical)\n        }\n        .refreshable {\n            await viewModel.loadInitialData()\n        }\n    }\n}\n\nstruct StatCard: View {\n    let title: String\n    let value: String\n    let icon: String\n    let color: Color\n    \n    var body: some View {\n        VStack(spacing: 12) {\n            Image(systemName: icon)\n                .font(.system(size: 30))\n                .foregroundColor(color)\n            \n            VStack(spacing: 4) {\n                Text(value)\n                    .font(.title.bold())\n                Text(title)\n                    .font(.caption)\n                    .foregroundColor(.secondary)\n            }\n        }\n        .frame(maxWidth: .infinity)\n        .padding()\n        .background(Color(.controlBackgroundColor))\n        .cornerRadius(12)\n    }\n}\n\nstruct ActivityRow: View {\n    let activity: ActivityItem\n    \n    var body: some View {\n        HStack(spacing: 12) {\n            Circle()\n                .fill(Color.blue)\n                .frame(width: 8, height: 8)\n            \n            VStack(alignment: .leading, spacing: 2) {\n                Text(activity.description)\n                    .font(.system(size: 14))\n                Text(activity.timestamp)\n                    .font(.caption)\n                    .foregroundColor(.secondary)\n            }\n            \n            Spacer()\n        }\n        .padding(.vertical, 4)\n    }\n}\n\nstruct QuickActionButton: View {\n    let title: String\n    let icon: String\n    let color: Color\n    let action: () -> Void\n    \n    var body: some View {\n        Button(action: action) {\n            HStack {\n                Image(systemName: icon)\n                    .foregroundColor(color)\n                Text(title)\n                    .font(.system(size: 14, weight: .medium))\n                Spacer()\n            }\n            .padding()\n            .background(Color(.controlBackgroundColor))\n            .cornerRadius(8)\n        }\n        .buttonStyle(.plain)\n    }\n}\n\nstruct ActiveWorkSessionCard: View {\n    let session: WorkSession\n    @ObservedObject var viewModel: DashboardViewModel\n    \n    var body: some View {\n        HStack {\n            VStack(alignment: .leading, spacing: 4) {\n                Text(\"Task ID: \\(session.taskId)\")\n                    .font(.headline)\n                Text(\"Started: \\(session.startTime)\")\n                    .font(.caption)\n                    .foregroundColor(.secondary)\n            }\n            \n            Spacer()\n            \n            Button(\"End Session\") {\n                Task {\n                    await viewModel.endWorkSession(sessionId: session.id)\n                }\n            }\n            .buttonStyle(.borderedProminent)\n        }\n        .padding()\n        .background(Color(.controlBackgroundColor))\n        .cornerRadius(8)\n    }\n}
