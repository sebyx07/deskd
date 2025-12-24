// Protocol definitions and message serialization
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
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
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Success { message: String },
    Error { error: String },
    Data { data: serde_json::Value },
}
