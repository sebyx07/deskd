# CLI Reference 📋

Complete reference for the `deskctl` command-line tool.

## Global Options

```bash
deskctl [OPTIONS] <COMMAND> [ARGS]
```

| Option | Description |
|--------|-------------|
| `--version` | Show version information |
| `--help` | Show help message |
| `--debug` | Enable debug logging |
| `--log-level <LEVEL>` | Set log level: trace, debug, info, warn, error |
| `--socket <PATH>` | Custom socket path (default: `~/.local/run/deskd.sock`) |
| `--desktop <ID>` | Execute on specific desktop (e.g., `gnome-wayland-0`) |
| `--retry <N>` | Retry failed operations N times (default: 1) |
| `--timeout <MS>` | Operation timeout in milliseconds |

## Input Commands

### type

Type text into the focused element.

```bash
deskctl type [OPTIONS] <TEXT>
```

| Option | Description |
|--------|-------------|
| `--secure` | Don't log content (for passwords) |
| `--speed <SPEED>` | Typing speed: fast, normal, slow (default: normal) |
| `--delay <MS>` | Delay between keystrokes (overrides speed) |
| `--verify` | Verify text was actually typed |

Examples:
```bash
deskctl type "Hello World"
deskctl type --secure "password123"
deskctl type --speed slow "Very important text"
deskctl type --delay 100 "Slow typing"
deskctl type --verify "Check text"
```

### key

Press keyboard keys and combinations.

```bash
deskctl key [OPTIONS] <KEY>
```

| Option | Description |
|--------|-------------|
| `--sequence` | Press multiple keys in sequence |
| `--hold <MS>` | Hold key for N milliseconds |

Examples:
```bash
deskctl key "Escape"
deskctl key "Return"
deskctl key "Tab"
deskctl key "Ctrl+C"
deskctl key "Alt+F4"
deskctl key "Ctrl+Shift+T"
deskctl key --sequence "Alt+F" "Down" "Return"
deskctl key --hold 1000 "Control"
```

Common keys: `Escape`, `Return`, `Tab`, `Backspace`, `Delete`, `Home`, `End`, `PageUp`, `PageDown`, `Insert`, `PrintScreen`

### paste

Paste from clipboard to focused element.

```bash
deskctl paste [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--clear-first` | Clear field before pasting |

Examples:
```bash
deskctl paste
deskctl paste --clear-first
```

### copy

Copy focused element's text to clipboard.

```bash
deskctl copy
```

### select_all

Select all text in focused element.

```bash
deskctl select_all
```

## Clicking Commands

### click

Click an element or coordinates.

```bash
deskctl click [OPTIONS] <ELEMENT|COORDS>
```

| Option | Description |
|--------|-------------|
| `--button <BUTTON>` | Which button: left, right, middle (default: left) |
| `--coords <X,Y>` | Click at coordinates instead of element |
| `--verify` | Verify click succeeded |
| `--wait <MS>` | Wait for element to be clickable |
| `--offset <X,Y>` | Offset from element center |

Examples:
```bash
deskctl click "Submit"
deskctl click --button right "File Menu"
deskctl click --coords 150,200
deskctl click --verify "Submit"
deskctl click --wait 2000 "Loading button"
deskctl click --offset 10,10 "Button"
```

### double-click

Double-click an element.

```bash
deskctl double-click <ELEMENT|COORDS>
```

Examples:
```bash
deskctl double-click "folder.txt"
deskctl double-click --coords 100,100
```

### right-click

Right-click (context menu) on element.

```bash
deskctl right-click <ELEMENT|COORDS>
```

Examples:
```bash
deskctl right-click "File"
deskctl right-click --coords 200,300
```

### middle-click

Middle-click on element.

```bash
deskctl middle-click <ELEMENT|COORDS>
```

### drag

Drag from one position to another.

```bash
deskctl drag [OPTIONS] <FROM_COORDS> <TO_COORDS>
```

| Option | Description |
|--------|-------------|
| `--duration <MS>` | Duration of drag in milliseconds |
| `--speed <SPEED>` | Drag speed: fast, normal, slow |

Examples:
```bash
deskctl drag 100,100 200,200
deskctl drag --duration 500 100,100 200,200
deskctl drag --speed slow 100,100 300,100
```

### scroll

Scroll in a direction.

```bash
deskctl scroll [OPTIONS] <DIRECTION> [AMOUNT]
```

| Option | Description |
|--------|-------------|
| `--element <NAME>` | Scroll specific element |
| `--amount <N>` | Number of scroll clicks (default: 1) |

Examples:
```bash
deskctl scroll up
deskctl scroll down 3
deskctl scroll --element "list" right
deskctl scroll --element "list" up 5
```

## Focus Commands

### focus

Focus an element.

```bash
deskctl focus [OPTIONS] <ELEMENT>
```

| Option | Description |
|--------|-------------|
| `--window <NAME>` | Focus window by name instead |
| `--app <NAME>` | Focus application by name |
| `--wait <MS>` | Wait for element to be focusable |
| `--get` | Get currently focused element |

Examples:
```bash
deskctl focus "Username"
deskctl focus --window "Firefox"
deskctl focus --app "gedit"
deskctl focus --wait 2000 "Input"
deskctl focus --get
```

### tab

Tab to next focusable element.

```bash
deskctl tab [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--backward` | Shift+Tab (previous element) |
| `--count <N>` | Tab N times |

Examples:
```bash
deskctl tab
deskctl tab --backward
deskctl tab --count 3
```

## Clipboard Commands

### clipboard

Clipboard operations.

