# Configuration Guide ⚙️

Configure deskd behavior via the configuration file or command-line arguments.

## Configuration File Locations

- **Per-user**: `~/.config/deskd/deskd.conf`
- **System-wide**: `/etc/deskd/deskd.conf`

The daemon loads configuration in this priority order:
1. Command-line arguments (highest)
2. Per-user config file
3. System-wide config file
4. Built-in defaults (lowest)

## Configuration Format

The config file uses simple `Key = Value` format (INI-style):

```ini
# Comments start with #

[Session]
SessionDiscovery = auto
PrimaryDesktop = gnome-wayland-0

[Database]
Path = ~/.local/share/deskd/state.db
BackupInterval = 86400
```

## Configuration Options

### Session Management

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `SessionDiscovery` | auto, manual | auto | Auto-discover active sessions |
| `PrimaryDesktop` | desktop-id | (auto) | Primary desktop for operations |
| `MonitorSessions` | yes, no | yes | Track session changes |
| `DesktopSwitchTimeout` | milliseconds | 5000 | Timeout for desktop switch |

Example:
```ini
[Session]
SessionDiscovery = auto
PrimaryDesktop = gnome-wayland-0
MonitorSessions = yes
DesktopSwitchTimeout = 5000
```

### Database

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `Path` | file path | ~/.local/share/deskd/state.db | Database file location |
| `BackupInterval` | seconds | 86400 (1 day) | Auto-backup interval |
| `VacuumOnStartup` | yes, no | no | Optimize DB at startup |
| `MaxConnections` | number | 5 | Connection pool size |

Example:
```ini
[Database]
Path = ~/.local/share/deskd/state.db
BackupInterval = 86400
VacuumOnStartup = no
MaxConnections = 5
```

### Wayland

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `WaylandCompositor` | auto, gnome, kde, sway, hyprland, wlroots | auto | Override compositor detection |
| `InputMethod` | portal, compositor, libei, ydotool, auto | auto | Input simulation method |
| `ScreenshotMethod` | portal, screencopy, compositor, grim, auto | auto | Screenshot method |
| `PortalPermissionTimeout` | seconds | 30 | Wait time for portal permission |

Example:
```ini
[Wayland]
WaylandCompositor = auto
InputMethod = portal
ScreenshotMethod = portal
PortalPermissionTimeout = 30
```

### Input Behavior

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `TypingDelay` | milliseconds | 50 | Delay between keystrokes |
| `TypingSpeed` | fast, normal, slow | normal | Default typing speed |
| `ClickDelay` | milliseconds | 100 | Delay after click |
| `FocusTimeout` | milliseconds | 5000 | Wait for focus to succeed |
| `RetryAttempts` | number | 3 | Retry failed operations |
| `RetryDelay` | milliseconds | 1000 | Delay between retries |
| `VerifyInput` | yes, no | yes | Verify input succeeded |

Example:
```ini
[Input]
TypingDelay = 50
TypingSpeed = normal
ClickDelay = 100
FocusTimeout = 5000
RetryAttempts = 3
RetryDelay = 1000
VerifyInput = yes
```

### Clipboard

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `ClipboardEnabled` | yes, no | yes | Enable clipboard operations |
| `ClipboardHistory` | number | 10 | Keep N recent items |
| `ClearAfterSecure` | yes, no | yes | Clear clipboard after `type_secure` |
| `ClipboardTimeout` | seconds | 5 | Wait for clipboard operation |

Example:
```ini
[Clipboard]
ClipboardEnabled = yes
ClipboardHistory = 10
ClearAfterSecure = yes
ClipboardTimeout = 5
```

### Multi-Desktop

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `CrossDesktopEnabled` | yes, no | yes | Allow cross-desktop operations |
| `AutoSwitchDesktop` | yes, no | no | Switch desktop automatically |
| `DesktopSwitchDelay` | milliseconds | 500 | Delay after switch |
| `MultiMonitorMode` | global, per-output | global | Coordinate system |

Example:
```ini
[MultiDesktop]
CrossDesktopEnabled = yes
AutoSwitchDesktop = no
DesktopSwitchDelay = 500
MultiMonitorMode = global
```

### Task Persistence

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `PersistQueueOnShutdown` | yes, no | yes | Save pending tasks |
| `MaxHistoryAge` | seconds | 2592000 (30 days) | Retain history for N seconds |
| `AutoCleanupHistory` | yes, no | yes | Auto-delete old history |
| `CleanupInterval` | seconds | 86400 (1 day) | Cleanup schedule |

