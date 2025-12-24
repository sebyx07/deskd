# Development Guide 🛠️

Contribute to deskd development. This guide covers building, testing, architecture, and best practices.

## Getting Started

### Prerequisites

- **Rust 1.70+** (latest stable recommended)
- **Linux kernel 5.10+**
- **System packages** for development

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Install system dependencies:
```bash
# Debian/Ubuntu
sudo apt install \
  build-essential \
  libsqlite3-dev \
  libdbus-1-dev \
  libatspi2.0-dev \
  pkg-config

# Fedora
sudo dnf install \
  gcc \
  sqlite-devel \
  dbus-devel \
  at-spi2-core-devel \
  pkg-config

# Arch
sudo pacman -S \
  base-devel \
  sqlite \
  dbus \
  at-spi2-core \
  pkg-config
```

### Clone Repository

```bash
git clone https://github.com/sebyx07/deskd
cd deskd
```

### Build & Test

```bash
# Build
cargo build

# Build release (optimized)
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

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

---

## Project Structure

```
deskd/
├── src/
│   ├── main.rs              # Daemon entry point
│   ├── lib.rs               # Library exports
│   ├── atspi/               # AT-SPI integration
│   │   ├── mod.rs
│   │   ├── tree.rs          # Accessibility tree
│   │   └── actions.rs       # Element actions
│   ├── wayland/             # Wayland integration
│   │   ├── mod.rs
│   │   ├── input.rs         # Input simulation
│   │   ├── screenshot.rs    # Screen capture
│   │   └── compositor.rs    # Compositor detection
│   ├── database/            # SQLite layer
│   │   ├── mod.rs
│   │   ├── schema.rs        # Database schema
│   │   └── migrations.rs    # Schema migrations
│   ├── session/             # Session management
│   │   ├── mod.rs
│   │   └── discovery.rs     # Desktop discovery
│   ├── input/               # Input simulation
│   │   ├── mod.rs
│   │   ├── typing.rs        # Typing operations
│   │   ├── clicking.rs      # Click operations
│   │   └── fallback.rs      # Fallback chains
│   ├── protocol/            # JSON-RPC protocol
│   │   ├── mod.rs
│   │   ├── handler.rs       # Request handler
│   │   └── types.rs         # Protocol types
│   ├── error.rs             # Error types
│   ├── config.rs            # Configuration
│   └── log.rs               # Logging setup
├── tests/
│   ├── integration_tests.rs
│   ├── atspi_tests.rs
│   └── database_tests.rs
├── Cargo.toml               # Dependencies
├── Cargo.lock               # Locked versions
├── docs/                    # Documentation
├── examples/                # Example workflows
├── mvp/                     # MVP specifications
└── README.md
```

---

## Key Components

### AT-SPI Interface (`src/atspi/`)

Semantic UI control through accessibility tree.

**Core Responsibility**: Query and interact with applications via AT-SPI D-Bus.

Key modules:
- `tree.rs` - Parse accessibility tree, cache elements
- `actions.rs` - Perform element actions (click, type, focus)

**Key Types**:
```rust
pub struct AtspiClient {
    connection: dbus::LocalConnection,
    cache: LruCache<String, Element>,
}

pub struct Element {
    path: ObjectPath,
    role: Role,
    name: String,
    properties: HashMap<String, Value>,
}
```

**Example Usage**:
```rust
let client = AtspiClient::new()?;
let elements = client.search_by_name("Submit")?;
let button = elements.first().ok_or("Not found")?;
button.click().await?;
```

### Wayland Integration (`src/wayland/`)

Direct desktop control via Wayland protocols.

**Core Responsibility**: Input simulation, screenshots, window management.

Key modules:
- `input.rs` - Keyboard/mouse input
- `screenshot.rs` - Screen capture
- `compositor.rs` - Detect compositor type and capabilities

**Key Types**:
```rust
pub struct WaylandContext {
    display: Display,
    compositor: CompositorType,
    input_method: InputMethod,
}

pub enum CompositorType {
    Gnome,
    Kde,
    Sway,
    Hyprland,
}
```

### Session Management (`src/session/`)

Multi-desktop orchestration.

**Core Responsibility**: Track active desktops, switch between them, manage D-Bus sessions.

**Key Types**:
```rust
pub struct SessionManager {
    sessions: HashMap<String, DesktopSession>,
    database: Database,
}

