# Database Schema 🗄️

deskd uses SQLite for persistent state management. This guide documents the schema, operations, and best practices.

## Database Location

- **Per-user**: `~/.local/share/deskd/state.db`
- **System-wide**: `/var/lib/deskd/state.db`

Configurable via `DatabasePath` in `deskd.conf`.

## Core Tables

### schema_version

Tracks database schema version for migrations.

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

| Column | Type | Description |
|--------|------|-------------|
| `version` | INTEGER | Schema version number |
| `applied_at` | TIMESTAMP | When migration was applied |

### tasks

Persistent queue of pending operations.

```sql
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    desktop_id TEXT,
    method TEXT NOT NULL,
    params JSON,
    priority INTEGER DEFAULT 0,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    scheduled_for TIMESTAMP,
    result JSON,
    error_message TEXT
);
```

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Unique task ID |
| `user_id` | TEXT | User who queued task |
| `desktop_id` | TEXT | Target desktop (optional) |
| `method` | TEXT | Operation method (click, type, etc.) |
| `params` | JSON | Operation parameters |
| `priority` | INTEGER | Priority level (higher = first) |
| `status` | TEXT | pending, running, completed, failed |
| `created_at` | TIMESTAMP | When task was created |
| `scheduled_for` | TIMESTAMP | When to execute (NULL = immediately) |
| `result` | JSON | Result of operation |
| `error_message` | TEXT | Error if failed |

Example:
```sql
INSERT INTO tasks (user_id, method, params, priority)
VALUES ('alice', 'click', '{"element": "Submit"}', 0);
```

### task_history

Audit log of completed operations.

```sql
CREATE TABLE task_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER REFERENCES tasks(id),
    user_id TEXT NOT NULL,
    desktop_id TEXT,
    method TEXT NOT NULL,
    params JSON,
    result JSON,
    success BOOLEAN,
    error_message TEXT,
    executed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    duration_ms INTEGER,
    client_token TEXT
);
```

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Unique history entry ID |
| `task_id` | INTEGER | Reference to original task |
| `user_id` | TEXT | User who executed operation |
| `desktop_id` | TEXT | Desktop where executed |
| `method` | TEXT | Operation method |
| `params` | JSON | Operation parameters |
| `result` | JSON | Operation result |
| `success` | BOOLEAN | Whether operation succeeded |
| `error_message` | TEXT | Error message if failed |
| `executed_at` | TIMESTAMP | When operation executed |
| `duration_ms` | INTEGER | How long it took |
| `client_token` | TEXT | Client identifier |

Query examples:
```sql
-- Recent operations
SELECT * FROM task_history ORDER BY executed_at DESC LIMIT 10;

-- Operations by user
SELECT * FROM task_history WHERE user_id = 'alice';

-- Failed operations
SELECT * FROM task_history WHERE success = FALSE;

-- Slowest operations
SELECT method, AVG(duration_ms) as avg_duration
FROM task_history GROUP BY method ORDER BY avg_duration DESC;
```

### auth_tokens

API token storage and metadata.

```sql
CREATE TABLE auth_tokens (
    token_hash TEXT PRIMARY KEY,
    description TEXT,
    permissions JSON,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMP,
    expires_at TIMESTAMP,
    rate_limit INTEGER,
    revoked BOOLEAN DEFAULT FALSE
);
```

| Column | Type | Description |
|--------|------|-------------|
| `token_hash` | TEXT | SHA256 hash of token |
| `description` | TEXT | Human-readable description |
| `permissions` | JSON | Array of allowed operations |
| `user_id` | TEXT | Token owner |
| `created_at` | TIMESTAMP | Creation time |
| `last_used_at` | TIMESTAMP | Last usage time |
| `expires_at` | TIMESTAMP | Token expiration (NULL = never) |
| `rate_limit` | INTEGER | Requests per second |
| `revoked` | BOOLEAN | Is token revoked? |

