use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct VaroNode {
    pub id: String,
    pub name: String,
    pub category: String,
    pub group_id: String,
    pub icon: String, // raw embedded SVG or base64 PNG, or placeholder
    pub filepath: Option<String>,
    pub default_for_group: bool,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub access: Option<Access>,
    pub commands: Vec<Command>,
    pub env: Vec<EnvVar>,
    pub date_modified: u64,
}

#[derive(Debug, Serialize)]
pub struct Status {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct Access {
    pub platforms: Vec<String>,
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Command {
    pub path: String,
    pub path_type: String,
    pub args: String,
    pub non_blocking: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
    pub operation: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EnvPreset {
    pub id: String,
    pub name: String,
    pub filepath: Option<String>,
    pub description: Option<String>,
    pub env: Vec<EnvVar>,
}