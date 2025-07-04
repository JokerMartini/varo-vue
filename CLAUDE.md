# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Varo is a Tauri-based desktop application that provides a launcher interface for software nodes/applications. It combines a Vue.js frontend with a Rust backend, managing environment presets and executing various software tools.

## Development Commands

### Frontend (Vue/Nuxt)
- `pnpm dev` - Start development server
- `pnpm build` - Build production frontend
- `pnpm generate` - Generate static site
- `pnpm preview` - Preview production build

### Backend (Rust/Tauri)
- `pnpm tauri dev` - Start development with Tauri backend
- `pnpm tauri build` - Build production application
- `cargo build` - Build Rust backend only (from src-tauri directory)
- `cargo test` - Run Rust tests (from src-tauri directory)

### Prerequisites
- Node.js and pnpm package manager
- Rust toolchain
- Set `VARO_PATH` environment variable pointing to test-data folder

## Architecture

### Frontend (Vue/Nuxt)
- **Framework**: Vue 3 with Nuxt 3, TypeScript, Tailwind CSS
- **State Management**: Pinia store (`src/stores/varoNodes.ts`)
- **Key Models**: `VaroNode`, `VaroNodeGroup`, `VaroCategory`, `EnvPreset`
- **Components**: Modular Vue components in `src/components/`
- **Main Store**: `useAppStore` manages nodes, groups, categories, and environment presets

### Backend (Rust/Tauri)
- **Core Module**: `src-tauri/src/core/` contains main business logic
- **Global State**: Thread-safe `AppState` with Mutex for concurrent access
- **Key Managers**: 
  - `config::manager` - Configuration loading/management
  - `env_presets::manager` - Environment preset handling
  - `nodes::manager` - Node discovery and execution
  - `system::info` - System information utilities

### Application Flow
1. **System Initialization**: Cache username, platform, environment variables
2. **Configuration Loading**: Load `config.json` and discover environment presets
3. **Node Discovery**: Load executable nodes from `VARO_PATH` based on selected preset
4. **Frontend Integration**: Expose functionality through Tauri commands

### Key Tauri Commands
- `get_username()` - Retrieve cached system username
- `get_platform()` - Get operating system identifier
- `get_config()` - Return current configuration state
- `get_env_presets()` - List available environment presets
- `set_selected_env_preset(id)` - Switch active environment preset
- `execute_node(id)` - Execute specified node's command sequence

## Code Organization

### Frontend Structure
- `src/pages/` - Nuxt pages (main app at `index.vue`)
- `src/components/` - Vue components (cards, modals, lists)
- `src/models/` - TypeScript data models
- `src/stores/` - Pinia stores for state management
- `src/utils/` - Utility functions (node grouping, etc.)

### Backend Structure
- `src-tauri/src/core/` - Core business logic modules
- `src-tauri/src/models/` - Rust data structures and error types
- `src-tauri/src/utils/` - Utility functions and helpers
- `src-tauri/src/lib.rs` - Main Tauri application setup

### Configuration
- `config.json` - Main application configuration
- `test-data/envs/` - Environment preset definitions
- `test-data/nodes/` - Node definitions and scripts
- Environment variables resolved through preset system

## Development Notes

### State Management
- Frontend uses Pinia for reactive state management
- Backend uses global `AppState` with Mutex for thread safety
- Environment switching triggers re-discovery of nodes

### Node System
- Nodes are discovered from `VARO_PATH` environment variable
- Grouped by category and can be organized hierarchically
- Support for visibility toggling and search filtering
- Each node has metadata (name, category, icon, description, status)

### Environment Presets
- JSON-based configuration files defining environment variables
- Support for token resolution within preset definitions
- Automatic selection of default preset on startup
- Frontend dropdown for preset switching