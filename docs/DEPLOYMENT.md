# Deployment Models 🚀

deskd supports multiple deployment architectures. Choose the model that fits your use case.

## Comparison

| Model | Use Case | Security | Complexity | Isolation |
|-------|----------|----------|-----------|-----------|
| **Per-User** | Single user, desktop automation | ⭐⭐⭐⭐⭐ | ⭐ | Per-user |
| **System-Wide** | Multi-user server, shared automation | ⭐⭐⭐ | ⭐⭐⭐⭐ | Per-user + auth |
| **Hybrid** | Complex multi-session setup | ⭐⭐⭐⭐ | ⭐⭐⭐ | Per-session |

## Per-User Daemon (Recommended)

Each user runs their own daemon instance.

### Architecture

```
User: alice                  User: bob
    ↓                           ↓
~/.local/run/deskd.sock    ~/.local/run/deskd.sock
    ↓                           ↓
~/.local/share/deskd/      ~/.local/share/deskd/
state.db                   state.db
```

### Configuration

**systemd User Service** (`~/.config/systemd/user/deskd.service`):

```ini
[Unit]
Description=Desktop Automation Daemon (User)
After=graphical-session.target
PartOf=graphical-session.target

[Service]
Type=notify
ExecStart=/usr/bin/deskd --user
Restart=on-failure
RestartSec=5s

# Wait for Wayland compositor
ExecStartPre=/bin/sh -c 'while [ -z "$WAYLAND_DISPLAY" ]; do sleep 0.5; done'

Environment=XDG_RUNTIME_DIR=%t
Environment=DBUS_SESSION_BUS_ADDRESS=unix:path=%t/bus

[Install]
WantedBy=graphical-session.target
```

### Locations

```
Socket:     ~/.local/run/deskd.sock (user-only)
Database:   ~/.local/share/deskd/state.db (user-only)
Config:     ~/.config/deskd/deskd.conf
Logs:       systemd journal or ~/.local/share/deskd/deskd.log
```

### Setup

```bash
# Enable per user
systemctl --user enable deskd

# Start daemon
systemctl --user start deskd

# Status
systemctl --user status deskd

# Logs
journalctl --user -u deskd -f
```

### Security

- ✓ No privilege escalation needed
- ✓ Isolated by user boundaries
- ✓ No shared state with other users
- ✓ Socket permissions: `0700` (user-only)
- ✓ Database permissions: `0600` (user-only)

### Limitations

- Can only control desktops owned by same user
- Doesn't support cross-user automation
- Separate daemon per user (more resource usage)

### Recommended For

- Personal desktop automation
- Single-user systems
- Development and testing
- Security-sensitive environments

---

## System-Wide Daemon (Advanced)

Single daemon instance for all users.

### Architecture

```
User: alice          User: bob          User: charlie
    ↓                   ↓                   ↓
Shared Unix Socket: /var/run/deskd.sock
    ↓
System Daemon (deskd system)
    ↓
Shared Database: /var/lib/deskd/state.db
    │
    ├─→ Desktop: alice:0 (GNOME)
    ├─→ Desktop: bob:1 (KDE)
    └─→ Desktop: charlie:2 (Sway)
```

### Configuration

**System Service** (`/lib/systemd/system/deskd.service`):

```ini
[Unit]
Description=Desktop Automation Daemon (System)
After=dbus.service
Requires=dbus.service

[Service]
Type=notify
User=deskd
ExecStart=/usr/bin/deskd --system
Restart=on-failure
RestartSec=5s

# Security hardening
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=read-only

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

### Locations

```
Socket:     /var/run/deskd.sock (system, restricted access)
Database:   /var/lib/deskd/state.db (system, restricted access)
Config:     /etc/deskd/deskd.conf
Logs:       systemd journal
```

### Setup

```bash
# Create deskd system user
sudo useradd -r -s /usr/sbin/nologin deskd

# Create directories
sudo mkdir -p /var/lib/deskd /var/run/deskd
sudo chown deskd:deskd /var/lib/deskd /var/run/deskd
sudo chmod 0750 /var/lib/deskd /var/run/deskd

# Enable system daemon
sudo systemctl enable deskd

# Start daemon
sudo systemctl start deskd

# Status
sudo systemctl status deskd
```

### Security

- Requires careful privilege handling
- Must authenticate socket connections
- Must verify user permissions
- Should use D-Bus for multi-session support
- Needs security review for production

**Socket Access Control**:
```ini
[Security]
SocketPermissions = 0770  # Group readable for authorized users
AllowRemoteConnection = no
RequireAuth = yes
AuditAllOperations = yes
```

**User Authentication**:
```bash
# Create auth token for user
deskctl auth create --user alice --description "alice-automation"