pub struct DesktopSession {
    session_id: String,
    user_id: String,
    desktop_type: String,
    d_bus_address: String,
}
```

### Input Fallback Chain (`src/input/`)

Smart fallback system for input operations.

**Core Responsibility**: Try multiple input methods, fall back on failure.

**Key Types**:
```rust
pub struct InputSimulator {
    atspi: AtspiClient,
    wayland: WaylandContext,
    methods: Vec<InputMethod>,
}

pub enum InputMethod {
    AtspiAction,
    Portal,
    CompositorIpc,
    Libei,
    Ydotool,
}
```

**Flow**:
```rust
async fn type_text(&self, text: &str) -> Result<()> {
    for method in &self.methods {
        match method.type_text(text).await {
            Ok(_) => {
                log::info!("Typed via {:?}", method);
                return Ok(());
            }
            Err(e) => {
                log::warn!("Failed with {:?}: {}", method, e);
                continue;
            }
        }
    }
    Err("All methods failed".into())
}
```

### Database Layer (`src/database/`)

SQLite persistence and schema management.

**Core Responsibility**: CRUD operations, migrations, connection pooling.

**Key Types**:
```rust
pub struct Database {
    pool: ConnectionPool,
    schema_version: u32,
}

pub async fn save_task(&mut self, task: Task) -> Result<u64> {
    let params = task.to_params();
    self.pool.execute(
        "INSERT INTO tasks (user_id, method, params) VALUES (?, ?, ?)",
        params,
    ).await
}
```

### Protocol Handler (`src/protocol/`)

JSON-RPC request handling.

**Core Responsibility**: Parse requests, route to handlers, return responses.

**Request Format**:
```json
{
  "method": "type",
  "params": {
    "text": "Hello",
    "secure": false
  }
}
```

**Handler**:
```rust
async fn handle_request(&mut self, req: Request) -> Response {
    match req.method.as_str() {
        "type" => self.handle_type(req.params).await,
        "click" => self.handle_click(req.params).await,
        "focus" => self.handle_focus(req.params).await,
        _ => Response::error("Unknown method"),
    }
}
```

---

## Code Quality Standards

### Testing

Every component must have tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_search() {
        let tree = AccessibilityTree::new();
        let elements = tree.search_by_name("Submit").unwrap();
        assert!(!elements.is_empty());
    }

    #[tokio::test]
    async fn test_click_operation() {
        let client = AtspiClient::new().await.unwrap();
        let result = client.click("Submit").await;
        assert!(result.is_ok());
    }
}
```

Run tests:
```bash
cargo test
cargo test --lib  # Only unit tests
cargo test --test '*'  # Only integration tests
```

### Code Style

- Follow Rust 2021 edition conventions
- Use `cargo fmt` for formatting
- Pass `cargo clippy` without warnings
- Document public APIs with doc comments
- Use meaningful variable and function names

**Example**:
```rust
/// Find elements matching a name pattern.
///
/// # Arguments
/// * `pattern` - Name pattern to search for (case-insensitive)
///
/// # Returns
/// Vector of matching elements, sorted by relevance
pub async fn search_by_name(&self, pattern: &str) -> Result<Vec<Element>> {
    // Implementation...
}
```

### Error Handling

Always use `Result` types. Never unwrap in production code:

```rust
// ✓ Good
let result = operation()?;

// ✓ Good with context
let result = operation().context("Failed to perform operation")?;

// ✓ Good with explicit error
let result = operation().map_err(|e| Error::Custom(e.to_string()))?;

// ✗ Bad - panics in production
let result = operation().unwrap();

// ✗ Bad - ignores error
let result = operation().unwrap_or_default();
```

### Async/Await

All I/O must be async:

```rust
// ✓ Good - async I/O
pub async fn click(&self, element: &Element) -> Result<()> {
    self.wayland.simulate_click(element.coords()).await?;
    Ok(())
}

// ✗ Bad - blocking I/O
pub fn click(&self, element: &Element) -> Result<()> {
    std::thread::sleep(Duration::from_millis(100));
    Ok(())
}
```

---

## Adding a New Feature

### Example: Add `scroll` Operation

1. **Define Protocol** in `src/protocol/types.rs`:
```rust
#[derive(Debug, Deserialize)]
pub struct ScrollRequest {
    pub element: String,
    pub direction: ScrollDirection,
    pub amount: u32,
}

#[derive(Debug, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}
```

