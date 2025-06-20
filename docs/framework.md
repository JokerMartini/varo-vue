# 🧭 Varo Overview

This document outlines the initialization sequence and architecture of the application.

---

## Application Flow

### Boot Sequence

The application follows a structured initialization process that establishes system-level caching and configuration loading before exposing functionality to the frontend.

#### Phase 1: System Initialization (One-time Setup)

During application startup, the following system-level operations are performed and cached for the duration of the session:

- **System User Context** - Retrieves and caches the current system username
- **Platform Detection** - Identifies and caches operating system information  
- **Environment Context** - Captures and caches relevant system environment variables
- **Global State Management** - Initializes a thread-safe global `AppState` within a `Mutex` for concurrent access
- **Configuration Bootstrap** - Loads and validates the primary `config.json` file (see [`config.md`](./config.md) for schema details)

#### Phase 2: Configuration & Preset Loading

Once the system foundation is established, the application loads environment-specific configurations:

- **Environment Preset Discovery** - Scans and loads all available EnvPresets from locations specified in `config.json`
- **Default Environment Selection** - Automatically selects an initial EnvPreset when:
  - A default is explicitly defined in `config.json` 
  - The specified EnvPreset exists and is accessible on disk
- **Node Discovery & Loading** - Discovers and loads executable Nodes from the `VARO_PATH` environment variable, sourced from either:
  - The currently active EnvPreset configuration
  - System environment variables (as fallback when no preset is selected)

### Frontend Integration

The backend exposes a set of Tauri commands that allow the frontend to interact with the application state and trigger operations.

#### Available Commands

| Command | Parameters | Purpose | Behavior |
|---------|------------|---------|----------|
| `get_username` | None | Retrieves cached system username | Returns string value from Phase 1 initialization |
| `get_platform` | None | Retrieves operating system identifier | Returns cached OS information |
| `get_config` | None | Exposes current configuration state | Returns serialized JSON of active config (debugging aid) |
| `get_env_presets` | None | Lists all discovered environment presets | Returns complete collection of available EnvPresets |
| `set_selected_env_preset` | `id: string` | Changes active environment preset | Switches to specified preset and re-triggers Node discovery (Phase 2 refresh) |
| `refresh_data` | None | Reloads configuration and preset data | **TODO**: Full refresh of Phase 2 operations without restart |
| `execute_node` | `id: string` | Executes specified Node's command sequence | Runs the command(s) associated with the given Node ID |

#### Command Flow Patterns

**Initial Load Pattern:**
1. Frontend calls `get_env_presets()` to populate environment selector
2. Frontend calls other `get_*` commands to populate initial UI state
3. User interaction drives subsequent `set_selected_env_preset()` and `execute_node()` calls

**Environment Switching Pattern:**
1. User selects different environment via frontend
2. Frontend calls `set_selected_env_preset(id)` 
3. Backend re-runs Phase 2 with new preset context
4. Frontend refreshes Node listings and related UI elements

**Node Execution Pattern:**
1. User triggers Node execution via frontend
2. Frontend calls `execute_node(id)`
3. Backend executes associated command sequence
4. Results/status communicated back to frontend (implementation dependent)
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