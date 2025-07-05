Perfect! Here's a **compact cheatsheet version** for you — designed for quick reference:

---

# 📄 Varo Node JSON Cheatsheet

### Required Fields
| Property          | Type       | Description |
|-------------------|------------|-------------|
| `name`            | string     | Display name. |
| `category`        | string     | Node category (e.g., Design, Tech). |
| `icon`            | string     | Path to SVG/PNG icon (relative or absolute). |
| `commands`        | array      | List of launch commands. |

---

### Optional Fields
| Property          | Type         | Description |
|-------------------|--------------|-------------|
| `group_id`         | string       | Optional grouping ID for versions. |
| `visible`         | boolean      | Defaults to `true`. |
| `default_for_group` | boolean      | Defaults to `false`. |
| `description`     | string       | App description. |
| `status`          | object       | Status badge (`name`, `color`). |
| `access`          | object       | Platform/user restrictions. |
| `env`             | array        | Environment variables to set on launch. |

---

# ⚙️ Commands Structure (Required)

```json
"commands": [
  {
    "path": "string",         // Required
    "path_type": "rel" | "abs" | "url",  // Optional
    "args": "string",          // Optional
    "wait": false        // Optional
  }
]
```

- `path`: Executable path or URL.
- `path_type`: How to interpret the path (auto-inferred if missing).
- `args`: Arguments to pass.
- `wait`: If true, wait for this command to finish before continuing. Defaults to `true`.

---

# 🔐 Access Structure (Optional)

```json
"access": {
  "platforms": ["win", "mac", "linux"],
  "allow": ["username1", "username2"],
  "deny": ["username3"]
}
```

- `platforms`: OS restriction (if missing, supports all OS).
- `allow`: Only listed users can see/run.
- `deny`: Listed users are blocked.

---

# 🌱 Env Structure (Optional)

```json
"env": [
  { "name": "VAR", "value": "something" },
  { "name": "OTHER_VAR", "value": "path", "operation": "append" }
]
```

- `operation`: `set` (default) | `append` | `prepend`

---

# 🧩 Quick JSON Skeleton

```json
{
  "name": "",
  "category": "",
  "icon": "",
  "commands": [],
  "group_id": "",
  "visible": true,
  "default_for_group": false,
  "description": "",
  "status": { "name": "", "color": "" },
  "access": { "platforms": [], "allow": [], "deny": [] },
  "env": []
}
```

---

# 📌 Notes

- `commands` is **mandatory** — no node can exist without at least one command.
- `access` is **optional** — omit entirely if not needed.
- `${VARIABLE}` substitution works inside `icon`, `path`, `args`, and `env` values.