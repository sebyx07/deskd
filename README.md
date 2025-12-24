# deskd 🖥️

A production-grade Rust daemon for Linux desktop automation. Control GTK, Qt, and other applications through AT-SPI with multi-user, multi-desktop support on Wayland and X11.

**GitHub** | [Documentation](./docs) | [Architecture](./docs/ARCHITECTURE.md) | [Installation](./docs/INSTALLATION.md)

---

## Quick Start

```bash
# Install
sudo apt install deskd

# Start daemon (per-user)
systemctl --user start deskd

# Use the CLI tool
deskctl click "Submit Button"
deskctl type "Hello World"
deskctl focus "Input Field"
```

## Key Features ⚡

- **Wayland-Native** - Pure Wayland support with X11 compatibility, not the other way around
- **Multi-User** - Secure per-user daemon instances with optional system-wide mode
- **Accessibility-First** - Uses AT-SPI for semantic UI control, not vision-based
- **Multi-Desktop** - Handles multiple desktops per user (GNOME, KDE, Sway, etc.)
- **Persistent State** - SQLite database for tasks, workflows, and audit logs
- **Smart Input** - Multiple input methods with intelligent fallback (Portal → libei → ydotool)
- **Sub-100ms Latency** - Async I/O, connection pooling, and caching for speed
- **Audit Trail** - Complete operation history for debugging and compliance

## Architecture

deskd exposes three primary interfaces:

- **AT-SPI Tree** - Semantic UI element discovery and interaction
- **Wayland Protocols** - Direct desktop control (focus, input, screenshots)
- **Unix Socket** - JSON-RPC daemon API and CLI tool (`deskctl`)

See [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for details.

## Documentation

| Document | Purpose |
|----------|---------|
| [Installation](./docs/INSTALLATION.md) | Setup and system requirements |
| [Quick Start Guide](./docs/QUICK_START.md) | Common tasks and examples |
| [Configuration](./docs/CONFIGURATION.md) | Configuration file reference |
| [CLI Reference](./docs/CLI_REFERENCE.md) | deskctl command reference |
| [Database Schema](./docs/DATABASE.md) | SQLite tables and operations |
| [Deployment Models](./docs/DEPLOYMENT.md) | Per-user vs system daemon |
| [Security](./docs/SECURITY.md) | Permissions and multi-user isolation |
| [Development](./docs/DEVELOPMENT.md) | Build, test, and contribute |

## Why deskd?

### vs. Dogtail (X11-focused)
- **Wayland-native**, not X11 with Wayland hacks
- Multi-user from ground up, not bolted on
- Persistent state in SQLite
- Production daemon architecture

### vs. Computer Use APIs (Cloud-based)
- **Local execution**, no cloud dependency
- Accessibility-first, not vision-based
- Multi-session support
- Fully auditable, works offline

### vs. AskUI/Commercial
- **Free and open source**
- Standard Linux daemon conventions
- No vendor lock-in
- Community-driven

## Deployment Models

Choose the model that fits your use case:

**Per-User Daemon** (Primary, Recommended)
- Each user runs their own daemon: `systemd --user`
- Socket: `~/.local/run/deskd.sock`
- Database: `~/.local/share/deskd/state.db`
- Most secure, isolated by user boundaries

**System-Wide Daemon** (Alternative)
- Single daemon for all users
- Socket: `/var/run/deskd.sock`
- Database: `/var/lib/deskd/state.db`
- Requires privilege separation and auth

**Hybrid** (Recommended for Complex Setups)
- User daemons for desktop interaction
- Optional system daemon for coordination
- Cross-user workflow support

See [DEPLOYMENT.md](./docs/DEPLOYMENT.md) for details.

## Protocol & API

Core operations via JSON-RPC over Unix socket:

```json
{
  "method": "type",
  "params": { "text": "Hello World" }
}
```

Supported operations:
- **Input**: `type`, `type_secure`, `key_press`, `key_combo`, `paste`, `copy`
- **Clicking**: `click`, `right_click`, `double_click`, `drag`, `scroll`
- **Focus**: `focus_element`, `focus_window`, `get_focused_element`, `wait_for_focus`
- **Desktop**: `list_desktops`, `switch_desktop`, `execute_on_desktop`
- **Clipboard**: `clipboard_get`, `clipboard_set`, `clipboard_history`
- **Database**: `db_query`, `get_task_history`, `save_workflow`, `resume_workflow`

See [CLI_REFERENCE.md](./docs/CLI_REFERENCE.md) for the full protocol.

## Building from Source

```bash
git clone https://github.com/sebyx07/deskd
cd deskd
cargo build --release
cargo test
```

See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for detailed build instructions.

## Requirements

- **Linux kernel** 5.10+
- **Wayland** or X11 display server
- **systemd** for daemon management
- **AT-SPI** (usually installed with desktop environment)
- **Rust 1.70+** (for building from source)

## Status

deskd is actively under development. Current phase: Foundation & AT-SPI Integration.

**Phase 1** ✓ Foundation & project structure
**Phase 2** 🔄 AT-SPI input operations
**Phase 3** ⏳ Wayland & multi-input methods
**Phase 4** ⏳ Multi-desktop sessions
**Phase 5** ⏳ Advanced features & polish

## Contributing

Contributions welcome! Please see [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for guidelines.

## Security

deskd uses secure-by-default practices:
- Memory zeroing for sensitive operations (`type_secure`)
- Per-user isolation with systemd
- Wayland portal permission model
- Audit trail in SQLite
- No direct sudo requirement

See [SECURITY.md](./docs/SECURITY.md) for details.

## Author

**Sebi** (@sebyx07) - sebyx07.pro@gmail.com

## License

(Add your license here)

## Support

- **Issues**: [GitHub Issues](https://github.com/sebyx07/deskd/issues)
- **Discussions**: [GitHub Discussions](https://github.com/sebyx07/deskd/discussions)
- **Documentation**: [./docs](./docs)

---

Made with ❤️ for the Linux desktop automation community
