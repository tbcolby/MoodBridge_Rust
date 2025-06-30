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

-- Prepopulate initial data if necessary (example)
INSERT OR IGNORE INTO case_info (docket_number, case_title, court, status)
VALUES ('2018-FA-004441', 'Colby v. McConnell Ek', 'Milwaukee County Circuit Court', 'Active');

