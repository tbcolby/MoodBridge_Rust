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
VALUES ('2018-FA-004441', 'Colby v. McConnell Ek', 'Milwaukee County Circuit Court', 'Active');

-- Insert sample legal patterns
INSERT OR IGNORE INTO legal_patterns (pattern_name, pattern_type, pattern_description, detection_criteria) VALUES
('Recurring Denial Pattern', 'violation', 'Detects patterns of recurring placement denials', '{"min_denials": 3, "time_window_days": 30}'),
('Communication Gap', 'communication', 'Identifies unusual gaps in communication', '{"max_gap_days": 7, "critical_periods": ["pre_placement", "post_denial"]}'),
('Evidence Correlation', 'timeline', 'Correlates timeline events with available evidence', '{"evidence_types": ["document", "communication"], "correlation_threshold": 0.8}');

