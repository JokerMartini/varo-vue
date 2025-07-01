use crate::utils::platform;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub username: String,
    pub platform: String,
}

impl SystemInfo {
    pub fn collect() -> Self {
        Self {
            username: platform::get_os_username(),
            platform: platform::get_platform(),
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_platform(&self) -> &str {
        &self.platform
    }
}