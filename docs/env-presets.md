# Environment Preset Files

Environment preset files allow you to define collections of environment variables that can be loaded into Varo applications. These `.json` files provide a structured way to manage different environment configurations for development, testing, production, and other scenarios.

## 📁 File Location and Discovery

Environment preset files are discovered from directories specified in your `config.json` file:

```json
{
  "env_presets": {
    "directories": [
      "C:/Studio/VaroPresets",
      "D:/Teams/Env_presets"
    ],
    "default_id": "studio-default"
  }
}
```

Varo will scan these directories for `.json` files and treat them as environment presets.

## 📋 File Structure

Each environment preset file must be a valid JSON file with the following structure:

### Required Fields

| Field         | Type     | Description                                                    |
| ------------- | -------- | -------------------------------------------------------------- |
| `name`        | `string` | Human-readable name displayed in the Varo interface           |
| `id`          | `string` | Unique identifier used to reference this preset               |
| `description` | `string` | Brief description of what this preset is intended for         |
| `env`         | `array`  | Array of environment variable definitions (see below)         |

### Environment Variable Definition

Each item in the `env` array defines how an environment variable should be set:

| Field       | Type     | Required | Description                                           |
| ----------- | -------- | -------- | ----------------------------------------------------- |
| `name`      | `string` | ✅       | The name of the environment variable                  |
| `value`     | `string` | ✅       | The value to assign (supports variable expansion)     |
| `operation` | `string` | ❌       | How to apply the value (default: `"set"`)             |

## 🔧 Operations

The `operation` field determines how the environment variable value is applied:

| Operation | Description                                                     |
| --------- | --------------------------------------------------------------- |
| `set`     | **Default.** Replace any existing value with the new value     |
| `append`  | Add the value to the end of the existing environment variable  |
| `prepend` | Add the value to the beginning of the existing variable        |

> **Note:** When using `append` or `prepend`, values are typically separated by the appropriate path separator for your operating system.

## 🔀 Variable Expansion

Environment variable values support expansion of other environment variables using the `${VARIABLE_NAME}` syntax:

- `${HOME}` - Expands to the user's home directory
- `${GITHUB_ROOT}` - Expands to a previously defined environment variable
- `${PATH}` - References the current PATH variable

## 📝 Complete Example

```json
{
  "name": "Production",
  "id": "master",
  "description": "Production environment variables for live pipeline",
  "env": [
    {
      "name": "AAA_LIVE",
      "value": "//Dev/Network/Path"
    },
    {
      "name": "AAA_LIVE",
      "value": "${HOME}/dev/libs",
      "operation": "append"
    },
    {
      "name": "VARO_PATH",
      "value": "${GITHUB_ROOT}/varo-vue/test-data-2",
      "operation": "set"
    },
    {
      "name": "NODE_ENV",
      "value": "production"
    },
    {
      "name": "PATH",
      "value": "C:/CustomTools/bin",
      "operation": "prepend"
    }
  ]
}
```

## 🎯 Best Practices

### Naming Conventions
- Use descriptive, lowercase IDs (e.g., `dev-local`, `prod-main`, `test-ci`)
- Choose clear, descriptive names that indicate the preset's purpose
- Include environment type in the description

### Organization
- Group related environment variables together
- Use consistent variable naming across presets
- Document complex variable expansions in the description

### Variable Management
- Use `set` for most simple variable assignments
- Use `append`/`prepend` carefully with path-like variables
- Test variable expansion to ensure correct resolution order

## 🔄 Loading Presets

Presets can be loaded in Varo through:

1. **Default loading** - The preset specified by `default_id` in your config
2. **Manual selection** - Choosing a preset from the Varo interface
3. **Application launch** - Selecting a preset when launching an application

## ⚠️ Common Issues

- **Duplicate variable names**: Multiple entries with the same `name` will be processed in order
- **Circular references**: Avoid `${VAR}` expansions that reference themselves
- **Path separators**: Use forward slashes `/` in paths when possible for cross-platform compatibility
- **JSON syntax**: Ensure proper JSON formatting with escaped backslashes in Windows paths

## 🔍 Validation

Varo will validate preset files and report errors for:
- Invalid JSON syntax
- Missing required fields (`name`, `id`, `description`, `env`)
- Invalid `operation` values
- Malformed variable expansion syntax