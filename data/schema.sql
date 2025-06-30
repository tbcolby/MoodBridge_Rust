-- Script to bootstrap the SQLite database
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS case_info (
  id INTEGER PRIMARY KEY,
  docket_number TEXT UNIQUE NOT NULL,
  case_title TEXT NOT NULL,
  court TEXT NOT NULL,
  status TEXT,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS placement_denials (
  id INTEGER PRIMARY KEY,
  denied_date TEXT NOT NULL,
  requested_start_time TEXT,
  requested_end_time TEXT,
  duration_hours REAL,
  denial_reason TEXT,
  violation_category TEXT,
  evidence_attached TEXT,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS timeline_events (
  id INTEGER PRIMARY KEY,
  event_date TEXT NOT NULL,
  event_type TEXT,
  event_title TEXT NOT NULL,
  event_description TEXT,
  importance_level INTEGER DEFAULT 3,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS exhibits (
  id INTEGER PRIMARY KEY,
  exhibit_label TEXT,
  document_name TEXT NOT NULL,
  file_path TEXT,
  file_size_bytes INTEGER,
  media_type TEXT,
  hash_sha256 TEXT,
  description TEXT,
  category TEXT,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS communications (
  id INTEGER PRIMARY KEY,
  communication_date TEXT NOT NULL,
  sender TEXT,
  recipient TEXT,
  medium TEXT,
  subject TEXT,
  message_content TEXT,
  related_to_placement BOOLEAN DEFAULT 0,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS violations (
  id INTEGER PRIMARY KEY,
  violation_date TEXT NOT NULL,
  violation_type TEXT,
  description TEXT,
  stipulation_reference TEXT,
  impact_score INTEGER DEFAULT 1,
  placement_denial_id INTEGER REFERENCES placement_denials(id),
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- AI Insights and Analytics Tables
CREATE TABLE IF NOT EXISTS ai_insights (
  id INTEGER PRIMARY KEY,
  entity_type TEXT NOT NULL, -- 'case', 'document', 'communication', etc.
  entity_id INTEGER NOT NULL,
  insight_type TEXT NOT NULL, -- 'pattern', 'risk_assessment', 'recommendation'
  insight_data TEXT NOT NULL, -- JSON data
  confidence_score REAL DEFAULT 0.0,
  generated_by TEXT DEFAULT 'ai_assistant',
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS document_vectors (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL,
  vector_data BLOB, -- Embedding vectors for semantic search
  chunk_index INTEGER DEFAULT 0,
  chunk_text TEXT,
  metadata TEXT, -- JSON metadata
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (document_id) REFERENCES exhibits(id)
);

CREATE TABLE IF NOT EXISTS ai_analysis_logs (
  id INTEGER PRIMARY KEY,
  operation_type TEXT NOT NULL,
  input_data TEXT,
  output_data TEXT,
  processing_time_ms INTEGER,
  model_used TEXT,
  success BOOLEAN DEFAULT 1,
  error_message TEXT,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS legal_patterns (
  id INTEGER PRIMARY KEY,
  pattern_name TEXT UNIQUE NOT NULL,
  pattern_type TEXT NOT NULL, -- 'violation', 'timeline', 'communication'
  pattern_description TEXT,
  detection_criteria TEXT, -- JSON criteria
  severity_weight REAL DEFAULT 1.0,
  active BOOLEAN DEFAULT 1,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Add AI metadata columns to existing tables
ALTER TABLE placement_denials ADD COLUMN ai_risk_score REAL DEFAULT 0.0;
ALTER TABLE placement_denials ADD COLUMN ai_analysis TEXT; -- JSON AI analysis
ALTER TABLE timeline_events ADD COLUMN ai_generated BOOLEAN DEFAULT 0;
ALTER TABLE timeline_events ADD COLUMN ai_confidence REAL DEFAULT 0.0;
ALTER TABLE communications ADD COLUMN sentiment_score REAL DEFAULT 0.0;
ALTER TABLE communications ADD COLUMN ai_summary TEXT;
ALTER TABLE exhibits ADD COLUMN ai_extracted_text TEXT;
ALTER TABLE exhibits ADD COLUMN ai_content_type TEXT;
ALTER TABLE exhibits ADD COLUMN processing_status TEXT DEFAULT 'pending';

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_ai_insights_entity ON ai_insights(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_ai_insights_type ON ai_insights(insight_type);
CREATE INDEX IF NOT EXISTS idx_document_vectors_doc ON document_vectors(document_id);
CREATE INDEX IF NOT EXISTS idx_legal_patterns_type ON legal_patterns(pattern_type);
CREATE INDEX IF NOT EXISTS idx_placement_denials_risk ON placement_denials(ai_risk_score);
CREATE INDEX IF NOT EXISTS idx_timeline_events_ai ON timeline_events(ai_generated);

-- Prepopulate initial data
INSERT OR IGNORE INTO case_info (docket_number, case_title, court, status)
VALUES ('DEMO-2024-001', 'Anonymous v. Anonymous', 'Demo County Circuit Court', 'Active');

-- Insert sample legal patterns
INSERT OR IGNORE INTO legal_patterns (pattern_name, pattern_type, pattern_description, detection_criteria) VALUES
('Recurring Denial Pattern', 'violation', 'Detects patterns of recurring placement denials', '{"min_denials": 3, "time_window_days": 30}'),
('Communication Gap', 'communication', 'Identifies unusual gaps in communication', '{"max_gap_days": 7, "critical_periods": ["pre_placement", "post_denial"]}'),
('Evidence Correlation', 'timeline', 'Correlates timeline events with available evidence', '{"evidence_types": ["document", "communication"], "correlation_threshold": 0.8}');

-- Project Management Tables
CREATE TABLE IF NOT EXISTS projects (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL DEFAULT 'planning', -- planning, active, paused, completed, cancelled
  priority TEXT NOT NULL DEFAULT 'medium', -- critical, high, medium, low
  start_date TEXT,
  target_date TEXT,
  completion_date TEXT,
  progress_percentage REAL DEFAULT 0.0,
  project_type TEXT NOT NULL DEFAULT 'feature', -- security, feature, infrastructure, documentation
  owner TEXT,
  estimated_hours REAL,
  actual_hours REAL DEFAULT 0.0,
  tags TEXT, -- JSON array of tags
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  project_id INTEGER NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL DEFAULT 'todo', -- todo, in_progress, review, testing, done, blocked
  priority TEXT NOT NULL DEFAULT 'medium', -- critical, high, medium, low
  task_type TEXT NOT NULL DEFAULT 'implementation', -- implementation, testing, documentation, bug_fix, research
  assignee TEXT,
  estimated_hours REAL,
  actual_hours REAL DEFAULT 0.0,
  due_date TEXT,
  completion_date TEXT,
  blocked_reason TEXT,
  dependencies TEXT, -- JSON array of task IDs
  labels TEXT, -- JSON array of labels
  ai_priority_score REAL,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS milestones (
  id INTEGER PRIMARY KEY,
  project_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  target_date TEXT NOT NULL,
  completion_date TEXT,
  status TEXT NOT NULL DEFAULT 'upcoming', -- upcoming, active, completed, missed
  milestone_type TEXT NOT NULL DEFAULT 'phase', -- phase, release, deadline, review
  success_criteria TEXT, -- JSON array of criteria
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS project_dependencies (
  id INTEGER PRIMARY KEY,
  dependent_project_id INTEGER NOT NULL,
  dependency_project_id INTEGER NOT NULL,
  dependency_type TEXT NOT NULL DEFAULT 'blocks', -- blocks, requires, enhances
  description TEXT,
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (dependent_project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (dependency_project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS work_sessions (
  id INTEGER PRIMARY KEY,
  task_id INTEGER NOT NULL,
  start_time TEXT NOT NULL,
  end_time TEXT,
  duration_minutes INTEGER,
  notes TEXT,
  session_type TEXT NOT NULL DEFAULT 'focused', -- focused, research, debugging, testing, documentation
  productivity_score INTEGER, -- 1-10 scale
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

-- Indexes for project management performance
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
CREATE INDEX IF NOT EXISTS idx_projects_priority ON projects(priority);
CREATE INDEX IF NOT EXISTS idx_projects_type ON projects(project_type);
CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);
CREATE INDEX IF NOT EXISTS idx_milestones_project_id ON milestones(project_id);
CREATE INDEX IF NOT EXISTS idx_milestones_target_date ON milestones(target_date);
CREATE INDEX IF NOT EXISTS idx_work_sessions_task_id ON work_sessions(task_id);
CREATE INDEX IF NOT EXISTS idx_work_sessions_start_time ON work_sessions(start_time);

-- Triggers to automatically update timestamps
CREATE TRIGGER IF NOT EXISTS update_project_timestamp 
  AFTER UPDATE ON projects
  BEGIN
    UPDATE projects SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
  END;

CREATE TRIGGER IF NOT EXISTS update_task_timestamp 
  AFTER UPDATE ON tasks
  BEGIN
    UPDATE tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
  END;

-- Initial project data based on our analysis
INSERT OR IGNORE INTO projects (name, description, status, priority, project_type, owner, estimated_hours, tags) VALUES
('MoodBridge Security Hardening', 'Critical security implementation for production readiness', 'active', 'critical', 'security', 'tyler', 24.0, '["security", "production", "authentication", "database"]'),
('DevCopilot Pro Launch', 'Complete and launch the AI development assistant', 'active', 'high', 'feature', 'tyler', 8.0, '["ai", "development", "launch", "tools"]'),
('MoodBridge API Development', 'Complete Phase 2 API endpoints implementation', 'active', 'high', 'feature', 'tyler', 32.0, '["api", "backend", "crud", "endpoints"]'),
('Project Synchronization', 'Resolve repository conflicts and sync multiple versions', 'active', 'medium', 'infrastructure', 'tyler', 4.0, '["git", "sync", "cleanup"]'),
('Testing Infrastructure', 'Implement comprehensive testing framework', 'planning', 'high', 'infrastructure', 'tyler', 20.0, '["testing", "quality", "automation"]'),
('Frontend Development', 'Askama templates and interactive dashboard', 'planning', 'medium', 'feature', 'tyler', 16.0, '["frontend", "ui", "templates", "dashboard"]'),
('Enterprise Features', 'Multi-tenancy and advanced case management', 'planning', 'medium', 'feature', 'tyler', 60.0, '["enterprise", "multi-tenant", "advanced"]');

-- Insert detailed tasks for immediate priorities
INSERT OR IGNORE INTO tasks (project_id, title, description, status, priority, task_type, estimated_hours, due_date) VALUES
-- MoodBridge Security Hardening tasks
(1, 'Migrate to persistent SQLite database', 'Replace in-memory database with persistent SQLite storage', 'todo', 'critical', 'implementation', 4.0, '2025-07-01'),
(1, 'Implement JWT authentication middleware', 'Add JWT-based authentication system', 'todo', 'critical', 'implementation', 6.0, '2025-07-01'),
(1, 'Add rate limiting protection', 'Implement rate limiting and brute force protection', 'todo', 'critical', 'implementation', 3.0, '2025-07-01'),
(1, 'Enable HTTPS/TLS configuration', 'Configure HTTPS for production deployment', 'todo', 'critical', 'implementation', 2.0, '2025-07-01'),
(1, 'Add security headers middleware', 'Implement CSP, HSTS, and other security headers', 'todo', 'high', 'implementation', 3.0, '2025-07-02'),
(1, 'Input validation and sanitization', 'Add comprehensive input validation to all endpoints', 'todo', 'high', 'implementation', 4.0, '2025-07-02'),
(1, 'Database encryption at rest', 'Implement SQLite database encryption', 'todo', 'high', 'implementation', 2.0, '2025-07-02'),

-- DevCopilot Pro Launch tasks
(2, 'Complete initial Git commit', 'Commit and push staged files to repository', 'todo', 'high', 'implementation', 1.0, '2025-07-01'),
(2, 'Test AI assistant functionality', 'Validate AI question/answer system', 'todo', 'high', 'testing', 2.0, '2025-07-01'),
(2, 'Implement real-time system monitoring', 'Add live CPU, memory, and uptime tracking', 'todo', 'medium', 'implementation', 3.0, '2025-07-01'),
(2, 'Deploy suggestion chips system', 'Complete interactive suggestion chips UI', 'todo', 'medium', 'implementation', 2.0, '2025-07-01'),

-- MoodBridge API Development tasks
(3, 'Dashboard metrics API endpoint', 'Implement /api/dashboard-metrics endpoint', 'todo', 'high', 'implementation', 4.0, '2025-07-02'),
(3, 'Placement denials CRUD operations', 'Complete create, read, update, delete for placement denials', 'todo', 'high', 'implementation', 6.0, '2025-07-03'),
(3, 'Timeline events management API', 'Implement timeline events CRUD endpoints', 'todo', 'high', 'implementation', 6.0, '2025-07-03'),
(3, 'Exhibits upload/download API', 'File upload and download functionality', 'todo', 'medium', 'implementation', 8.0, '2025-07-04'),
(3, 'Analytics queries implementation', 'Advanced analytics and reporting queries', 'todo', 'medium', 'implementation', 8.0, '2025-07-05'),

-- Project Synchronization tasks
(4, 'Resolve submodule conflicts', 'Fix modified submodule content in /tmp/MoodBridge_Rust', 'todo', 'medium', 'bug_fix', 1.0, '2025-07-01'),
(4, 'Push pending commits', 'Push 3 unpushed commits from /tmp repository', 'todo', 'medium', 'implementation', 1.0, '2025-07-01'),
(4, 'Consolidate repository versions', 'Merge changes and remove duplicate repositories', 'todo', 'medium', 'implementation', 2.0, '2025-07-01'),

-- Testing Infrastructure tasks
(5, 'Set up unit testing framework', 'Configure cargo test environment and initial tests', 'todo', 'high', 'implementation', 4.0, '2025-07-03'),
(5, 'API endpoint integration tests', 'Create integration tests for all API endpoints', 'todo', 'high', 'testing', 8.0, '2025-07-04'),
(5, 'Authentication flow tests', 'Test authentication and authorization flows', 'todo', 'high', 'testing', 4.0, '2025-07-04'),
(5, 'Error handling test suite', 'Comprehensive error handling validation', 'todo', 'medium', 'testing', 4.0, '2025-07-05');

-- Insert milestones
INSERT OR IGNORE INTO milestones (project_id, name, description, target_date, milestone_type) VALUES
(1, 'Security Foundation Complete', 'Basic authentication and persistent storage implemented', '2025-07-02', 'phase'),
(1, 'Production Security Ready', 'All critical security measures in place', '2025-07-05', 'phase'),
(2, 'DevCopilot Pro MVP', 'Minimum viable product launched', '2025-07-01', 'release'),
(3, 'API Phase 2 Complete', 'All Phase 2 endpoints implemented and tested', '2025-07-05', 'phase'),
(4, 'Repository Cleanup Complete', 'All duplicate repositories resolved and synced', '2025-07-01', 'deadline'),
(5, 'Testing Infrastructure Ready', 'Comprehensive testing framework operational', '2025-07-05', 'phase');

-- Insert project dependencies
INSERT OR IGNORE INTO project_dependencies (dependent_project_id, dependency_project_id, dependency_type, description) VALUES
(3, 1, 'requires', 'API development requires security foundation to be in place'),
(5, 1, 'requires', 'Testing infrastructure requires stable security implementation'),
(6, 3, 'requires', 'Frontend development requires completed API endpoints'),
(7, 1, 'requires', 'Enterprise features require security and authentication foundation');

