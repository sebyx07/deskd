// Protocol definitions and message serialization
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Request {
    // Input operations
    Type { text: String },
    TypeSecure { text: String },
    Click { x: i32, y: i32 },

    // Focus operations
    Focus { element_id: String },

    // Desktop operations
    ListDesktops,

    // Clipboard operations
    ClipboardGet,
    ClipboardSet { content: String },

    // Database queries
    GetTaskHistory { limit: Option<usize> },

    // AT-SPI Element operations (Phase 2)
    FindElement {
        name: Option<String>,
        role: Option<String>,
    },
    ClickElement {
        name: String,
        button: Option<String>,
    },
    DoubleClickElement {
        name: String,
    },
    TypeIntoElement {
        name: String,
        text: String,
        secure: Option<bool>,
    },
    FocusElement {
        name: String,
    },
    GetFocusedElement,

    // Wayland operations (Phase 3)
    KeyPress {
        key: String,
    },
    KeyCombo {
        combo: String, // e.g., "Ctrl+C"
    },
    KeySequence {
        keys: Vec<String>,
    },
    ClickAt {
        x: i32,
        y: i32,
        button: Option<String>, // "left", "right", "middle"
    },
    Drag {
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
    },
    Screenshot {
        region: Option<String>, // "fullscreen", "window", or "selection"
        include_cursor: Option<bool>,
    },
    DetectCompositor,
    GetCapabilities,
    ClipboardHistory {
        limit: Option<usize>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Response {
    Success { message: String },
    Error { error: String },
    Data { data: serde_json::Value },
}
