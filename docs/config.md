# ⚙️ Global Configuration File (`config.json`)

The `config.json` file provides global defaults and override settings for the Varo application. It helps control behaviors such as default environment presets, fallback icons, and default node visibility without requiring every XML node to specify them explicitly.

---

## 📍 Location

Place this file directly in the root of your `VARO_PATH` directory:

```
VARO_PATH/
│
├── nodes/                     # All node json file definitions
│   ├── photoshop.json
│   ├── gitlab.json
│   └── jenkins.json
│
├── icons/                     # Icon assets used in nodes
│   ├── gitlab.svg
│   ├── jenkins.svg
│   └── photoshop.png
│
├── envs/                      # Environment presets (user-selectable)
│   ├── dev.env.json
│   └── prod.env.json
││
└── config.json                # Optional global config, fallback settings
```

---

## 🧩 Example `config.json`

```json
{
  "defaultEnv": "dev.env.json",
}
```

---

## 🔑 Supported Properties

| Property         | Type      | Description |
|------------------|-----------|-------------|
| `defaultEnv`     | `string`  | Specifies the default environment file to load on app start. Refers to a file in the `/envs` folder. |
| `iconFallback`   | `string`  | A path (relative or absolute) to a default icon to use when a node is missing its icon. |

---

## ✅ Best Practices

- Use `config.json` to simplify your XML definitions by offloading defaults here.
- Keep user-specific or session-specific data (like last-selected node) outside this file — consider saving those in a user preferences file instead.
- Always ensure paths (e.g., `iconFallback`) are valid and accessible within or outside the `VARO_PATH`.

---

This configuration file is optional but powerful — use it to reduce boilerplate and maintain global consistency across all your nodes.
