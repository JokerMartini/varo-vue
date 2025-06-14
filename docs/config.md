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

---

## ✅ Best Practices

- Use `config.json` to simplify your XML definitions by offloading defaults here.
- Keep user-specific or session-specific data (like last-selected node) outside this file — consider saving those in a user preferences file instead.

---

This configuration file is optional but powerful — use it to reduce boilerplate and maintain global consistency across all your nodes.


# IDEA
- i'd like to provide users a way to easily deploy the Varo application and control the default settings of the application. My initial thought is to have support for a config.json file of sorts. Where this file could easily expand over time to contain settings to control Varo.
- I'm struggling to determine the best way to handle where this file should live? While at the same time giving both small and large teams an easy way to deploy and/or push updates to this config file.
- Below are some options and maybe we can find an elegant way to support all the options with cascading overrides of sorts. This way a studio could have global settings in the env path but then the users local config would override any settings within their config that are defined. Same would apply for the long term support for a config as an arg of the exe like CLI.
  1. Env variable that defines the location of a config file
  2. look for a config file in the user doc's folder
  3. Down the road allow a config file to be passed in as a arg to the executable 