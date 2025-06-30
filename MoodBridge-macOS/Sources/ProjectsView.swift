import SwiftUI

struct ProjectsView: View {
    @ObservedObject var viewModel: DashboardViewModel
    @State private var showingNewProject = false
    
    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Projects")
                        .font(.largeTitle.bold())
                    Text("Manage your legal projects")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                
                Spacer()
                
                Button("New Project") {
                    showingNewProject = true
                }
                .buttonStyle(.borderedProminent)
            }
            .padding(.horizontal)
            
            // Projects List
            ScrollView {
                LazyVGrid(columns: [
                    GridItem(.flexible()),
                    GridItem(.flexible())
                ], spacing: 16) {
                    ForEach(viewModel.projects) { project in
                        ProjectCard(project: project, viewModel: viewModel)
                    }
                }
                .padding(.horizontal)
            }
        }
        .sheet(isPresented: $showingNewProject) {
            NewProjectSheet(viewModel: viewModel)
        }
        .task {
            await viewModel.loadProjects()
        }
    }
}

struct ProjectCard: View {
    let project: Project
    @ObservedObject var viewModel: DashboardViewModel
    
    private var projectTasks: [MBTask] {
        viewModel.tasks.filter { $0.projectId == project.id }
    }
    
    private var completedTasks: Int {
        projectTasks.filter { $0.status == .completed }.count
    }
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text(project.name)
                        .font(.headline)
                        .lineLimit(2)
                    
                    if let description = project.description {
                        Text(description)
                            .font(.caption)
                            .foregroundColor(.secondary)
                            .lineLimit(3)
                    }
                }
                
                Spacer()
                
                // Priority badge
                Text(project.priority.rawValue)
                    .font(.caption.bold())
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(viewModel.priorityColor(for: project.priority).opacity(0.2))
                    .foregroundColor(viewModel.priorityColor(for: project.priority))
                    .cornerRadius(4)
            }
            
            // Progress
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    Text("Progress")
                        .font(.caption.bold())
                    Spacer()
                    Text("\(completedTasks)/\(projectTasks.count)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                
                ProgressView(value: Double(completedTasks), total: Double(max(1, projectTasks.count)))
                    .progressViewStyle(LinearProgressViewStyle(tint: .blue))
            }
            
            // Status and actions
            HStack {
                Text(project.status.rawValue)
                    .font(.caption)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(statusColor(for: project.status).opacity(0.2))
                    .foregroundColor(statusColor(for: project.status))
                    .cornerRadius(4)
                
                Spacer()
                
                Button("View Tasks") {
                    viewModel.selectedProject = project
                }
                .font(.caption)
                .buttonStyle(.borderless)
            }
        }
        .padding()
        .background(Color(.controlBackgroundColor))
        .cornerRadius(12)
    }
    
    private func statusColor(for status: ProjectStatus) -> Color {
        switch status {
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

struct NewProjectSheet: View {
    @ObservedObject var viewModel: DashboardViewModel
    @Environment(\.dismiss) private var dismiss
    
    @State private var name = ""
    @State private var description = ""
    @State private var priority = Priority.medium
    
    var body: some View {
        NavigationView {
            Form {
                Section("Project Details") {
                    TextField("Project Name", text: $name)
                    TextField("Description", text: $description, axis: .vertical)
                        .lineLimit(3...6)
                }
                
                Section("Priority") {
                    Picker("Priority", selection: $priority) {
                        ForEach(Priority.allCases, id: \.self) { priority in
                            Text(priority.rawValue).tag(priority)
                        }
                    }
                    .pickerStyle(.segmented)
                }
            }
            .navigationTitle("New Project")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                
                ToolbarItem(placement: .confirmationAction) {
                    Button("Create") {
                        Task {
                            await viewModel.createProject(
                                name: name,
                                description: description.isEmpty ? nil : description,
                                priority: priority
                            )
                            dismiss()
                        }
                    }
                    .disabled(name.isEmpty)
                }
            }
        }
        .frame(width: 500, height: 400)
    }
}
