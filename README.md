# gmsv_dotenv

A Garry's Mod binary module that loads `.env` files and exposes their values to Lua, written in Rust.

---

## Installation

1. Download the correct binary for your server from the [Releases](../../releases) page:
   - `gmsv_dotenv_win64.dll` — Windows 64-bit (default since GMod x86-64 branch)
   - `gmsv_dotenv_win32.dll` — Windows 32-bit (legacy branch)
   - `gmsv_dotenv_linux64.so` — Linux 64-bit
   - `gmsv_dotenv_linux.so` — Linux 32-bit

2. Place the file in `garrysmod/lua/bin/`.

3. Load the module at the top of your server-side Lua:

```lua
require("dotenv")
```

---

## Quick Start

Create a `.env` file in your `garrysmod/` directory:

```env
DB_HOST=localhost
DB_PORT=3306
DB_NAME=my_database
DB_PASS="super secret"

DEBUG=true
MAX_PLAYERS=32

# This is a comment and will be ignored
EMPTY_VALUE=
```

Load and read it in Lua:

```lua
require("dotenv")

env.load(".env")

print(env.getString("DB_HOST"))      -- localhost
print(env.getNumber("DB_PORT"))      -- 3306
print(env.getBoolean("DEBUG"))       -- true
print(env.getInteger("MAX_PLAYERS")) -- 32
print(env.getString("EMPTY_VALUE"))  -- nil  (empty values are treated as nil)
```

---

## API

### `env.load(filePath)`

Reads and parses the given `.env` file. The path is relative to the `garrysmod/` directory.

```lua
env.load(".env")
env.load("cfg/server.env")
```

---

### `env.getString(key [, fallback])`

Returns the value as a string, or `fallback` if the key is not set.

```lua
env.getString("DB_HOST")               -- "localhost"
env.getString("MISSING_KEY")           -- nil
env.getString("MISSING_KEY", "default") -- "default"
```

---

### `env.getNumber(key [, fallback])`

Returns the value parsed as a number (`float`), or `fallback` if not set or not a valid number.

```lua
env.getNumber("DB_PORT")        -- 3306
env.getNumber("VERSION", 1.0)   -- 1.0  (if VERSION is not set)
```

---

### `env.getInteger(key [, fallback])`

Returns the value parsed as an integer (floored), or `fallback` if not set.

```lua
env.getInteger("MAX_PLAYERS")   -- 32
env.getInteger("TIMEOUT", 30)   -- 30  (if TIMEOUT is not set)
```

---

### `env.getBoolean(key [, fallback])`

Returns `true` or `false` for the strings `"true"` / `"false"` (case-insensitive), or `fallback` if the key is not set or the value is neither.

```lua
env.getBoolean("DEBUG")           -- true
env.getBoolean("DEBUG", false)    -- false  (if DEBUG is not set)
env.getBoolean("FEATURE_FLAGS")   -- nil    (if value is not "true"/"false")
```

---

### `env.getKeys()`

Returns a sequential table containing all loaded key names.

```lua
for _, key in ipairs(env.getKeys()) do
    print(key, env.getString(key))
end
```

---

### `env.parse(body)`

Parses a raw `.env` string without touching the filesystem. Returns two values: a table of key/value pairs and an error table (or `nil` if there were no errors).

```lua
local values, errors = env.parse([[
    HOST=localhost
    PORT=3306
    BAD_LINE
]])

print(values["HOST"])  -- localhost
print(errors[1])       -- "Invalid Key: BAD_LINE"
```

---

### `env(key [, fallback])` — Shorthand

The `env` table is callable and behaves identically to `env.getString`.

```lua
print(env("DB_HOST"))             -- localhost
print(env("MISSING", "fallback")) -- fallback
```

---

## .env File Format

```env
# Lines starting with # are comments

SIMPLE=Hello World
QUOTED_DOUBLE="Hello World"
QUOTED_SINGLE='Hello World'

# Inline comments are stripped
INLINE=Hello World   # this part is ignored

# Whitespace around keys and values is trimmed
  SPACED_KEY  =   spaced value

# Empty values are treated as nil and not stored
EMPTY=

# Escaped quotes inside a quoted value
COMPLEX="He said \"hello\", how are you?"

# A # inside quotes is NOT treated as a comment
HASHTAG="This #tag is kept"
```

**Rules:**
- Keys and values are trimmed of surrounding whitespace
- `"double"` and `'single'` quoted values have their quotes stripped
- `#` starts an inline comment unless it appears inside quotes
- `\` escapes the next character (prevents `"` or `'` from closing a quote)
- Empty values (`KEY=`) are treated as `nil` — the key will not be stored
- Duplicate keys: the **first** occurrence wins

---

## Building from Source

**Requirements:**
- Rust nightly (`rustup toolchain install nightly`)
- For cross-compilation on Linux: `mingw-w64`

**Windows (local):**

```powershell
rustup target add i686-pc-windows-msvc x86_64-pc-windows-msvc
.\build.ps1
```

**Linux / CI:**

```bash
apt-get install -y mingw-w64
rustup toolchain install nightly
rustup target add x86_64-pc-windows-gnu i686-pc-windows-gnu x86_64-unknown-linux-gnu i686-unknown-linux-gnu --toolchain nightly
cargo +nightly build --release --target x86_64-pc-windows-gnu
cargo +nightly build --release --target i686-pc-windows-gnu
cargo +nightly build --release --target x86_64-unknown-linux-gnu
cargo +nightly build --release --target i686-unknown-linux-gnu
```

Releases are built and published automatically via [GitHub Actions](.github/workflows/release.yml) on every version tag.

---

## License

MIT — see [LICENSE](LICENSE).
