The config.json file provides a way to customize Varo's behavior without modifying the application itself. This file supports global, user-local, and per-launch overrides through a cascading configuration system.

## 📁 Supported Config Locations (in order of priority)

1. **Built-in defaults** – compiled into the app
2. **Global config** – defined via the `VARO_CONFIG_PATH` environment variable
3. **User config** – located at `~/AppData/Local/varo/config.json`
4. **Command-line config** – (planned) passed using `--config /path/to/file.json`

> 🔁 Each config layer overrides the values from lower-priority layers using deep merging. Missing values are filled in by the layer below.

## 🔧 Example `config.json`

```json
{
  "env_presets": {
    "directories": [
      "C:/Studio/VaroPresets",
      "D:/Teams/Env_presets"
    ],
    "default_id": "studio-default"
  },
  "ui": {
    "darkMode": true,
    "groups": false,
    "categories": true,
    "showHiddenNodes": false
  }
}
```

## 🧩 Configuration Reference

### `env_presets`
Settings related to environment variable presets shown in the Varo interface.

| Key           | Type       | Description                                                                         |
| ------------- | ---------- | ----------------------------------------------------------------------------------- |
| `directories` | `string[]` | A list of absolute or relative paths to folders containing `.json` env preset files |
| `default_id`   | `string`   | The ID of the preset that should be loaded by default when Varo launches            |

### `ui`
Settings that control the appearance and behavior of the Varo user interface.

| Key          | Type      | Description                                                      |
| ------------ | --------- | ---------------------------------------------------------------- |
| `darkMode`   | `boolean` | When `true`, uses dark theme. When `false`, uses light theme    |
| `groups`     | `boolean` | When `true`, nodes are grouped by their group-id                |
| `categories` | `boolean` | When `true`, enables grouping nodes by categories               |
| `showHiddenNodes` | `boolean` | When `true`, shows hidden nodes in the UI             |

## 📦 Planned/Optional Future Settings
These settings are not yet supported but may be added in future versions:

| Key              | Description                                            |
| ---------------- | ------------------------------------------------------ |
| `nodeGroups`     | Custom group definitions for apps (e.g. by department) |

## 🛠️ How to Use
### ✅ Studio-Wide (Global)
Set an environment variable before launching Varo:

```bash
# Windows
set VARO_CONFIG_PATH=C:/Studio/Configs/varo.config.json

# macOS/Linux
export VARO_CONFIG_PATH=/Users/shared/varo/config.json
```

### ✅ Per-User
Place a config file at:

```bash
~/AppData/Local/varo/config.json
```
This file overrides any settings defined globally.

### ✅ (Coming Soon) CLI Override
In the future, you'll be able to run:

```bash
varo.exe --config /custom/path/config.json
```