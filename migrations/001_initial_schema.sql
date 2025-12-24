-- Initial database schema for deskd
-- Tables: tasks, task_history, auth_tokens, workflows, workflow_state,
-- element_cache, desktop_sessions, preferences, schema_version

-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tasks table for tracking async operations
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    task_type TEXT NOT NULL,
    status TEXT NOT NULL, -- pending, running, completed, failed
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    error TEXT,
    metadata TEXT -- JSON
);

-- Task history for audit trail
CREATE TABLE IF NOT EXISTS task_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    details TEXT, -- JSON
    user_id TEXT,
    session_id TEXT,
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);

-- Authentication tokens for system-wide daemon
CREATE TABLE IF NOT EXISTS auth_tokens (
    token TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    permissions TEXT -- JSON array
);

-- Workflows for multi-step operations
CREATE TABLE IF NOT EXISTS workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    steps TEXT NOT NULL, -- JSON array
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Workflow state for resumable workflows
CREATE TABLE IF NOT EXISTS workflow_state (
    workflow_id TEXT PRIMARY KEY,
    current_step INTEGER NOT NULL DEFAULT 0,
    state TEXT, -- JSON
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (workflow_id) REFERENCES workflows(id)
);

-- Element cache for AT-SPI elements
CREATE TABLE IF NOT EXISTS element_cache (
    element_id TEXT PRIMARY KEY,
    desktop_id TEXT NOT NULL,
    role TEXT NOT NULL,
    name TEXT,
    description TEXT,
    path TEXT, -- AT-SPI object path
    cached_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP
);

-- Desktop sessions tracking
CREATE TABLE IF NOT EXISTS desktop_sessions (
    session_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    display TEXT NOT NULL, -- :0, :1, etc.
    compositor TEXT, -- gnome, kde, sway, hyprland, etc.
    wayland_display TEXT, -- wayland-0, wayland-1, etc.
    is_active BOOLEAN DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- User preferences
CREATE TABLE IF NOT EXISTS preferences (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Clipboard history (optional feature)
CREATE TABLE IF NOT EXISTS clipboard_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    content_type TEXT, -- text/plain, image/png, etc.
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    session_id TEXT
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_task_history_task_id ON task_history(task_id);
CREATE INDEX IF NOT EXISTS idx_task_history_timestamp ON task_history(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_element_cache_desktop_id ON element_cache(desktop_id);
CREATE INDEX IF NOT EXISTS idx_desktop_sessions_user_id ON desktop_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_clipboard_history_timestamp ON clipboard_history(timestamp DESC);

-- Insert initial schema version
INSERT INTO schema_version (version) VALUES (1);
