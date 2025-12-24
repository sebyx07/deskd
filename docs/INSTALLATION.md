# Installation & Setup 📦

## System Requirements

- **OS**: Linux (kernel 5.10+)
- **Display Server**: Wayland or X11
- **Init System**: systemd
- **Accessibility**: AT-SPI (bundled with GTK/Qt)
- **Build**: Rust 1.70+ (for source builds only)

## Installing from Package

### Ubuntu/Debian

```bash
# Add repository (if applicable)
sudo apt update

# Install deskd and deskctl
sudo apt install deskd

# Enable user service
systemctl --user enable deskd
systemctl --user start deskd

# Verify installation
deskctl --version
```

### Fedora/RHEL

```bash
sudo dnf install deskd
systemctl --user enable deskd
systemctl --user start deskd
```

### Arch Linux

```bash
sudo pacman -S deskd
systemctl --user enable deskd
systemctl --user start deskd
```

## Building from Source

### Prerequisites

Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Install system dependencies:

```bash
# Debian/Ubuntu
sudo apt install libsqlite3-dev libdbus-1-dev libatspi2.0-dev

# Fedora
sudo dnf install sqlite-devel dbus-devel at-spi2-core-devel

# Arch
sudo pacman -S sqlite dbus at-spi2-core
```

### Build Steps

```bash
# Clone repository
git clone https://github.com/sebyx07/deskd
cd deskd

# Build
cargo build --release

# Run tests
cargo test --release

# Install
cargo install --path .
```

The binary will be installed to `~/.cargo/bin/deskd` and `~/.cargo/bin/deskctl`.

## First-Time Setup

### 1. Start the Daemon

```bash
# Per-user daemon (recommended)
systemctl --user start deskd

# Or run directly for debugging
deskd --user --log-level debug
```

### 2. Grant Permissions (Wayland)

On first run, you may see a permission dialog:

```
"deskd wants to:
  □ Record your desktop
  □ Control your desktop"
```

Click **Allow** to grant permissions. This is a one-time setup.

**Alternative**: Pre-grant permissions via dconf:

```bash
# For GNOME
dconf write /org/gnome/desktop/remote-access/require-encryption false

# Or use portal settings
deskctl permissions request remote-desktop
deskctl permissions request screen-capture
```

### 3. Verify Installation

```bash
# Check daemon status
systemctl --user status deskd

# List active desktops
deskctl desktop list

# Take a screenshot
deskctl screenshot

# Type text
deskctl type "Hello World"

# Check daemon logs
journalctl --user -u deskd -f
```

## Configuration

The daemon uses a configuration file at:
- **Per-user**: `~/.config/deskd/deskd.conf`
- **System-wide**: `/etc/deskd/deskd.conf`

See [CONFIGURATION.md](./CONFIGURATION.md) for available options.

## Database Initialization

The SQLite database is created automatically on first run:
- **Per-user**: `~/.local/share/deskd/state.db`
- **System-wide**: `/var/lib/deskd/state.db`

Check database status:

```bash
deskctl db version
deskctl db stats
```

## Next Steps

- [Quick Start Guide](./QUICK_START.md) - Common tasks
- [Configuration](./CONFIGURATION.md) - Customize behavior
- [CLI Reference](./CLI_REFERENCE.md) - All available commands

## Troubleshooting

### Daemon won't start

Check logs:
```bash
journalctl --user -u deskd -n 50
```

Common issues:
- Missing AT-SPI server: Install `at-spi2-core`
- Wrong Wayland compositor: See [DEPLOYMENT.md](./DEPLOYMENT.md)
- Port already in use: Check for existing daemon instances

### Permissions dialog not appearing

If you don't see the permissions dialog:

```bash
# Check if permissions already granted
deskctl permissions status

# Or request manually
deskctl permissions request remote-desktop
```

### Operations fail silently

Enable debug logging:

```bash
# Restart with debug output
deskctl --debug type "test"

# Or check daemon logs
journalctl --user -u deskd -f
```

### AT-SPI not working

Verify AT-SPI is running:

```bash
ps aux | grep at-spi
```

If not running, restart the session or:

```bash
eval "$(/usr/lib/at-spi2-core/at-spi2-bus-launcher --launch-immediately)"
```

## Uninstalling

```bash
# Stop daemon
systemctl --user stop deskd

# Disable autostart
systemctl --user disable deskd

# Remove package
sudo apt remove deskd

# Remove user files (optional)
rm -rf ~/.local/share/deskd
rm -rf ~/.config/deskd
```

## Upgrading

```bash
# Update package
sudo apt update && sudo apt upgrade deskd

# Restart daemon
systemctl --user restart deskd

# Verify upgrade
deskctl --version
```

The database schema will migrate automatically if needed.

---

Next: [Quick Start Guide](./QUICK_START.md)