2. **Implement Handler** in `src/input/clicking.rs`:
```rust
pub async fn scroll(
    &self,
    element: &str,
    direction: ScrollDirection,
    amount: u32,
) -> Result<()> {
    let coords = self.find_element_coords(element).await?;

    for _ in 0..amount {
        self.wayland.scroll(coords, direction).await?;
    }

    Ok(())
}
```

3. **Add Protocol Handler** in `src/protocol/handler.rs`:
```rust
"scroll" => {
    let req: ScrollRequest = serde_json::from_value(params)?;
    self.input.scroll(&req.element, req.direction, req.amount).await
}
```

4. **Add CLI Command** in `src/cli/commands.rs`:
```rust
pub async fn handle_scroll(&self, args: &ScrollArgs) -> Result<()> {
    let client = DaemonClient::connect().await?;
    client.scroll(&args.element, args.direction, args.amount).await?;
    println!("Scrolled {} {} times", args.direction, args.amount);
    Ok(())
}
```

5. **Add Tests**:
```rust
#[tokio::test]
async fn test_scroll_up() {
    let input = InputSimulator::new().await.unwrap();
    let result = input.scroll("list", ScrollDirection::Up, 3).await;
    assert!(result.is_ok());
}
```

6. **Update Documentation** in `docs/CLI_REFERENCE.md`

7. **Submit PR** with all above changes

---

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -- --user
RUST_LOG=deskd=debug cargo run -- --user
RUST_LOG=trace cargo run -- --user
```

### Use Interactive Debugger

```bash
# Install debugger (gdb or lldb)
sudo apt install gdb

# Build with debug symbols
cargo build

# Run with debugger
gdb ./target/debug/deskd

# Common commands
(gdb) break main
(gdb) run
(gdb) backtrace
(gdb) print variable
(gdb) continue
```

### Check with Clippy

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Profile Performance

```bash
# Build with profiling
cargo build --release

# Run with perf
perf record ./target/release/deskd --user
perf report

# Or use flamegraph
cargo install flamegraph
cargo flamegraph
```

---

## Contributing Guidelines

1. **Fork & Branch**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make Changes**
   - Follow code style guidelines
   - Add tests for new functionality
   - Update documentation
   - Keep commits focused and descriptive

3. **Test & Lint**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt
   ```

4. **Commit**
   ```bash
   git commit -m "Add scroll operation

   - Implement scroll handler for AT-SPI and Wayland
   - Add unit tests
   - Update CLI reference
   - Fixes #123"
   ```

5. **Push & Create PR**
   ```bash
   git push origin feature/my-feature
   ```

6. **Address Feedback** - Respond to review comments

---

## Performance Optimization

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Benchmark specific function
cargo bench test_name
```

### Memory Profiling

```bash
# Use valgrind
valgrind --leak-check=full ./target/debug/deskd --user

# Or use heaptrack
heaptrack ./target/debug/deskd --user
heaptrack_gui heaptrack.deskd.3104.gz
```

### Async Runtime Tuning

deskd uses Tokio runtime. Configure in `src/main.rs`:

```rust
let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get())
    .enable_all()
    .build()?;
```

---

## Release Checklist

- [ ] All tests pass: `cargo test --release`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation updated: `docs/` and `README.md`
- [ ] Changelog updated: `CHANGELOG.md`
- [ ] Version bumped: `Cargo.toml`
- [ ] Git tags created: `git tag v0.x.x`
- [ ] Release notes written

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs)
- [AT-SPI Spec](https://wiki.gnome.org/Accessibility/AT-SPI)
- [Wayland Protocols](https://wayland.freedesktop.org/specs/)
- [SQLite Rust Crate](https://docs.rs/sqlite/)

---

## Getting Help

- **GitHub Issues**: [github.com/sebyx07/deskd/issues](https://github.com/sebyx07/deskd/issues)
- **GitHub Discussions**: [github.com/sebyx07/deskd/discussions](https://github.com/sebyx07/deskd/discussions)
- **Email**: sebyx07.pro@gmail.com
- **Documentation**: Check [docs/](./docs/) for detailed guides
- **Source Code**: Read implementation for deep understanding

---

See [ARCHITECTURE.md](./ARCHITECTURE.md) for system design and [DEPLOYMENT.md](./DEPLOYMENT.md) for deployment considerations.
