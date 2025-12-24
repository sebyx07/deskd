# Security & Permissions 🔒

deskd implements security-by-default practices. This guide covers permission models, sensitive operations, and best practices.

## Threat Model

### Assumptions

- **Per-user daemon**: User controls their own desktop; daemon isolated by systemd
- **System daemon**: Requires authentication; users cannot access other users' operations
- **Local-only**: No remote access by default; network is controlled separately
- **Trusted display server**: Wayland/X11 and compositors are trusted

### Protected Against

- ✓ Unauthorized users accessing other users' automation
- ✓ Sensitive data (passwords) appearing in logs
- ✓ Cross-session interference
- ✓ Malicious scripts accessing unconstrained methods

### Not Protected Against

- ✗ Compromised user account (runs in user context)
- ✗ Trojan X11 or Wayland server
- ✗ Compromised sudo/systemd
- ✗ Physical access to system

---

## Permission Models

### Per-User Daemon (Recommended)

No additional permissions needed. Daemon runs as the user.

```bash
# User: alice
systemctl --user start deskd

# Daemon runs with alice's privileges
ps aux | grep deskd
# alice  1234  0.0  0.1  deskd --user

# Can only access alice's desktops
# Cannot access bob's automation
```

**Socket Security**:
```bash
# Only alice can connect
ls -la ~/.local/run/deskd.sock
# srw-------  1 alice alice  deskd.sock

chmod 0700 ~/.local/run/deskd.sock  # User-only access
```

### System Daemon with Authentication

Requires tokens for all operations.

```bash
# Create authentication token
deskctl auth create --user alice --description "laptop-automation"
# Output: token_abc123xyz...

# Use token in requests
deskctl --token token_abc123xyz click "Submit"

# Token stored hashed in database
sqlite3 /var/lib/deskd/state.db "SELECT * FROM auth_tokens"
```

**Token Configuration**:
```ini
[Security]
RequireAuth = yes
TokenExpiry = 2592000  # 30 days
```

---

## Wayland Portal Permissions

Desktop automation requires permissions for sensitive operations.

### First-Time Permission Dialog

When daemon starts on Wayland, you see:

```
"deskd" wants to:
☐ Record your desktop (screen capture)
☐ Control your desktop (input/window management)
☐ Access clipboard
```

Click **Allow** to grant permissions. This is a one-time setup.

### How Permissions Work

1. **Portal Protocol** - Communicates via XDG Desktop Portal
2. **User Dialog** - System dialog shown to user (not by daemon)
3. **Permission Storage** - Saved in `~/.local/share/xdg-desktop-portal/` or similar
4. **Enforcement** - Portal enforces restrictions at Wayland level

### Pre-Grant Permissions

For headless or automated setup:

```bash
# Request specific permissions
deskctl permissions request remote-desktop
deskctl permissions request screen-capture
deskctl permissions request clipboard

# Check status
deskctl permissions status

# Revoke if needed
deskctl permissions revoke remote-desktop
```

### Permission Scopes

| Permission | Allows | Requires |
|-----------|--------|----------|
| `remote-desktop` | Input simulation, window control | Portal permission |
| `screen-capture` | Screenshots, element discovery | Portal permission |
| `clipboard` | Read/write clipboard | Portal permission (varies) |

---

## Sensitive Data Protection

### Secure Typing

Never log passwords or sensitive input.

```bash
# Regular typing - logged
deskctl type "public data"
# Appears in:
# - Daemon logs (with full text)
# - Task history (with full text)
# - Audit logs

# Secure typing - memory zeroed, not logged
deskctl type --secure "password123"
# Appears in:
# - Daemon logs: "[secure input of N characters]"
# - Task history: "[secure input]"
# - Audit logs: "[secure input]" only
```

**Configuration**:
```ini
[Security]
SensitiveLogging = no  # Never log sensitive data
LogSecureInput = false  # Don't log secure type calls
```

### Clipboard Handling

Secure operations clear clipboard after use.

```bash
# Default: clipboard NOT cleared
deskctl type "public"
deskctl clipboard get  # Still contains "public"

# With secure input: clipboard cleared
deskctl type --secure "password"
deskctl clipboard get  # Returns empty

# Configure behavior
[Clipboard]
ClearAfterSecure = yes
ClipboardTimeout = 5
```

### Database Security

SQLite database stores sensitive data.

**File Permissions**:
```bash
# Should be user-only readable
ls -la ~/.local/share/deskd/state.db
# -rw-------  1 alice alice  state.db

chmod 0600 ~/.local/share/deskd/state.db
```

**What's Stored**:
- ✓ Task history (operations performed)
- ✓ Workflow definitions
- ✓ Hashed auth tokens (never plaintext)
- ✓ User preferences
- ✗ Passwords or secrets (use secure typing)

