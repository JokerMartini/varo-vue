# 📦 Varo Node JSON Specification

This document defines the structure and available properties of a `node` JSON used by the Varo application.

Varo loads all node definitions from `.json` files located in the `VARO_PATH/nodes/` directory. Each JSON file defines an application/tool and how it is launched and categorized in the app.

---

## 📄 Basic Structure

Each node is represented as a single `.json` file with this shape:

```json
{
  "uid": "ps-2018",
  "name": "Photoshop 2018",
  "groupId": "photoshop",
  "category": "Design",
  "icon": "icons/ps2018.svg",
  "visible": true,
  "defaultForGroup": false
}
```

You may also include optional fields like `description`, `status`, `commands`, `access`, and `env`.

---

## 🧩 Node Properties

| Property         | Type      | Required | Description |
|------------------|-----------|----------|-------------|
| `uid`            | `string`  | ✅       | A unique ID for the node. Used internally. |
| `name`           | `string`  | ✅       | Display name of the node. |
| `groupId`        | `string`  | ⬜       | Optional group to associate multiple nodes (e.g., versions). |
| `category`       | `string`  | ✅       | Logical category used for filtering or sorting (e.g., "Design", "Tech"). |
| `icon`           | `string`  | ✅       | Path to an SVG/PNG icon. Can be **absolute** or **relative** to `VARO_PATH`. |
| `visible`        | `boolean` | ⬜       | Whether the node should be visible by default. Defaults to `true`. |
| `defaultForGroup`| `boolean` | ⬜       | If true, this node is considered the default selection within its group. |

---

## 📝 Description (Optional)

You can add a user-facing description:

```json
"description": "This tool launches Photoshop with default settings."
```

---

## 🟨 Status (Optional)

Displays a badge in the UI to indicate the status of the node:

```json
"status": {
  "name": "Beta",
  "color": "#ffffff",
  "background": "#dd55dd"
}
```

| Property     | Type   | Description |
|--------------|--------|-------------|
| `name`       | string | Label shown in the UI (e.g., "Beta", "Live"). |
| `color`      | hex    | Text color (e.g., `#ffffff`). |
| `background` | hex    | Background color of the badge (e.g., `#ff00ff`). |

---

## 🔐 Access Control (`access`)

Control platform compatibility and user-level visibility.

```json
"access": {
  "platforms": ["win", "mac"],
  "allow": ["john", "jane"],
  "deny": ["doe"]
}
```

| Property    | Type       | Description |
|-------------|------------|-------------|
| `platforms` | string[]   | OS support, e.g., `["win", "mac", "linux"]`. If omitted, considered all-platform. |
| `allow`     | string[]   | Optional list of usernames allowed. |
| `deny`      | string[]   | Optional list of usernames denied. |

---

## ⚙️ Commands (`commands`)

Defines one or more launch commands executed when the node is launched.

```json
"commands": [
  {
    "path": "https://www.google.com/",
    "type": "url",
    "nonBlocking": true
  },
  {
    "path": "./${HOME}/scripts/simplePrint.py",
    "type": "rel"
  },
  {
    "path": "C:/Users/${USERNAME}/Desktop/helloWorld.py",
    "args": "0 kevin"
  }
]
```

| Property      | Type     | Required | Description |
|---------------|----------|----------|-------------|
| `path`        | string   | ✅       | The command path to execute. Can contain `${VARS}`. |
| `type`        | `rel`/`abs`/`url` | ⬜ | Defines how to interpret the path. If omitted, auto-inferred. |
| `args`        | string   | ⬜       | Optional arguments passed to the command. |
| `nonBlocking` | boolean  | ⬜       | If `true`, the command will not block the next one from running. Default is `false`. |

---

## 🌱 Environment Variables (`env`)

Defines environment variables that are set when launching commands.

```json
"env": [
  { "name": "ROOT", "value": "//Network/Path" },
  { "name": "LIB_PATH", "value": "${ROOT}/libs", "action": "append" }
]
```

| Property | Type     | Required | Description |
|----------|----------|----------|-------------|
| `name`   | string   | ✅       | Environment variable name. |
| `value`  | string   | ✅       | Value to set, can contain other `${VARS}`. |
| `action` | `set` / `append` / `prepend` | ⬜ | How to apply the variable. Defaults to `set`. |

---

## 📁 Icon Resolution

- **Relative paths** (e.g., `icons/app.svg`) are resolved relative to the root `VARO_PATH`.
- **Absolute paths** (e.g., `C:/Assets/app.svg`) are supported as-is.

---

## 🧪 Example: Full Node JSON

```json
{
  "uid": "ps-2018",
  "name": "Photoshop 2018",
  "groupId": "photoshop",
  "category": "Design",
  "icon": "icons/ps2018.svg",
  "visible": true,
  "defaultForGroup": false,
  "description": "This is a sample application",
  "status": {
    "name": "Beta",
    "color": "#ffffff",
    "background": "#dd55dd"
  },
  "access": {
    "platforms": ["win", "mac"],
    "allow": ["john", "jane"],
    "deny": ["doe"]
  },
  "commands": [
    {
      "path": "https://www.google.com/",
      "type": "url",
      "nonBlocking": true
    },
    {
      "path": "./${HOME}/scripts/simplePrint.py",
      "type": "rel"
    },
    {
      "path": "C:/Users/${USERNAME}/Desktop/helloWorld.py",
      "args": "0 kevin"
    }
  ],
  "env": [
    {
      "name": "ROOT",
      "value": "//Network/Path"
    },
    {
      "name": "LIB_PATH",
      "value": "${ROOT}/libs",
      "action": "append"
    }
  ]
}
```

---

## 🔄 Variable Substitution

You can use `${VARIABLE}` inside paths, args, and environment values. These will be replaced at runtime using:
- App defaults
- System/user environment
- User-selected environment profiles (e.g., `dev.env.json`)

---

## 📌 File Placement Guidelines

| File Type     | Folder              | Notes                            |
|---------------|---------------------|----------------------------------|
| JSON Node Files | `VARO_PATH/nodes/`  | One file per node (recommended) |
| Icons         | `VARO_PATH/icons/`  | Use relative paths in `icon`    |
| Env Profiles  | `VARO_PATH/envs/`   | Used to switch environments     |

---

## Code Snippets

```ts
// icon parsing
function resolveIconPath(iconAttr: string): string {
  if (path.isAbsolute(iconAttr)) return iconAttr;
  return path.resolve(VARO_PATH, iconAttr);
}
```