### workflows

Multi-step workflow definitions.

```sql
CREATE TABLE workflows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    user_id TEXT NOT NULL,
    steps JSON NOT NULL,
    metadata JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    tags JSON
);
```

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Workflow ID |
| `name` | TEXT | Unique workflow name |
| `user_id` | TEXT | Creator user |
| `steps` | JSON | Array of workflow steps |
| `metadata` | JSON | Additional metadata |
| `created_at` | TIMESTAMP | Creation time |
| `updated_at` | TIMESTAMP | Last modification time |
| `tags` | JSON | Array of tags for organization |

Example workflow JSON:
```json
{
  "steps": [
    {"action": "click", "element": "Username"},
    {"action": "type", "text": "alice@example.com"},
    {"action": "key", "key": "Tab"},
    {"action": "type", "text": "password", "secure": true},
    {"action": "click", "element": "Sign In"}
  ]
}
```

### workflow_state

Resume interrupted workflows.

```sql
CREATE TABLE workflow_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workflow_id INTEGER NOT NULL REFERENCES workflows(id),
    current_step INTEGER,
    state JSON,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    paused_at TIMESTAMP,
    resumed_at TIMESTAMP,
    completed_at TIMESTAMP
);
```

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | State record ID |
| `workflow_id` | INTEGER | Workflow being executed |
| `current_step` | INTEGER | Current step index |
| `state` | JSON | Step-local state/variables |
| `started_at` | TIMESTAMP | Execution start time |
| `paused_at` | TIMESTAMP | When paused |
| `resumed_at` | TIMESTAMP | When resumed |
| `completed_at` | TIMESTAMP | Completion time |

### element_cache

Optional persistent UI element cache.

```sql
CREATE TABLE element_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    desktop_id TEXT NOT NULL,
    window_id TEXT,
    element_path TEXT,
    properties JSON,
    cached_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP
);
```

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Cache entry ID |
| `desktop_id` | TEXT | Desktop where element exists |
| `window_id` | TEXT | Parent window ID |
| `element_path` | TEXT | AT-SPI path to element |
| `properties` | JSON | Cached element properties |
| `cached_at` | TIMESTAMP | When cached |
| `expires_at` | TIMESTAMP | Cache expiration time |

### desktop_sessions

Active desktop environment registry.

```sql
CREATE TABLE desktop_sessions (
    session_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    desktop_type TEXT,
    display TEXT,
    d_bus_address TEXT,
    wayland_compositor TEXT,
    wayland_display TEXT,
    x11_display TEXT,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

| Column | Type | Description |
|--------|------|-------------|
| `session_id` | TEXT | logind session ID |
| `user_id` | TEXT | User running session |
| `desktop_type` | TEXT | GNOME, KDE, Sway, etc. |
| `display` | TEXT | Display identifier |
| `d_bus_address` | TEXT | D-Bus session address |
| `wayland_compositor` | TEXT | Compositor type |
| `wayland_display` | TEXT | WAYLAND_DISPLAY value |
| `x11_display` | TEXT | DISPLAY value (if available) |
| `started_at` | TIMESTAMP | When session started |
| `last_seen` | TIMESTAMP | Last activity time |

Query example:
```sql
-- List active desktops for current user
SELECT session_id, desktop_type, display FROM desktop_sessions
WHERE user_id = 'alice' AND last_seen > datetime('now', '-5 minutes');
```

### preferences

User settings per desktop.

```sql
CREATE TABLE preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    desktop_id TEXT,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, desktop_id, key)
);
```

| Column | Type | Description |
|--------|------|-------------|
| `id` | INTEGER | Preference ID |
| `user_id` | TEXT | User |
| `desktop_id` | TEXT | Desktop (NULL = global) |
| `key` | TEXT | Setting key |
| `value` | TEXT | Setting value |
| `updated_at` | TIMESTAMP | Last update time |

## Database Operations

### Check Database Version

```bash
deskctl db version
```

### View Statistics

```bash
deskctl db stats
```

Shows:
- Total records per table
- Database size
- Last backup time
- Cache statistics

### Query History

```bash
# All operations
deskctl db history

