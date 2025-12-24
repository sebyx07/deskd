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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Response {
    Success { message: String },
    Error { error: String },
    Data { data: serde_json::Value },
}
