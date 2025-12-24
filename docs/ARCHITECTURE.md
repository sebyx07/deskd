# Architecture Overview 🏗️

deskd is a multi-layered daemon designed around principles of separation of concerns, semantic UI control, and graceful degradation.

## System Layers

```
┌─────────────────────────────────────────┐
│         CLI Tool (deskctl)              │
│   JSON-RPC over Unix Socket             │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│       Daemon (deskd)                    │
│  Request Router & Protocol Handler      │
├─────────────────────────────────────────┤
│   Desktop Control Layer                 │
│  ├─ AT-SPI Interface (semantic)         │
│  ├─ Wayland Protocols (direct)          │
│  └─ X11 Fallback (compat)               │
├─────────────────────────────────────────┤
│   Input Simulation (Smart Fallback)     │
│  ├─ Portal (universal, secure)          │
│  ├─ Compositor IPC (fast, direct)       │
│  ├─ libei/libinput (emerging)           │
│  └─ ydotool (userspace fallback)        │
├─────────────────────────────────────────┤
│   Session Management                    │
│  ├─ logind Session Discovery            │
│  ├─ D-Bus Session Tracking              │
│  └─ Multi-Desktop Orchestration         │
├─────────────────────────────────────────┤
│   Persistent State (SQLite)             │
│  ├─ Tasks & Workflows                   │
│  ├─ Audit Logs                          │
│  ├─ Authentication Tokens               │
│  └─ Element Cache                       │
└─────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│   Linux Kernel & Display Servers        │
│  ├─ Wayland Compositor                  │
│  ├─ X11 Server (XWayland)               │
│  └─ systemd/logind                      │
└─────────────────────────────────────────┘
```

## Core Components

### 1. Daemon (deskd)

Main service process handling:
- **Request routing** - Parse JSON-RPC, dispatch to handlers
- **Session discovery** - Find all active desktop sessions
- **Multi-desktop orchestration** - Track and switch between desktops
- **Error recovery** - Retry with fallback methods
- **Performance** - Connection pooling, caching, async I/O

**Technology**: Tokio async runtime, Unix sockets, JSON-RPC

### 2. AT-SPI Interface

Primary method for UI element discovery and semantic control:

```
Application (GTK/Qt)
    ↓
AT-SPI D-Bus
    ↓
deskd AT-SPI Client
    ↓
Accessibility Tree (in-memory cache)
    ↓
Element Query & Action
```

Advantages:
- Works across GTK, Qt, Java Swing, etc.
- Semantic understanding (roles, states, labels)
- Accessible names and descriptions
- Built-in action interfaces (click, type, etc.)

Limitations:
- Requires AT-SPI server (usually present)
- Cannot interact with native X11 apps directly
- Performance depends on app complexity

### 3. Wayland Integration

Direct desktop control without going through apps:

**Method Priority**:
1. **RemoteDesktop Portal** - Universal, requires permission
2. **Compositor IPC** - Fast, compositor-specific (Sway, Hyprland)
3. **libei/libinput** - Emerging standard
4. **ydotool** - Userspace fallback

Supports:
- Input simulation (keyboard, mouse)
- Screenshot/screen capture
- Window management
- Clipboard operations
- Multi-monitor coordination

### 4. Session Management

Auto-discovers and tracks:
- User sessions via logind
- Active desktop environments
- D-Bus session addresses
- Wayland compositor types
- X11 displays
- Multi-monitor setups

Stored in `desktop_sessions` SQLite table for persistence.

### 5. SQLite Database

Persistent storage for:
- **Tasks** - Queue of operations to execute
- **Task History** - Audit log of completed tasks
- **Workflows** - Multi-step automation definitions
- **Workflow State** - Resume interrupted workflows
- **Element Cache** - Optional persistent UI element cache
- **Auth Tokens** - API token storage
- **Desktop Sessions** - Active session registry
- **Preferences** - User settings per desktop

Connection pooling via r2d2 for concurrency.

### 6. Input Simulation Layer

Intelligent fallback system for typing/clicking:

```
High-level: "click button, type text"
    ↓
Input Type Decision
    ├─ Can use AT-SPI action? (semantic)
    │   ↓
    │   Try AT-SPI → Success ✓
    │   ↓ (Failure)
    │
    ├─ Can use coordinate-based? (visual)
    │   ↓
    │   Try Portal → Success ✓
    │   ↓ (Failure)
    │
    ├─ Try Compositor IPC → Success ✓
    │   ↓ (Failure)
    │
    └─ Try ydotool → Success ✓
        ↓ (Failure)

        Error: All methods failed
```

