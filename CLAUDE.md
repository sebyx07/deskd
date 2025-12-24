# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**deskd** is a production-grade Rust daemon that exposes Linux desktop control through AT-SPI for multi-user, multi-desktop environments. It is Wayland-native with X11 compatibility and uses SQLite for persistent state management.

Key differentiators:
- Wayland-first design (not X11 with Wayland hacks)
- Multi-user support from the ground up (per-user or system-wide daemon models)
- Persistent state in SQLite for tasks, workflows, and audit logs
- Multiple input simulation methods with intelligent fallback
- Standard Linux daemon architecture following systemd conventions

## Documentation

- **MVP Specs:** See `mvp/**/*.txt` for detailed feature specifications and implementation notes
- Read relevant .txt files before implementing features to understand requirements and design decisions

## Build & Development Commands

Since this is an empty project, these commands will be added as the build system is set up:

```bash
# Build the project
cargo build

# Build with release optimizations
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests

# Lint
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check without building
cargo check
```

## Architecture Overview

### Deployment Models

**Per-User Instance (Primary)**
- Each user runs their own daemon via `systemd --user`
- Socket: `~/.local/run/deskd.sock`
- Database: `~/.local/share/deskd/state.db`
- Config: `~/.config/deskd/deskd.conf`
- Most secure, isolated by user boundaries

**System-Wide Daemon (Alternative)**
- Single daemon serving all users
- Socket: `/var/run/deskd.sock`
- Database: `/var/lib/deskd/state.db`
- Requires privilege separation and authentication

**Hybrid Model (Recommended)**
- User daemons for desktop interaction
- Optional system daemon for coordination
- Cross-user workflow support

### Core Components

**Daemon (`deskd`)**
- Main service process
- Handles multiple desktop sessions per user
- Async I/O using Tokio
- AT-SPI integration for accessibility tree
- SQLite connection pooling (r2d2 crate)

**CLI Tool (`deskctl`)**
- User-facing command-line interface
- Communicates with daemon via Unix socket
- Supports desktop management, input operations, clipboard, workflows, and database queries

**Database Layer**
- SQLite for persistent state
- Core tables: tasks, task_history, auth_tokens, workflows, workflow_state, element_cache, desktop_sessions, preferences
- Automatic schema migration on startup
- Connection pooling for performance

**Wayland Integration**
- RemoteDesktop Portal (preferred, universal)
- Compositor-specific IPC (Sway, Hyprland, KWin)
- libei/libinput for input injection
- ydotool fallback
- Auto-detection of compositor type and capabilities

**AT-SPI Interface**
- Primary method for UI element discovery
- Preferred method for semantic operations (click, type, focus)
- Fallback to coordinate-based input when AT-SPI unavailable

**Desktop Control Subagent**
- Use Task tool with `subagent_type='desktop-controller'` when needing to interact with UI elements
- Handles clicking, typing, navigation, screenshots, and UI verification
- Automates desktop tasks by combining deskd primitives with visual/AT-SPI context
- For testing UI workflows or debugging desktop application behavior

### Input Simulation Priority

**Typing:**
1. AT-SPI action interface (semantic, preferred)
2. Wayland input simulation
3. X11 input simulation (fallback)
4. Direct libinput injection

**Clicking:**
1. AT-SPI action interface (semantic)
2. Wayland input simulation (coordinates)
3. X11 input simulation (fallback)
4. Direct libinput injection

**Screenshot:**
1. ScreenCast Portal (universal)
2. wlr-screencopy (wlroots compositors)
3. Compositor-specific DBus methods
4. grim/slurp external tools

### Multi-Desktop Session Handling

- Daemon discovers all user sessions via logind
- Tracks active desktop environments in `desktop_sessions` table
- Each desktop has separate AT-SPI and D-Bus connection
- Commands can specify target desktop or auto-select primary
- Supports multiple monitors with global coordinate system

## Code Organization Principles

### Code Quality & SOLID Principles
- **Single Responsibility:** Each module/struct should have one reason to change
- **Open/Closed:** Extend behavior via traits, not modification
- **Dependency Inversion:** Depend on abstractions (traits), not concrete types
- Keep functions small and focused; separate concerns into different modules
- Use Rust's type system for compile-time guarantees
- Avoid unwrap() in production; use proper error propagation