### Memory Safety

Sensitive operations use Rust's safety guarantees:
- String secrets are overwritten after use (via `zeroize` crate)
- No unbounded allocations
- No buffer overflows possible
- No use-after-free vulnerabilities

---

## Multi-User Isolation

### Per-User Daemon (Automatic)

Each user's daemon is isolated by systemd.

```bash
# User: alice
systemctl --user status deskd
# Running as alice (UID 1000)

# User: bob
systemctl --user status deskd
# Running as bob (UID 1001)

# No shared state between users
# No shared socket
# No shared database
```

### System Daemon (Requires Implementation)

System daemon must enforce permissions:

```python
# Pseudocode: System daemon request handler
def handle_request(request, user):
    # Verify user owns the target desktop
    if not user_owns_desktop(user, request.desktop):
        return Unauthorized()

    # Verify user has permission to call method
    if not user_has_permission(user, request.method):
        return Forbidden()

    # Audit operation
    audit_log(user, request.method, request.params)

    # Execute operation
    return execute(request)
```

**Audit Logging**:
```bash
# View operations by user
deskctl db history --user alice

# View all cross-user operations
SELECT * FROM task_history
WHERE user_id != client_user_id;

# Alert on suspicious activity
deskctl db history --user alice --since "1 hour ago" --count
```

---

## Best Practices

### For Users

1. **Use Per-User Daemon** (if possible)
   - Simplest and most secure
   - No configuration needed
   - Automatic isolation

2. **Use Secure Typing for Secrets**
   ```bash
   deskctl type --secure "password"  # ✓ Good
   deskctl type "password"           # ✗ Bad (logged)
   ```

3. **Review Automation Scripts**
   - Understand what workflows do
   - Don't run untrusted automation
   - Check for hardcoded credentials

4. **Rotate Auth Tokens Regularly**
   ```bash
   # Delete old token
   deskctl auth delete old-token-id

   # Create new token
   deskctl auth create --user alice
   ```

5. **Monitor Audit Logs**
   ```bash
   # Check recent operations
   deskctl db history --since "1 day ago"

   # Alert on failures
   deskctl db history --error-only
   ```

### For Administrators (System Daemon)

1. **Strong Authentication**
   ```ini
   [Security]
   RequireAuth = yes
   TokenExpiry = 604800  # 7 days
   ```

2. **Restrict Socket Access**
   ```bash
   # Only deskd group can access
   chmod 0770 /var/run/deskd.sock

   # Add authorized users to group
   usermod -a -G deskd alice
   usermod -a -G deskd bob
   ```

3. **Regular Audits**
   ```bash
   # Weekly audit report
   deskctl db export --since "7 days ago" --output audit-weekly.json
   ```

4. **Secure Configuration**
   ```bash
   chmod 0640 /etc/deskd/deskd.conf
   chown root:deskd /etc/deskd/deskd.conf
   ```

5. **Enable Security Hardening** in systemd:
   ```ini
   [Service]
   NoNewPrivileges=yes
   PrivateTmp=yes
   ProtectSystem=strict
   ProtectHome=read-only
   RestrictRealtime=yes
   RestrictSUIDSGID=yes
   LockPersonality=yes
   ```

---

## Vulnerability Disclosure

Found a security vulnerability?

1. **Do not** open public issue
2. **Email** security@example.com with details
3. **Include**: Proof of concept, impact assessment
4. **Wait** for response (typically 48 hours)
5. **Expect** coordinated disclosure timeline

---

## Security Checklist

- [ ] Using per-user daemon or authenticated system daemon
- [ ] Socket permissions are 0700 (per-user) or 0770 (system)
- [ ] Database permissions are 0600
- [ ] Using `type --secure` for passwords and sensitive input
- [ ] Regular backups of database
- [ ] Audit logs reviewed periodically
- [ ] Auth tokens rotated at least yearly
- [ ] systemd hardening enabled (system daemon)
- [ ] No hardcoded credentials in automation scripts
- [ ] Wayland portal permissions granted explicitly

---

## Related Documentation

- [Deployment Models](./DEPLOYMENT.md) - Security implications of each model
- [Configuration](./CONFIGURATION.md) - Security-related settings
- [Database](./DATABASE.md) - Audit log structure
- [Development](./DEVELOPMENT.md) - Security guidelines for contributors

---

## References

- [XDG Desktop Portal Spec](https://flatpak.github.io/xdg-desktop-portal/)
- [Wayland Security Architecture](https://wayland.freedesktop.org/security.html)
- [systemd Security Hardening](https://www.freedesktop.org/software/systemd/man/systemd.exec.html)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