# Use token in requests
deskctl --token <token> click "Submit"
```

### Operational Considerations

- Single point of failure (use systemd Restart)
- Shared database across users (careful concurrency)
- Cross-user operations possible (audit required)
- More resource efficient than per-user
- Easier to manage at scale

### Limitations

- Higher complexity
- Requires security review
- Single daemon crash affects all users
- Shared state management complexity

### Recommended For

- Multi-user servers
- Kiosk-like environments
- Automation at scale
- Managed enterprise deployments

---

## Hybrid Model (Recommended for Complex Setups)

Combines per-user and system daemons for flexibility.

### Architecture

```
User: alice                          User: bob
    ↓                                  ↓
Per-User Daemon                    Per-User Daemon
(~/.local/run/deskd.sock)         (~/.local/run/deskd.sock)
    │                                  │
    └──────────┬───────────────────────┘
               ↓
    System Daemon (optional)
    /var/run/deskd.sock
    (Coordination & Cross-User Workflows)
```

### Configuration

**Per-User Service** (`~/.config/systemd/user/deskd.service`):

```ini
[Unit]
Description=Desktop Automation Daemon
After=graphical-session.target

[Service]
Type=notify
ExecStart=/usr/bin/deskd --user
Restart=on-failure

[Install]
WantedBy=graphical-session.target
```

**System Coordinator** (`/lib/systemd/system/deskd-system.service`):

```ini
[Unit]
Description=Desktop Automation System Coordinator
After=dbus.service

[Service]
Type=notify
User=deskd
ExecStart=/usr/bin/deskd --system --coordinator-only
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

### Setup

```bash
# Enable per-user daemon
systemctl --user enable deskd

# Enable system coordinator (optional)
sudo systemctl enable deskd-system
sudo systemctl start deskd-system

# Start user daemon
systemctl --user start deskd
```

### Benefits

- Per-user isolation (security)
- Optional system coordination
- Cross-user workflows when needed
- Graceful degradation (works without system daemon)
- Resource efficient

### Recommended For

- Complex multi-desktop environments
- Need for both isolation and coordination
- Large organizations
- Flexible deployment scenarios

---

## Multi-Desktop Support (All Models)

All deployment models support multiple desktops per user.

### Example: GNOME and KDE on Same User

```bash
# Start GNOME session
DISPLAY=:0 gnome-session &

# Start KDE session
DISPLAY=:1 startkde &

# List desktops
deskctl desktop list
# Output:
# gnome-wayland-0
# kde-wayland-1

# Execute on specific desktop
deskctl --desktop kde-wayland-1 click "Submit"
```

### Service Template (System Daemon)

For system daemon with multiple desktops:

```ini
# /lib/systemd/system/deskd@.service
[Unit]
Description=Desktop Automation for %I
After=graphical-session.target

[Service]
Type=notify
User=%i
ExecStart=/usr/bin/deskd --user --desktop %I
Restart=on-failure

[Install]
WantedBy=graphical-session.target
```

Usage:
```bash
systemctl --user start deskd@gnome
systemctl --user start deskd@kde
```

---

## Switching Between Models

### Per-User → System-Wide

```bash
# Stop per-user daemon
systemctl --user stop deskd
systemctl --user disable deskd

# Setup system daemon
sudo mkdir -p /var/lib/deskd
sudo chown deskd:deskd /var/lib/deskd
sudo systemctl enable deskd
sudo systemctl start deskd

# Migrate database
deskctl db export --output backup.json
sudo deskctl db import backup.json
```

### System-Wide → Per-User

```bash
# Stop system daemon
sudo systemctl stop deskd
sudo systemctl disable deskd

# Export database
deskctl db export --output backup.json

# Enable per-user daemon
systemctl --user enable deskd
systemctl --user start deskd

# Import database
deskctl db import backup.json
```

---

## Troubleshooting

### Daemon Won't Start

Check logs:
```bash
# Per-user
journalctl --user -u deskd -n 50

# System-wide
sudo journalctl -u deskd -n 50
```

Common issues:
- No Wayland compositor: Wait for desktop to fully load
- Database locked: Stop other daemon instances
- Permissions denied: Check file ownership and permissions

### Connection Issues

```bash
# Check socket exists
ls -la ~/.local/run/deskd.sock

# Check socket permissions
stat ~/.local/run/deskd.sock

# Try explicit socket path
deskctl --socket ~/.local/run/deskd.sock click "Submit"
```

### Multi-Session Conflicts

```bash
# List active sessions
deskctl desktop list --verbose

# Kill stale sessions
systemctl --user restart deskd
```

---

See [INSTALLATION.md](./INSTALLATION.md) for setup and [SECURITY.md](./SECURITY.md) for security considerations.
