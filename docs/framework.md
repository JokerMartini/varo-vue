# 🧭 Varo Overview

This document outlines the initialization sequence and architecture of the application.

---

## Application Flow

### 1. Initial Setup (One-time Initialization)

The application performs the following system-level caching operations:

- **System Username** - Retrieves and caches the current user
- **Operating System** - Identifies and caches OS information
- **Environment Variables** - Caches system environment variables
- **Global State Management** - Instantiates and manages a global `AppState` within a `Mutex` for thread-safe access

### 2. Configuration Loading

After initial setup, the application loads configuration and preset data:

- **Configuration File** - Loads and caches `config.json` (see [`config.md`](./config.md) for detailed format specification)
- **Environment Presets** - Discovers and loads EnvPresets from disk locations specified in `config.json`
- **Default Selection** - Sets the initial selected EnvPreset if:
  - Defined in `config.json`
  - The specified EnvPreset exists on disk
- **Node Discovery** - Loads Nodes found within the `VARO_PATH` environment variable from either:
  - The currently selected EnvPreset
  - System environment variables (fallback)

### 3. Frontend API (Tauri Commands)

The application exposes the following commands to the frontend:

| Command | Parameters | Description |
|---------|------------|-------------|
| `get_username` | None | Returns the cached system username |
| `get_platform` | None | Returns the operating system name |
| `get_config` | None | Returns the current configuration as a JSON string (for debugging) |
| `get_env_presets` | None | Returns the complete list of available environment presets |
| `set_selected_env_preset` | `id: string` | Sets the active environment preset and triggers Node refresh |
| `refresh_data` | None | **TODO**: Triggers reload of configuration and preset data |
| `execute_node` | `id: string` | Executes the commands associated with the specified Node ID |

## Notes

- The `refresh_data` command (or similar) needs to be implemented to allow frontend-triggered data refresh
- Node execution requires the target Node to exist within the currently selected EnvPreset's scope
- All state modifications are thread-safe through the global `AppState` mutex

## Related Documentation

- [Configuration Format](./config.md) - Detailed `config.json` schema and examples

---

## Things To Do

- Where to we save user settings related to the front end? Do we use RUST or Vue to do that?
    - Such as Dark/Light theme
    - Default selected Preset
---

## 🧱 AppState Structure

The global state of the application is held in:

```rust
pub struct AppState {
    pub config: Value,
    pub env_vars: HashMap<String, String>,
    pub env_presets: Vec<EnvPreset>,
    ...
}



## Env Preset
- can have tokens that will get resolved