### Error Handling
- Return clear, actionable errors
- Implement retry logic with configurable attempts
- Try alternative methods on failure (e.g., if AT-SPI fails, try coordinate-based input)
- Log detailed diagnostics for debugging
- Never panic in daemon code; use Result types

### Security Requirements
- `type_secure` method must zero memory after use and never log content
- Validate user permissions for cross-user operations in system daemon mode
- Authenticate Unix socket connections in system-wide mode
- Audit all operations in task_history table
- Clipboard operations should be auditable

### Async Architecture
- All I/O operations are async (Tokio runtime)
- Non-blocking database access
- Event-driven session monitoring
- Concurrent request handling

### Wayland-First Design
- Never assume X11 is available
- Detect compositor type and capabilities at runtime
- Use XDG Desktop Portals as preferred method
- Implement compositor-specific optimizations (Sway IPC, KWin DBus, etc.)
- Handle XWayland separately for legacy apps

### Database Versioning
- Schema version tracked in `schema_version` table
- Automatic migration on daemon startup
- Migration scripts in `/usr/share/deskd/migrations/`
- Backup before applying migrations

## Testing Strategy

- Unit tests for individual components (AT-SPI parser, input methods, database operations)
- Integration tests for full workflow (start daemon, execute command, verify result)
- Mock AT-SPI interfaces for testing without real desktop
- Test multi-user scenarios
- Test permission boundaries
- Test all fallback chains (e.g., Portal → libei → ydotool)
- Performance tests for sub-100ms latency requirement

## Configuration Management

Configuration file (`deskd.conf`) supports:
- Session discovery mode (auto/manual)
- Primary desktop selection
- Database path and backup settings
- Wayland compositor and input method preferences
- Input timing (typing delay, click delay, focus timeout)
- Retry behavior
- Clipboard settings
- Multi-desktop options
- Task persistence settings

## Systemd Integration

- User service: `systemd --user` unit file
- Depends on `graphical-session.target`
- Type=notify for proper startup signaling
- Wait for Wayland compositor before starting
- Support service templates for multi-desktop: `deskd@gnome.service`, `deskd@kde.service`
- Automatic restart on failure

## Protocol Methods Reference

Core operations exposed via JSON-RPC or similar protocol:
- Input: `type`, `type_secure`, `key_press`, `key_combo`, `paste`, `copy`
- Clicking: `click`, `right_click`, `double_click`, `drag`, `scroll_click`
- Focus: `focus`, `focus_window`, `get_focused_element`, `wait_for_focus`
- Desktop: `list_desktops`, `switch_desktop`, `execute_on_desktop`
- Clipboard: `clipboard_get`, `clipboard_set`, `clipboard_history`
- Database: `db_query`, `get_task_history`, `save_workflow`, `resume_workflow`

## Important Design Decisions

**Why SQLite:**
- Zero configuration, embedded
- Transparent, auditable storage
- Standard SQL for querying audit logs
- ACID transactions for reliability
- No external database server required

**Why AT-SPI Primary:**
- Semantic understanding of UI elements
- Accessibility information (roles, states, labels)
- More reliable than vision/OCR approaches
- Standard Linux accessibility protocol
- Works across GTK, Qt, and other toolkits

**Why Multiple Input Methods:**
- Wayland compositors vary in capabilities
- No single universal input method exists yet
- Graceful degradation improves reliability
- Portal method most secure but requires user permission
- Direct methods (libei, ydotool) work without prompts

**Performance Targets:**
- Sub-100ms latency for most operations
- Connection pooling for database and AT-SPI
- In-memory caching with SQLite persistence
- Batch operations use transactions
- Async I/O prevents blocking

## Success Metrics

- Control all GTK/Qt applications via AT-SPI
- Work on pure Wayland without X11
- Handle multiple users simultaneously
- Persist tasks across daemon restarts
- 99%+ reliability for focus/type/click operations
- Sub-100ms latency for most operations
- Complete audit trail in SQLite
- Zero-config installation via package manager
