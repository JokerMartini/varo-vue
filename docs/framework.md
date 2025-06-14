# Varo App Startup Flow

This document outlines how the Varo Tauri application initializes its core state and prepares environment data for use by the frontend and CLI.

---

## 🧭 Overview

The application follows this sequence:

1. App initializes
2. Loads `config.json` (see [`config.md`](./config.md) for format)
3. Extracts startup environment variables
4. Loads all environment presets based on config-defined directories
5. Instantiates and manages a global `AppState` inside a `Mutex`
6. Exposes Tauri commands (e.g., `get_env_presets`) to the frontend

---

## 🧱 AppState Structure

The global state of the application is held in:

```rust
pub struct AppState {
    pub config: Value,
    pub env_vars: HashMap<String, String>,
    pub env_presets: Vec<EnvPreset>,
}