Example:
```ini
[Tasks]
PersistQueueOnShutdown = yes
MaxHistoryAge = 2592000
AutoCleanupHistory = yes
CleanupInterval = 86400
```

### Logging

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `LogLevel` | trace, debug, info, warn, error | info | Log verbosity |
| `LogFile` | file path | (stderr) | Log file location |
| `LogFormat` | json, text | text | Log format |
| `SensitiveLogging` | yes, no | no | Log sensitive data (passwords) |

Example:
```ini
[Logging]
LogLevel = info
LogFile = ~/.local/share/deskd/deskd.log
LogFormat = text
SensitiveLogging = no
```

### Performance

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `AtspiBatchSize` | number | 100 | AT-SPI query batch size |
| `ElementCacheTtl` | seconds | 300 | UI element cache lifetime |
| `SessionPollInterval` | seconds | 5 | Check for session changes |
| `ThreadPoolSize` | number | (auto) | Worker thread count |

Example:
```ini
[Performance]
AtspiBatchSize = 100
ElementCacheTtl = 300
SessionPollInterval = 5
ThreadPoolSize = 4
```

### Security

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `SocketPermissions` | octal | 0700 | Socket file permissions |
| `DatabasePermissions` | octal | 0600 | Database file permissions |
| `AllowRemoteConnection` | yes, no | no | Allow non-local connections |
| `RequireAuth` | yes, no | yes | Require authentication token |
| `AuditAllOperations` | yes, no | yes | Log all operations |

Example:
```ini
[Security]
SocketPermissions = 0700
DatabasePermissions = 0600
AllowRemoteConnection = no
RequireAuth = yes
AuditAllOperations = yes
```

## Example Configuration Files

### Minimal (Default Behavior)

```ini
[Session]
SessionDiscovery = auto

[Database]
Path = ~/.local/share/deskd/state.db
```

### Performance-Tuned

```ini
[Session]
SessionDiscovery = auto

[Database]
Path = ~/.local/share/deskd/state.db
MaxConnections = 10

[Input]
TypingDelay = 25
RetryAttempts = 5

[Performance]
ThreadPoolSize = 8
ElementCacheTtl = 600
```

### Reliable (for Flaky Systems)

```ini
[Input]
TypingDelay = 100
TypingSpeed = slow
ClickDelay = 200
RetryAttempts = 5
RetryDelay = 2000
VerifyInput = yes

[MultiDesktop]
DesktopSwitchDelay = 1000

[Wayland]
PortalPermissionTimeout = 60
```

### Debug

```ini
[Logging]
LogLevel = debug
LogFile = ~/.local/share/deskd/deskd.log

[Input]
VerifyInput = yes
RetryAttempts = 5
```

## Command-Line Overrides

Override config file settings with command-line arguments:

```bash
deskd --user \
  --log-level debug \
  --typing-delay 100 \
  --primary-desktop kde-wayland-0 \
  --database-path /custom/path/state.db
```

## Environment Variables

Configure via environment variables (overrides config file):

```bash
DESKD_LOG_LEVEL=debug
DESKD_PRIMARY_DESKTOP=gnome-wayland-0
DESKD_TYPING_DELAY=100
deskd --user
```

Common variables:
- `DESKD_LOG_LEVEL` - Log verbosity
- `DESKD_PRIMARY_DESKTOP` - Primary desktop ID
- `DESKD_TYPING_DELAY` - Milliseconds between keystrokes
- `DESKD_CLICK_DELAY` - Milliseconds after click
- `DESKD_DATABASE_PATH` - Database file location
- `DESKD_SOCKET_PATH` - Unix socket path
- `DESKD_WAYLAND_COMPOSITOR` - Override compositor detection

## Verifying Configuration

Check loaded configuration:

```bash
# Show active configuration
deskctl config show

# Show config file path
deskctl config path

# Validate config file
deskctl config validate
```

Reload configuration without restart:

```bash
deskctl config reload
```

## Troubleshooting

**Config file not found**
```bash
# Create default config
mkdir -p ~/.config/deskd
echo "[Session]" > ~/.config/deskd/deskd.conf
```

**Settings not taking effect**
```bash
# Restart daemon to load new config
systemctl --user restart deskd

# Or reload
deskctl config reload
```

**Find active configuration**
```bash
deskctl config show
```

---

See [INSTALLATION.md](./INSTALLATION.md) for setup and [DEVELOPMENT.md](./DEVELOPMENT.md) for advanced configuration options.