```bash
deskctl clipboard <SUBCOMMAND> [ARGS]
```

Subcommands:
- `get` - Read clipboard
- `set <TEXT>` - Write to clipboard
- `history` - Show recent clipboard items
- `clear` - Clear clipboard

Examples:
```bash
deskctl clipboard get
deskctl clipboard set "Copy this"
deskctl clipboard history
deskctl clipboard clear
```

## Desktop Commands

### desktop

Multi-desktop operations.

```bash
deskctl desktop <SUBCOMMAND> [ARGS]
```

Subcommands:
- `list` - List active desktops
- `current` - Show current desktop
- `set-primary <ID>` - Set primary desktop
- `switch <ID>` - Switch to desktop

Examples:
```bash
deskctl desktop list
deskctl desktop current
deskctl desktop set-primary gnome-wayland-0
deskctl desktop switch kde-wayland-0
```

## Screenshot Commands

### screenshot

Capture screen or window.

```bash
deskctl screenshot [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-o, --output <FILE>` | Output file path |
| `--window <NAME>` | Capture specific window |
| `--output-monitor <NAME>` | Capture specific monitor |
| `--include-cursor` | Include mouse cursor |
| `--region <X,Y,W,H>` | Capture region |

Examples:
```bash
deskctl screenshot -o screen.png
deskctl screenshot --window "Firefox" -o window.png
deskctl screenshot --output-monitor HDMI-1 -o monitor.png
deskctl screenshot --include-cursor -o with-cursor.png
deskctl screenshot --region 0,0,800,600 -o region.png
```

## Database Commands

### db

Database operations.

```bash
deskctl db <SUBCOMMAND> [ARGS]
```

Subcommands:

**history** - View operation history
```bash
deskctl db history [OPTIONS]
  --user <NAME>        Filter by user
  --since <TIME>       Since when (e.g., "1 hour ago", "2025-01-01")
  --until <TIME>       Until when
  --limit <N>          Limit results
  --success-only       Only successful operations
  --error-only         Only failed operations
```

**stats** - Database statistics
```bash
deskctl db stats
```

**version** - Check schema version
```bash
deskctl db version
```

**vacuum** - Optimize database
```bash
deskctl db vacuum
```

**cleanup** - Remove old records
```bash
deskctl db cleanup [OPTIONS]
  --older-than <TIME>  Delete records older than (e.g., "30 days")
  --confirm            Confirm without asking
```

**backup** - Backup database
```bash
deskctl db backup <OUTPUT_PATH>
```

**export** - Export to JSON
```bash
deskctl db export [OPTIONS]
  --output <FILE>      Output file
  --format <FORMAT>    Format: json, csv (default: json)
  --table <NAME>       Export specific table
```

Examples:
```bash
deskctl db history
deskctl db history --user alice --since "1 day ago"
deskctl db stats
deskctl db version
deskctl db backup backup.db
deskctl db cleanup --older-than "30 days" --confirm
deskctl db export --output audit.json
```

## Workflow Commands

### workflow

Multi-step workflow operations.

```bash
deskctl workflow <SUBCOMMAND> [ARGS]
```

Subcommands:

**save** - Save workflow definition
```bash
deskctl workflow save <FILE>
```

**list** - List all workflows
```bash
deskctl workflow list [OPTIONS]
  --user <NAME>        Filter by user
  --status <STATUS>    Filter by status: draft, active, paused, completed
```

**run** - Execute workflow
```bash
deskctl workflow run [OPTIONS] <NAME|ID>
  --dry-run            Simulate without executing
  --continue           Continue from last step
```

**pause** - Pause running workflow
```bash
deskctl workflow pause <ID>
```

**resume** - Resume paused workflow
```bash
deskctl workflow resume <ID>
```

**delete** - Delete workflow
```bash
deskctl workflow delete <ID>
```

**show** - Show workflow definition
```bash
deskctl workflow show <ID>
```

Examples:
```bash
deskctl workflow save login.json
deskctl workflow list
deskctl workflow run login
deskctl workflow run --dry-run login
deskctl workflow pause <id>
deskctl workflow resume <id>
deskctl workflow show <id>
deskctl workflow delete <id>
```

## Permissions Commands

### permissions

Permission management.

```bash
deskctl permissions <SUBCOMMAND>
```

Subcommands:
- `status` - Show current permissions
- `request <TYPE>` - Request permission
- `revoke <TYPE>` - Revoke permission

Permission types: `remote-desktop`, `screen-capture`, `clipboard`

Examples:
```bash
deskctl permissions status
deskctl permissions request remote-desktop
deskctl permissions revoke screen-capture
```

## Element Commands

### element

Find and inspect UI elements.

```bash
deskctl element <SUBCOMMAND> [ARGS]
```

Subcommands:

**search** - Find elements
```bash
deskctl element search [OPTIONS] <PATTERN>
  --type <TYPE>        Element type (button, input, window, etc.)
  --parent <ELEMENT>   Search within parent
  --limit <N>          Limit results
```

**info** - Get element info
```bash
deskctl element info <ELEMENT>
```

**tree** - Show accessibility tree
```bash
deskctl element tree [OPTIONS]
  --root <ELEMENT>     Start from element
  --depth <N>          Tree depth to show
```

Examples:
```bash
deskctl element search "submit"
deskctl element search --type button "Login"
deskctl element info "Submit"
deskctl element tree
deskctl element tree --depth 3
```

---

See [QUICK_START.md](./QUICK_START.md) for practical examples and [CONFIGURATION.md](./CONFIGURATION.md) for daemon configuration.