# Filter by user
deskctl db history --user alice

# Filter by time
deskctl db history --since "1 day ago"

# Filter by method
deskctl db history --method click

# Only failures
deskctl db history --error-only

# Limit results
deskctl db history --limit 50
```

### Backup Database

```bash
# Automatic backup (enabled by default)
# BackupInterval in deskd.conf (default: 86400 seconds)

# Manual backup
deskctl db backup ~/backup-$(date +%Y%m%d).db

# List backups
deskctl db backup --list
```

### Optimize Database

```bash
deskctl db vacuum
```

Runs SQLite VACUUM to:
- Reclaim unused space
- Optimize query performance
- Rebuild indexes

### Clean Old Records

```bash
# Remove history older than 30 days
deskctl db cleanup --older-than "30 days" --confirm

# Remove task records older than 7 days
deskctl db cleanup --older-than "7 days" --table tasks --confirm

# Dry run (don't delete)
deskctl db cleanup --older-than "30 days"
```

### Export Data

```bash
# Export as JSON
deskctl db export --output audit.json

# Export specific table
deskctl db export --output tasks.json --table tasks

# Export as CSV
deskctl db export --output history.csv --format csv

# Filter by time
deskctl db export --since "2025-01-01" --output january.json
```

## Direct SQL Queries

For advanced operations, use direct SQL:

```bash
deskctl db query "SELECT * FROM task_history WHERE success = FALSE LIMIT 10"

deskctl db query "SELECT method, COUNT(*) FROM task_history GROUP BY method"

deskctl db query "DELETE FROM element_cache WHERE expires_at < datetime('now')"
```

## Best Practices

### Performance

- Use indexes for frequently queried columns
- Vacuum database weekly: `deskctl db vacuum`
- Archive old history: `deskctl db cleanup --older-than "30 days"`
- Batch insertions in transactions

### Data Integrity

- Regular backups: `deskctl db backup ~/backups/state-$(date +%Y%m%d).db`
- Use transactions for multi-step operations
- Verify checksums on restore

### Security

- Database permissions: `chmod 600 state.db`
- Never log sensitive data (`type_secure`)
- Audit all operations via `task_history`
- Rotate auth tokens regularly

### Retention

Configure in `deskd.conf`:
```ini
[Tasks]
MaxHistoryAge = 2592000  # 30 days
AutoCleanupHistory = yes
CleanupInterval = 86400  # Daily
```

## Schema Migrations

On daemon startup:

1. Check `schema_version` table
2. Apply pending migrations
3. Update version number
4. Backup before migration

Migration scripts in `/usr/share/deskd/migrations/`:
- `001_initial.sql` - Initial schema
- `002_add_workflows.sql` - Workflow support
- `003_add_sessions.sql` - Multi-desktop sessions

Manual migration:

```bash
# Check current version
deskctl db version

# Manually apply migrations
deskctl db migrate

# Rollback to specific version
deskctl db migrate --to 2
```

## Troubleshooting

### Database Locked

```bash
# Check for running operations
ps aux | grep deskd

# Restart daemon
systemctl --user restart deskd
```

### Corruption

```bash
# Attempt repair
deskctl db repair

# Or restore from backup
deskctl db restore ~/backup-20250101.db
```

### Large Database

```bash
# Check size
ls -lh ~/.local/share/deskd/state.db

# Vacuum to reclaim space
deskctl db vacuum

# Archive old history
deskctl db cleanup --older-than "90 days" --confirm
```

---

See [CONFIGURATION.md](./CONFIGURATION.md) for database configuration options and [CLI_REFERENCE.md](./CLI_REFERENCE.md) for the complete `db` command reference.
