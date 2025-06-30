import SwiftUI

struct TasksView: View {
    @ObservedObject var viewModel: DashboardViewModel
    @State private var showingNewTask = false
    @State private var selectedFilter = TaskFilter.all
    
    enum TaskFilter: String, CaseIterable {
        case all = "All"
        case todo = "To Do"
        case inProgress = "In Progress"
        case review = "Review"
        case completed = "Completed"
    }
    
    private var filteredTasks: [MBTask] {
        switch selectedFilter {
        case .all:
            return viewModel.tasks
        case .todo:
            return viewModel.tasks.filter { $0.status == .todo }
        case .inProgress:
            return viewModel.tasks.filter { $0.status == .inProgress }
        case .review:
            return viewModel.tasks.filter { $0.status == .review }
        case .completed:
            return viewModel.tasks.filter { $0.status == .completed }
        }
    }
    
    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Tasks")
                        .font(.largeTitle.bold())
                    Text("Manage your legal tasks")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                
                Spacer()
                
                Button("New Task") {
                    showingNewTask = true
                }
                .buttonStyle(.borderedProminent)
            }
            .padding(.horizontal)
            
            // Filters
            HStack {
                Picker("Filter", selection: $selectedFilter) {
                    ForEach(TaskFilter.allCases, id: \.self) { filter in
                        Text(filter.rawValue).tag(filter)
                    }
                }
                .pickerStyle(.segmented)
                
                Spacer()
            }
            .padding(.horizontal)
            
            // Tasks List
            ScrollView {
                LazyVStack(spacing: 12) {
                    ForEach(filteredTasks) { task in
                        TaskCard(task: task, viewModel: viewModel)
                    }
                }
                .padding(.horizontal)
            }
        }
        .sheet(isPresented: $showingNewTask) {
            NewTaskSheet(viewModel: viewModel)
        }
        .task {
            await viewModel.loadTasks()
        }
    }
}

struct TaskCard: View {
    let task: MBTask
    @ObservedObject var viewModel: DashboardViewModel
    
    private var project: Project? {
        viewModel.projects.first { $0.id == task.projectId }
    }
    
    var body: some View {
        HStack(spacing: 16) {
            // Status indicator
            Circle()
                .fill(viewModel.statusColor(for: task.status))
                .frame(width: 12, height: 12)
            
            // Task content
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    Text(task.title)
                        .font(.headline)
                        .strikethrough(task.status == .completed)
                    
                    Spacer()
                    
                    // Priority badge
                    Text(task.priority.rawValue)
                        .font(.caption.bold())
                        .padding(.horizontal, 8)
                        .padding(.vertical, 4)
                        .background(viewModel.priorityColor(for: task.priority).opacity(0.2))
                        .foregroundColor(viewModel.priorityColor(for: task.priority))
                        .cornerRadius(4)
                }
                
                if let description = task.description {
                    Text(description)
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .lineLimit(2)
                }
                
                // Project and timing info
                HStack {
                    if let project = project {
                        Label(project.name, systemImage: "folder.fill")
                            .font(.caption)
                            .foregroundColor(.blue)
                    }
                    
                    Spacer()
                    
                    if let estimatedHours = task.estimatedHours {
                        Label("\(estimatedHours, specifier: "%.1f")h est.", systemImage: "clock")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    
                    if let actualHours = task.actualHours {
                        Label("\(actualHours, specifier: "%.1f")h actual", systemImage: "clock.fill")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            }
            
            // Actions
            VStack(spacing: 8) {
                // Status picker
                Menu(task.status.rawValue) {
                    ForEach(TaskStatus.allCases, id: \.self) { status in
                        Button(status.rawValue) {
                            Task {
                                await viewModel.updateTaskStatus(taskId: task.id, status: status)
                            }
                        }
                    }
                }
                .menuStyle(.borderlessButton)
                .font(.caption)
                
                // Start work session
                if task.status == .inProgress {
                    Button("Start Timer") {
                        Task {
                            await viewModel.startWorkSession(taskId: task.id)
                        }
                    }
                    .font(.caption)
                    .buttonStyle(.borderless)
                }
            }
        }
        .padding()
        .background(Color(.controlBackgroundColor))
        .cornerRadius(8)
    }
}

struct NewTaskSheet: View {
    @ObservedObject var viewModel: DashboardViewModel
    @Environment(\.dismiss) private var dismiss
    
    @State private var title = ""
    @State private var description = ""
    @State private var selectedProjectId = ""
    @State private var priority = Priority.medium
    @State private var estimatedHours: Double = 1.0
    @State private var dueDate = Date()
    @State private var hasDueDate = false
    
    var body: some View {
        NavigationView {
            Form {
                Section("Task Details") {
                    TextField("Task Title", text: $title)
                    TextField("Description", text: $description, axis: .vertical)
                        .lineLimit(3...6)
                }
                
                Section("Project") {
                    Picker("Project", selection: $selectedProjectId) {
                        Text("Select Project").tag("")
                        ForEach(viewModel.projects) { project in
                            Text(project.name).tag(project.id)
                        }
                    }
                }
                
                Section("Details") {
                    Picker("Priority", selection: $priority) {
                        ForEach(Priority.allCases, id: \.self) { priority in
                            Text(priority.rawValue).tag(priority)
                        }
                    }
                    
                    HStack {
                        Text("Estimated Hours")
                        Spacer()
                        TextField("Hours", value: $estimatedHours, format: .number)
                            .textFieldStyle(.roundedBorder)
                            .frame(width: 80)
                    }
                    
                    Toggle("Has Due Date", isOn: $hasDueDate)
                    
                    if hasDueDate {
                        DatePicker("Due Date", selection: $dueDate, displayedComponents: .date)
                    }
                }
            }
            .navigationTitle("New Task")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                
                ToolbarItem(placement: .confirmationAction) {
                    Button("Create") {
                        Task {
                            let dueDateString = hasDueDate ? ISO8601DateFormatter().string(from: dueDate) : nil
                            await viewModel.createTask(
                                projectId: selectedProjectId,
                                title: title,
                                description: description.isEmpty ? nil : description,
                                priority: priority,
                                estimatedHours: estimatedHours,
                                dueDate: dueDateString
                            )
                            dismiss()
                        }
                    }
                    .disabled(title.isEmpty || selectedProjectId.isEmpty)
                }
            }
        }
        .frame(width: 600, height: 500)
    }
}
