# Quick Start Guide ⚡

Get started with deskd in 5 minutes.

## Setup (2 minutes)

```bash
# Install (or build from source)
sudo apt install deskd

# Start daemon
systemctl --user enable deskd
systemctl --user start deskd

# Verify it's running
deskctl --version
```

## Basic Operations

### Typing Text

```bash
# Type into focused element
deskctl type "Hello World"

# Type password (no logging)
deskctl type --secure "mypassword"

# Type with custom speed
deskctl type --speed slow "Careful typing"
```

### Clicking

```bash
# Click element by name
deskctl click "Submit"

# Right-click (context menu)
deskctl click --button right "File"

# Double-click
deskctl double-click "folder.txt"

# Click at coordinates
deskctl click --coords 100,200
```

### Focus Management

```bash
# Focus an element
deskctl focus "Username Field"

# Focus a window by name
deskctl focus --window "Firefox"

# Get currently focused element
deskctl focus --get
```

### Keyboard Shortcuts

```bash
# Single key
deskctl key "Escape"
deskctl key "Return"
deskctl key "Tab"

# Key combination
deskctl key "Ctrl+C"
deskctl key "Ctrl+Shift+T"
deskctl key "Alt+F4"

# Sequence of keys
deskctl key --sequence "Alt+F" "Down" "Return"
```

### Clipboard

```bash
# Read clipboard
deskctl clipboard get

# Write to clipboard
deskctl clipboard set "Copy this text"

# See clipboard history
deskctl clipboard history

# Clear clipboard
deskctl clipboard clear
```

### Screenshots

```bash
# Full screenshot
deskctl screenshot -o screen.png

# Screenshot of specific monitor
deskctl screenshot --output HDMI-1 -o monitor.png

# Screenshot of window
deskctl screenshot --window "Firefox" -o window.png
```

## Real-World Example: Login Form

```bash
# Scenario: Fill and submit a login form

# Take screenshot to see what we're working with
deskctl screenshot -o before.png

# Focus username field and type
deskctl focus "Username"
deskctl type "alice@example.com"

# Tab to password field
deskctl key "Tab"

# Type password securely
deskctl type --secure "hunter2"

# Click submit button
deskctl click "Sign In"

# Verify success (take screenshot)
deskctl screenshot -o after.png
```

## Workflows: Automate Multi-Step Tasks

Create a workflow file `login.json`:

```json
{
  "name": "Login Workflow",
  "steps": [
    {
      "action": "focus",
      "element": "Username"
    },
    {
      "action": "type",
      "text": "alice@example.com"
    },
    {
      "action": "key",
      "key": "Tab"
    },
    {
      "action": "type",
      "text": "hunter2",
      "secure": true
    },
    {
      "action": "click",
      "element": "Sign In"
    },
    {
      "action": "wait",
      "time": 2000
    },
    {
      "action": "screenshot",
      "output": "logged-in.png"
    }
  ]
}
```

Run the workflow:

```bash
# Save workflow
deskctl workflow save login.json

# Execute workflow
deskctl workflow run login

# List all workflows
deskctl workflow list

# Pause a running workflow
deskctl workflow pause <workflow-id>

# Resume a paused workflow
deskctl workflow resume <workflow-id>
```

## Multi-Desktop Support

If you have multiple desktops (GNOME and KDE, for example):

```bash
# List all active desktops
deskctl desktop list

# Get current desktop
deskctl desktop current

# Set primary desktop
deskctl desktop set-primary kde-wayland-0

# Execute operation on specific desktop
deskctl --desktop gnome-wayland-0 click "Submit"

# Switch to a desktop
deskctl desktop switch kde-wayland-0
```

## Database & History

```bash
# View recent operations
deskctl db history

# Filter by user
deskctl db history --user alice

# Filter by time
deskctl db history --since "1 hour ago"

# View task queue
deskctl db tasks

# Clear old history
deskctl db cleanup --older-than "30 days"

# Backup database
deskctl db backup backup.db
```

## Debugging

Enable debug output:

```bash
# Run command with debug logging
deskctl --debug click "Submit"

# Monitor daemon logs in real-time
journalctl --user -u deskd -f

# Increase daemon log level
deskctl --log-level debug click "Submit"
```

Check daemon status:

```bash
# Service status
systemctl --user status deskd

# Check permissions
deskctl permissions status

# List active desktop sessions
deskctl desktop list --verbose
```

## Common Patterns

### Wait for Element to Appear

```bash
# Wait up to 5 seconds for element to appear
deskctl focus --wait 5000 "Loading spinner"

# Then interact
deskctl click "Submit"
```

### Retry on Failure

```bash
# Retry up to 3 times
deskctl --retry 3 click "Submit"
```

### Combine Operations

```bash
# Focus, clear field, type new text
deskctl focus "Search"
deskctl key "Ctrl+A"
deskctl type "new search term"
deskctl key "Return"
```

## What's Next?

- [Full CLI Reference](./CLI_REFERENCE.md) - All commands and options
- [Configuration](./CONFIGURATION.md) - Customize behavior
- [Architecture](./ARCHITECTURE.md) - Understand how it works
- [Examples](../examples/) - More real-world workflows (if available)

## Tips & Tricks

- Use `deskctl --help` for command syntax
- Use `journalctl --user -u deskd` for daemon diagnostics
- Set `type-speed slow` in config for flaky input
- Increase `click-delay` if elements move during click
- Use `type --secure` for any sensitive input (passwords, PINs, credit cards)

## Troubleshooting

**"Element not found"**
- Check element name with `deskctl element --search "partial name"`
- Take screenshot to see actual UI
- AT-SPI may need to be restarted

**"Click didn't work"**
- Try taking screenshot before/after to verify
- Element might need focus first: `deskctl focus "Name" && deskctl click "Name"`
- Try waiting: `deskctl focus --wait 2000 "Name"`

**"Type didn't appear"**
- Verify focus: `deskctl focus --get`
- Try clipboard paste instead: `deskctl clipboard set "text" && deskctl key "Ctrl+V"`
- Check if field accepts input: `deskctl type --speed slow "text"`

---

See [CLI Reference](./CLI_REFERENCE.md) for the complete command reference.
