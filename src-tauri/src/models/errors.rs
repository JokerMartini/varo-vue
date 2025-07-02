use std::fmt;

#[derive(Debug, Clone)]
pub enum VaroError {
    ConfigError(String),
    EnvPresetError(String),
    NodeError(String),
    SystemError(String),
    IoError(String),
    SerializationError(String),
    LockError(String),
    ExecutionError(String),
}

impl fmt::Display for VaroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VaroError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            VaroError::EnvPresetError(msg) => write!(f, "Environment preset error: {}", msg),
            VaroError::NodeError(msg) => write!(f, "Node error: {}", msg),
            VaroError::SystemError(msg) => write!(f, "System error: {}", msg),
            VaroError::IoError(msg) => write!(f, "IO error: {}", msg),
            VaroError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            VaroError::LockError(msg) => write!(f, "Lock error: {}", msg),
            VaroError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for VaroError {}

// Conversion from std::io::Error
impl From<std::io::Error> for VaroError {
    fn from(err: std::io::Error) -> Self {
        VaroError::IoError(err.to_string())
    }
}

// Conversion from serde_json::Error
impl From<serde_json::Error> for VaroError {
    fn from(err: serde_json::Error) -> Self {
        VaroError::SerializationError(err.to_string())
    }
}

// Helper for converting String errors (during migration)
impl From<String> for VaroError {
    fn from(err: String) -> Self {
        VaroError::SystemError(err)
    }
}

// Conversion to String (for Tauri commands)
impl From<VaroError> for String {
    fn from(err: VaroError) -> Self {
        err.to_string()
    }
}

pub type VaroResult<T> = Result<T, VaroError>;

// Helper methods for creating specific error types
impl VaroError {
    pub fn config<T: Into<String>>(msg: T) -> Self {
        VaroError::ConfigError(msg.into())
    }

    pub fn env_preset<T: Into<String>>(msg: T) -> Self {
        VaroError::EnvPresetError(msg.into())
    }

    pub fn node<T: Into<String>>(msg: T) -> Self {
        VaroError::NodeError(msg.into())
    }

    pub fn execution<T: Into<String>>(msg: T) -> Self {
        VaroError::ExecutionError(msg.into())
    }

    pub fn lock<T: Into<String>>(msg: T) -> Self {
        VaroError::LockError(msg.into())
    }
}