Each failure logs diagnostics for debugging.

## Data Flow Examples

### Typing Text

```
deskctl type "Hello"
    ↓
JSON-RPC: { "method": "type", "params": { "text": "Hello" } }
    ↓
Daemon receives request
    ↓
Input Handler decides method:
  1. Try AT-SPI action on focused element
  2. Fall back to key simulation via Portal
  3. Fall back to libei
  4. Fall back to ydotool
    ↓
Text appears in focused input field
    ↓
Return: { "success": true, "method_used": "atspi_action" }
```

### Clicking Element

```
deskctl click "Submit"
    ↓
Query AT-SPI tree for element named "Submit"
    ↓
Element found at coordinates (150, 200)
    ↓
Try AT-SPI action first
    ├─ Success: Action triggered
    └─ Fail: Try coordinate-based click
        ↓
        Use Portal/Compositor/libei/ydotool
        ↓
        Verify click succeeded (screenshot/state check)
    ↓
Return: { "success": true, "element": "Submit", "method": "..." }
```

### Multi-Desktop Operation

```
deskctl --desktop kde-wayland-1 type "text"
    ↓
Look up desktop "kde-wayland-1" in desktop_sessions table
    ↓
Switch to that D-Bus session
    ↓
Connect to KDE's AT-SPI bus
    ↓
Execute type operation on KDE desktop
    ↓
Return result
```

## Design Principles

### Semantic First, Visual Fallback

- Prefer AT-SPI semantic operations (more reliable)
- Fall back to coordinate-based only when necessary
- Never rely on vision/OCR as primary method

### Wayland Native, X11 Compatible

- Assume Wayland is primary
- Use XDG Desktop Portals for compatibility
- Support X11 via XWayland detection
- Never assume X11 is available

### Security by Default

- User daemon isolation via systemd
- Portal permissions for privileged operations
- Memory zeroing for sensitive operations
- Audit trail of all operations
- No sudo required for user daemon

### Graceful Degradation

- Multiple input methods with fallback chain
- Partial failures return useful diagnostics
- Never panic in daemon code
- Log detailed errors for debugging
- Suggest remediation in error messages

### Performance Targets

- **Sub-100ms latency** for most operations
- Connection pooling for DB and AT-SPI
- In-memory caching with SQLite persistence
- Async I/O prevents blocking
- Batch operations use transactions

## Error Handling Strategy

Every operation has a retry strategy:

1. **Attempt** with primary method
2. **Log** detailed diagnostics
3. **Analyze** failure reason
4. **Try** alternative method
5. **Return** result or detailed error

Example diagnostic log:
```
[WARN] AT-SPI action failed for element "Submit": timeout
[INFO] Falling back to coordinate-based click at (150, 200)
[DEBUG] Using Portal for coordinate input simulation
[INFO] Click succeeded, element state changed
```

## Extensibility

deskd is designed for extension:

- **New input methods** - Add to fallback chain
- **New protocols** - Implement IPC variant
- **New data sources** - Add database tables
- **New operations** - Add JSON-RPC methods
- **New compositors** - Add compositor-specific drivers

See [DEVELOPMENT.md](./DEVELOPMENT.md) for contribution guidelines.

## Comparison: deskd vs Alternatives

| Feature | deskd | Dogtail | Computer Use APIs | AskUI |
|---------|-------|---------|-------------------|-------|
| Wayland-native | ✓ | ✗ | ? | ? |
| Multi-user | ✓ | ✗ | ? | ✗ |
| AT-SPI semantic | ✓ | ✓ | ✗ | ✗ |
| Persistent state | ✓ | ✗ | ? | ✗ |
| Local control | ✓ | ✓ | ✗ | ✗ |
| Free/Open | ✓ | ✓ | ✗ | ✗ |
| Production-ready | 🔄 | ✓ | ✓ | ✓ |
| Linux-first | ✓ | ✓ | ✗ | ✗ |

---

See [DEPLOYMENT.md](./DEPLOYMENT.md) for deployment architecture and [DATABASE.md](./DATABASE.md) for data schema details.
