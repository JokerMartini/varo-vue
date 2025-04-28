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
| `groupId`         | string       | Optional grouping ID for versions. |
| `visible`         | boolean      | Defaults to `true`. |
| `defaultForGroup` | boolean      | Defaults to `false`. |
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
    "pathType": "rel" | "abs" | "url",  // Optional
    "args": "string",          // Optional
    "nonBlocking": true        // Optional
  }
]
```

- `path`: Executable path or URL.
- `pathType`: How to interpret the path (auto-inferred if missing).
- `args`: Arguments to pass.
- `nonBlocking`: If true, don't wait for this command to finish.

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
  "groupId": "",
  "visible": true,
  "defaultForGroup": false,
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