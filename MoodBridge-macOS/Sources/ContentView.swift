import SwiftUI

struct ContentView: View {
    @StateObject private var viewModel = DashboardViewModel()
    @State private var selectedTab = 0
    
    var body: some View {
        NavigationSplitView {
            // Sidebar
            VStack(alignment: .leading, spacing: 20) {
                // Logo and Branding
                VStack {
                    Image(systemName: "brain.head.profile")
                        .font(.system(size: 50))
                        .foregroundStyle(
                            LinearGradient(
                                colors: [.blue, .purple, .pink],
                                startPoint: .topLeading,
                                endPoint: .bottomTrailing
                            )
                        )
                    
                    Text("MoodBridge")
                        .font(.title.bold())
                        .foregroundStyle(
                            LinearGradient(
                                colors: [.blue, .purple],
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                    
                    Text("Legal Tech Dashboard")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .padding(.bottom, 30)
                
                // Navigation Items
                VStack(alignment: .leading, spacing: 12) {
                    SidebarButton(
                        title: "Dashboard",
                        icon: "chart.bar.fill",
                        isSelected: selectedTab == 0
                    ) {
                        selectedTab = 0
                    }
                    
                    SidebarButton(
                        title: "Projects",
                        icon: "folder.fill",
                        isSelected: selectedTab == 1
                    ) {
                        selectedTab = 1
                    }
                    
                    SidebarButton(
                        title: "Tasks",
                        icon: "checkmark.circle.fill",
                        isSelected: selectedTab == 2
                    ) {
                        selectedTab = 2
                    }
                    
                    SidebarButton(
                        title: "Analytics",
                        icon: "chart.line.uptrend.xyaxis",
                        isSelected: selectedTab == 3
                    ) {
                        selectedTab = 3
                    }
                }
                
                Spacer()
                
                // Connection Status
                ConnectionStatusView(viewModel: viewModel)
            }
            .padding(20)
            .frame(minWidth: 250)
            .background(Color(.controlBackgroundColor))
        } detail: {
            // Main Content
            Group {
                switch selectedTab {
                case 0:
                    DashboardView(viewModel: viewModel)
                case 1:
                    ProjectsView(viewModel: viewModel)
                case 2:
                    TasksView(viewModel: viewModel)
                case 3:
                    AnalyticsView(viewModel: viewModel)
                default:
                    DashboardView(viewModel: viewModel)
                }
            }
            .frame(minWidth: 600, minHeight: 400)
        }
        .onAppear {
            Task {
                await viewModel.loadInitialData()
            }
        }
    }
}

struct SidebarButton: View {
    let title: String
    let icon: String
    let isSelected: Bool
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: icon)
                    .frame(width: 20)
                Text(title)
                    .font(.system(size: 14, weight: .medium))
                Spacer()
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 8)
            .background(
                RoundedRectangle(cornerRadius: 8)
                    .fill(isSelected ? Color.blue.opacity(0.2) : Color.clear)
            )
            .foregroundColor(isSelected ? .blue : .primary)
        }
        .buttonStyle(.plain)
    }
}

struct ConnectionStatusView: View {
    @ObservedObject var viewModel: DashboardViewModel
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Circle()
                    .fill(viewModel.isConnected ? .green : .red)
                    .frame(width: 8, height: 8)
                Text(viewModel.isConnected ? "Connected" : "Disconnected")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            if !viewModel.isConnected {
                Button("Reconnect") {
                    Task {
                        await viewModel.checkConnection()
                    }
                }
                .font(.caption)
                .buttonStyle(.borderless)
            }
        }
        .padding(12)
        .background(Color(.controlBackgroundColor))
        .cornerRadius(8)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
            .frame(width: 1000, height: 700)
    }
}
