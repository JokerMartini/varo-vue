# 📦 Varo Node JSON Specification

This document defines the structure and available properties of a `node` JSON file used by the Varo application.

Varo loads all node definitions from `.json` files located in the `VARO_PATH/nodes/` directory. Each JSON file defines an application/tool and how it is launched and categorized in the app.

---

## 📄 Basic Structure

Each node is represented as a single `.json` file with this shape:

```json
{
  "name": "Photoshop 2018",
  "groupId": "photoshop",
  "category": "Design",
  "icon": "icons/ps2018.svg",
  "visible": true,
  "defaultForGroup": false,
  "commands": [ ... ]
}
```

You may also include optional fields like `description`, `status`, `access`, and `env`.

**Note:**  
- `commands` is **required**.
- `access` is **optional**.

---

## 🧩 Node Properties

| Property          | Type      | Required | Description |
|-------------------|-----------|----------|-------------|
| `name`            | `string`  | ✅        | Display name of the node. |
| `groupId`         | `string`  | ⬜        | Optional group to associate multiple nodes (e.g., versions). |
| `category`        | `string`  | ✅        | Logical category used for filtering or sorting (e.g., "Design", "Tech"). |
| `icon`            | `string`  | ✅        | Path to an SVG/PNG icon. Can be **absolute** or **relative** to `VARO_PATH`. |
| `visible`         | `boolean` | ⬜        | Whether the node should be visible by default. Defaults to `true`. |
| `defaultForGroup` | `boolean` | ⬜        | If true, this node is considered the default selection within its group. |
| `description`     | `string`  | ⬜        | User-facing description shown in the app. |
| `status`          | `object`  | ⬜        | Optional status badge to show next to the node. |
| `commands`        | `array`   | ✅        | One or more launch commands for this node. |
| `access`          | `object`  | ⬜        | Platform and user access restrictions. |
| `env`             | `array`   | ⬜        | Optional environment variables to inject at runtime. |

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
  "color": "#ffffff"
}
```

| Property     | Type   | Description |
|--------------|--------|-------------|
| `name`       | string | Label shown in the UI (e.g., "Beta", "Live"). |
| `color`      | hex | rgb | name    | Text color (e.g., `#01B0D1`, `rgb(1,176,209)`, `blue`). |

---

## 🔐 Access Control (Optional)

Control platform compatibility and user-level visibility:

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

## ⚙️ Commands (Required)

Defines one or more launch commands executed when the node is launched.

```json
"commands": [
  {
    "path": "https://www.google.com/",
    "pathType": "url",
    "nonBlocking": true
  },
  {
    "path": "./${HOME}/scripts/simplePrint.py",
    "pathType": "rel"
  },
  {
    "path": "C:/Users/${USERNAME}/Desktop/helloWorld.py",
    "args": "0 kevin"
  }
]
```

| Property      | Type                 | Required | Description |
|---------------|----------------------|----------|-------------|
| `path`        | string                | ✅        | The command path to execute. Can contain `${VARS}`. |
| `pathType`        | `rel`/`abs`/`url`     | ⬜        | Defines how to interpret the path. If omitted, auto-inferred. |
| `args`        | string                | ⬜        | Optional arguments passed to the command. |
| `nonBlocking` | boolean               | ⬜        | If `true`, the command will not block the next one from running. Defaults to `false`. |

---

## 🌱 Environment Variables (Optional)

Defines environment variables that are set when launching commands.

```json
"env": [
  { "name": "ROOT", "value": "//Network/Path" },
  { "name": "LIB_PATH", "value": "${ROOT}/libs", "operation": "append" }
]
```

| Property | Type                       | Required | Description |
|----------|----------------------------|----------|-------------|
| `name`   | string                      | ✅        | Environment variable name. |
| `value`  | string                      | ✅        | Value to set, can contain other `${VARS}`. |
| `operation` | `set` / `append` / `prepend` | ⬜        | How to apply the variable. Defaults to `set`. |

---

## 📁 Icon Resolution

- **Relative paths** (e.g., `icons/app.svg`) are resolved relative to the root `VARO_PATH`.
- **Absolute paths** (e.g., `C:/Assets/app.svg`) are supported as-is.

---

## 🧪 Example: Full Node JSON

```json
{
  "name": "Photoshop 2018",
  "groupId": "photoshop",
  "category": "Design",
  "icon": "icons/ps2018.svg",
  "visible": true,
  "defaultForGroup": false,
  "description": "This is a sample application",
  "status": {
    "name": "Beta",
    "color": "#ffffff"
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
      "operation": "append"
    }
  ]
}
```

*future support...*

```yaml
name: Photoshop 2018
groupId: photoshop
category: Design
icon: icons/ps2018.svg
visible: true
defaultForGroup: false
description: This is a sample application
status:
  name: Beta
  color: "#ffffff"
access:
  platforms:
    - win
    - mac
  allow:
    - john
    - jane
  deny:
    - doe
commands:
  - path: https://www.google.com/
    type: url
    nonBlocking: true
  - path: ./${HOME}/scripts/simplePrint.py
    type: rel
  - path: C:/Users/${USERNAME}/Desktop/helloWorld.py
    args: "0 kevin"
env:
  - name: ROOT
    value: //Network/Path
  - name: LIB_PATH
    value: ${ROOT}/libs
    operation: append
```

---

## 🔄 Variable Substitution

You can use `${VARIABLE}` inside `icon`, `path`, `args`, and `env` `value` fields. These will be replaced at runtime based on:
- App defaults
- System/user environment
- User-selected environment profiles (e.g., `dev.env.json`)

---

## 📌 File Placement Guidelines

| File Type        | Folder              | Notes                            |
|------------------|---------------------|----------------------------------|
| JSON Node Files   | `VARO_PATH/nodes/`  | One file per node (recommended). |
| Icons            | `VARO_PATH/icons/`  | Use relative paths for `icon`.   |
| Env Profiles     | `VARO_PATH/envs/`   | Used for switching environment settings. |